use std::sync::Arc;

use async_trait::async_trait;
use futures::future::{join_all, try_join_all};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::ingredient::{Ingredient, IngredientModel};
use crate::domain::entities::recipe::{
    IngredientUnit, IngredientWithAmount, IngredientWithAmountModel, Recipe, RecipeChangeset,
};

use super::errors::{
    AddIngredientIntoRecipeError, DeleteIngredientFromRecipeError, DeleteRecipeError,
    GetAllRecipesError, UpdateIngredientInRecipeError, UpdateRecipeError,
};
use super::RecipeRepositoryService;
use super::{
    errors::{GetRecipeByIdError, InsertRecipeError},
    RecipeRepository,
};

pub struct PostgresRecipeRepository(pub PgPool);

async fn insert_ingredient(
    pool: &PgPool,
    id: Uuid,
    ingredient: &IngredientWithAmount,
) -> Result<(), AddIngredientIntoRecipeError> {
    let amount = serde_json::to_value(ingredient.amount.clone())
        .map_err(|e| AddIngredientIntoRecipeError::UnknownError(e.into()))?;

    sqlx::query_file!(
        "queries/recipes/insert_ingredient.sql",
        id,
        ingredient.ingredient.id,
        amount,
        ingredient.notes,
        ingredient.optional
    )
    .execute(pool)
    .await
    .map_err(AddIngredientIntoRecipeError::from)?;

    Ok(())
}

async fn update_timestamps_in_recipe(pool: &PgPool, id: Uuid) {
    let _ = sqlx::query_file!("queries/recipes/update_recipe_timestamps.sql", id)
        .execute(pool)
        .await;
}

#[async_trait]
impl RecipeRepository for PostgresRecipeRepository {
    #[tracing::instrument(
        "[RECIPE REPOSITORY] [POSTGRES] Insert new recipe",
        skip(self)
    )]
    async fn insert(&self, input: Recipe) -> Result<(), InsertRecipeError> {
        let time = serde_json::to_value(&input.time)
            .map_err(|e| InsertRecipeError::UnknownError(e.into()))?;

        let servings = serde_json::to_value(&input.servings)
            .map_err(|e| InsertRecipeError::UnknownError(e.into()))?;

        let tx = self.0.begin().await.map_err(InsertRecipeError::from)?;

        let result = sqlx::query_file!(
            "queries/recipes/insert_recipe.sql",
            input.id,
            input.name,
            input.description,
            &input.steps.as_ref(),
            time,
            servings,
            serde_json::json!({})
        )
        .fetch_one(&self.0)
        .await
        .map_err(InsertRecipeError::from)?;

        join_all(
            input
                .ingredients
                .iter()
                .map(|i| insert_ingredient(&self.0, result.id, i)),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<()>, AddIngredientIntoRecipeError>>()?;

        tx.commit().await.map_err(InsertRecipeError::from)?;

        Ok(())
    }

    #[tracing::instrument(
        "[RECIPE REPOSITORY] [POSTGRES] Get recipe by ID",
        skip(self)
    )]
    async fn get_by_id(&self, id: &Uuid) -> Result<Recipe, GetRecipeByIdError> {
        let result = sqlx::query_file!("queries/recipes/get_recipe.sql", id)
            .fetch_one(&self.0)
            .await
            .map_err(|e| GetRecipeByIdError::with_id(id, e))?;

        let result_ingredients = sqlx::query_file_as!(
            IngredientWithAmountModel,
            "queries/recipes/get_ingredients_for_recipe.sql",
            id
        )
        .fetch_all(&self.0)
        .await?;

        let ingredients = result_ingredients
            .iter()
            .map(IngredientWithAmount::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(GetRecipeByIdError::from)?;

        let time = serde_json::from_value(result.time)?;

        let servings = serde_json::from_value(result.servings)?;

        let recipe = Recipe {
            id: result.id,
            name: result.name,
            description: result.description,
            steps: result.steps.try_into()?,
            time,
            servings,
            ingredients: ingredients.try_into()?,
            created_at: result.created_at,
            updated_at: result.updated_at,
        };

        Ok(recipe)
    }

    #[tracing::instrument(
        "[RECIPE REPOSITORY] [POSTGRES] Get all recipes",
        skip(self)
    )]
    async fn get_all(&self) -> Result<Vec<Recipe>, GetAllRecipesError> {
        let result = sqlx::query_file!("queries/recipes/get_all_recipes.sql")
            .fetch_all(&self.0)
            .await
            .map_err(|e| GetAllRecipesError::UnknownError(e.into()))?;

        let recipes_ft: Vec<_> = result
            .into_par_iter()
            .map(async |recipe| {
                let result_ingredients = sqlx::query_file_as!(
                    IngredientWithAmountModel,
                    "queries/recipes/get_ingredients_for_recipe.sql",
                    recipe.id
                )
                .fetch_all(&self.0)
                .await?;

                let ingredients = result_ingredients
                    .iter()
                    .map(IngredientWithAmount::try_from)
                    .collect::<Result<Vec<_>, _>>()?;

                let time = serde_json::from_value(recipe.time)?;

                let servings = serde_json::from_value(recipe.servings)?;

                let recipe = Recipe {
                    id: recipe.id,
                    name: recipe.name,
                    description: recipe.description,
                    steps: recipe.steps.try_into()?,
                    time,
                    servings,
                    ingredients: ingredients.try_into()?,
                    created_at: recipe.created_at,
                    updated_at: recipe.updated_at,
                };

                Ok::<Recipe, GetAllRecipesError>(recipe)
            })
            .collect();

        let recipes = try_join_all(recipes_ft).await?;

        Ok(recipes)
    }

    #[tracing::instrument(
        "[RECIPE REPOSITORY] [POSTGRES] Delete recipe",
        skip(self)
    )]
    async fn delete(&self, recipe: &Recipe) -> Result<(), DeleteRecipeError> {
        let tx = self.0.begin().await?;

        sqlx::query_file!(
            "queries/recipes/delete_ingredients_for_recipe.sql",
            recipe.id
        )
        .execute(&self.0)
        .await?;

        sqlx::query_file!("queries/recipes/delete_recipe.sql", recipe.id)
            .execute(&self.0)
            .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn update(
        &self,
        recipe: &Recipe,
        changeset: RecipeChangeset,
    ) -> Result<(), UpdateRecipeError> {
        let id = &recipe.id;
        let tx = self.0.begin().await?;
        let mut updated = false;

        if let Some(value) = changeset.name {
            if value != recipe.name {
                sqlx::query!(
                    r#"
                    UPDATE recipes
                    SET name = $2
                    WHERE id = $1
                    "#,
                    id,
                    value
                )
                .execute(&self.0)
                .await?;
                updated = true;
            }
        };

        if let Some(value) = changeset.description {
            if value != recipe.description {
                sqlx::query!(
                    r#"
                    UPDATE recipes
                    SET description = $2
                    WHERE id = $1
                    "#,
                    id,
                    value
                )
                .execute(&self.0)
                .await?;
                updated = true;
            }
        };

        if let Some(value) = changeset.servings {
            if value != recipe.servings {
                let value = serde_json::to_value(value)
                    .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;

                sqlx::query!(
                    r#"
                    UPDATE recipes
                    SET servings = $2
                    WHERE id = $1
                    "#,
                    id,
                    value
                )
                .execute(&self.0)
                .await?;
                updated = true;
            }
        }

        if let Some(value) = changeset.time {
            if value != recipe.time {
                let value = serde_json::to_value(value)
                    .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;

                sqlx::query!(
                    r#"
                    UPDATE recipes
                    SET time = $2
                    WHERE id = $1
                    "#,
                    id,
                    value
                )
                .execute(&self.0)
                .await?;
                updated = true;
            }
        }

        if let Some(value) = changeset.steps {
            if value != recipe.steps {
                let value = value.as_ref();

                sqlx::query!(
                    r#"
                    UPDATE recipes
                    SET steps = $2
                    WHERE id = $1
                    "#,
                    id,
                    value
                )
                .execute(&self.0)
                .await?;
                updated = true;
            }
        }

        if updated {
            update_timestamps_in_recipe(&self.0, *id).await;
        }

        tx.commit()
            .await
            .map_err(|e| UpdateRecipeError::UnknownError(e.into()))?;
        Ok(())
    }

    async fn add_ingredient(
        &self,
        recipe: &Recipe,
        ingredient: IngredientWithAmount,
    ) -> Result<(), AddIngredientIntoRecipeError> {
        insert_ingredient(&self.0, recipe.id, &ingredient).await?;
        update_timestamps_in_recipe(&self.0, recipe.id).await;

        Ok(())
    }

    async fn delete_ingredient(
        &self,
        recipe: &Recipe,
        ingredient: &IngredientWithAmount,
    ) -> Result<(), DeleteIngredientFromRecipeError> {
        sqlx::query_file!(
            "queries/recipes/delete_ingredient_from_recipe_by_id.sql",
            recipe.id,
            ingredient.ingredient.id
        )
        .execute(&self.0)
        .await?;

        update_timestamps_in_recipe(&self.0, recipe.id).await;

        Ok(())
    }

    async fn update_ingredient_amount(
        &self,
        recipe: &Recipe,
        ingredient: &IngredientWithAmount,
        new_amount: &IngredientUnit,
    ) -> Result<(), UpdateIngredientInRecipeError> {
        let tx = self.0.begin().await?;

        let amount = serde_json::to_value(new_amount)?;

        sqlx::query_file!(
            "queries/recipes/update_ingredient_in_recipe.sql",
            recipe.id,
            ingredient.ingredient.id,
            amount
        )
        .execute(&self.0)
        .await?;

        update_timestamps_in_recipe(&self.0, recipe.id).await;

        tx.commit().await?;

        Ok(())
    }

    async fn recipes_containing_ingredient_exist(
        &self,
        ingredient: Ingredient,
    ) -> eyre::Result<bool> {
        let recipes_using_ingredient = sqlx::query_file!(
            "queries/recipes/get_recipes_using_ingredient.sql",
            ingredient.id
        )
        .fetch_optional(&self.0)
        .await?;

        Ok(recipes_using_ingredient.is_some())
    }
}

impl PostgresRecipeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub fn service(self) -> RecipeRepositoryService {
        Arc::new(Box::new(self))
    }
}
