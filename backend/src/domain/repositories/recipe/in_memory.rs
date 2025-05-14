use async_trait::async_trait;
use chrono::Utc;
use eyre::eyre;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use uuid::Uuid;

use crate::domain::{
    entities::{
        ingredient::Ingredient,
        recipe::{IngredientUnit, IngredientWithAmount, Recipe, RecipeChangeset},
    },
    repositories::recipe::errors::InsertRecipeError,
};

use super::{
    errors::{
        AddIngredientIntoRecipeError, DeleteIngredientFromRecipeError, DeleteRecipeError, GetAllRecipesError, GetRecipeByIdError, UpdateIngredientInRecipeError, UpdateRecipeError
    },
    RecipeRepository, RecipeRepositoryService,
};

pub struct InMemoryRecipeRepository(pub Mutex<HashMap<uuid::Uuid, Recipe>>);

#[async_trait]
impl RecipeRepository for InMemoryRecipeRepository {
    async fn insert(&self, input: Recipe) -> Result<(), InsertRecipeError> {
        let mut lock = self.0.lock()?;

        if lock.iter().any(|(id, _)| id == &input.id) {
            tracing::error!("The recipe with ID {} already exists.", input.id);
            return Err(InsertRecipeError::Conflict("recipe id".to_string()));
        };

        lock.insert(input.id, input.clone());

        Ok(())
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Recipe, GetRecipeByIdError> {
        let lock = self.0.lock()?;

        let result = lock
            .get(id)
            .cloned()
            .ok_or_else(|| GetRecipeByIdError::NotFound(*id))?;

        Ok(result)
    }

    async fn get_all(&self) -> Result<Vec<Recipe>, GetAllRecipesError> {
        let lock = self.0.lock()?;

        let result = lock
            .values()
            .cloned()
            .collect::<Vec<_>>();

        Ok(result)
    }

    async fn delete(&self, recipe: &Recipe) -> Result<(), DeleteRecipeError> {
        let id = &recipe.id;
        let mut lock = self.0.lock()?;

        lock.remove(id)
            .ok_or(DeleteRecipeError::UnknownError(eyre!(
                "The recipe could not be found somehow"
            )))?;

        Ok(())
    }

    async fn update(
        &self,
        recipe: &Recipe,
        changeset: RecipeChangeset,
    ) -> Result<(), UpdateRecipeError> {
        let mut lock = self.0.lock()?;
        let recipe = lock
            .get_mut(&recipe.id)
            .ok_or(UpdateRecipeError::UnknownError(eyre!(
                "The recipe could not be found somehow"
            )))?;

        let mut updated = false;

        if let Some(v) = changeset.name {
            recipe.name = v;
            updated = true;
        };

        if let Some(v) = changeset.time {
            recipe.time = v;
            updated = true;
        };

        if let Some(v) = changeset.steps {
            recipe.steps = v;
            updated = true;
        };

        if let Some(v) = changeset.servings {
            recipe.servings = v;
            updated = true;
        };

        if let Some(v) = changeset.description {
            recipe.description = v;
            updated = true;
        };

        if updated {
            recipe.updated_at = Utc::now();
        };

        Ok(())
    }

    async fn add_ingredient(
        &self,
        recipe: &Recipe,
        ingredient: IngredientWithAmount,
    ) -> Result<(), AddIngredientIntoRecipeError> {
        let mut lock = self.0.lock()?;
        let recipe = lock
            .get_mut(&recipe.id)
            .ok_or(AddIngredientIntoRecipeError::UnknownError(eyre!(
                "Recipe is not in the repo"
            )))?;

        recipe.ingredients.push(ingredient);
        recipe.updated_at = Utc::now();

        Ok(())
    }

    async fn delete_ingredient(
        &self,
        recipe: &Recipe,
        ingredient: &IngredientWithAmount,
    ) -> Result<(), DeleteIngredientFromRecipeError> {
        let mut lock = self.0.lock()?;
        let recipe =
            lock.get_mut(&recipe.id)
                .ok_or(DeleteIngredientFromRecipeError::UnknownError(eyre!(
                    "Recipe is not in the repo somehow"
                )))?;

        let new_ingredients: Vec<_> = recipe
            .ingredients
            .iter()
            .filter(|&ingam| ingam.ingredient.id != ingredient.ingredient.id)
            .cloned()
            .collect();

        recipe.ingredients = new_ingredients
            .try_into()
            .map_err(DeleteIngredientFromRecipeError::ValidationError)?;
        recipe.updated_at = Utc::now();

        Ok(())
    }

    async fn update_ingredient_amount(
        &self,
        recipe: &Recipe,
        ingredient: &IngredientWithAmount,
        new_amount: &IngredientUnit,
    ) -> Result<(), UpdateIngredientInRecipeError> {
        let mut lock = self.0.lock()?;
        let recipe =
            lock.get_mut(&recipe.id)
                .ok_or(UpdateIngredientInRecipeError::UnknownError(eyre!(
                    "Recipe is not in the repo somehow"
                )))?;

        let ingredient = recipe
            .ingredients
            .iter_mut()
            .find(|i| i.ingredient.id == ingredient.ingredient.id)
            .ok_or(UpdateIngredientInRecipeError::UnknownError(eyre!(
                "Ingredient somehow is not in the recipe, but the command made sure there was."
            )))?;

        ingredient.amount = new_amount.clone();
        recipe.updated_at = Utc::now();

        Ok(())
    }

    async fn recipes_containing_ingredient_exist(
        &self,
        ingredient: Ingredient,
    ) -> eyre::Result<bool> {
        let lock = self.0.lock().map_err(|_| eyre!("Poison issue"))?;
        let some_recipe_with_ingredient = lock.par_iter().find_any(|(_id, r)| {
            r.ingredients
                .iter()
                .any(|i| i.ingredient.id == ingredient.id)
        });

        Ok(some_recipe_with_ingredient.is_some())
    }
}

impl Default for InMemoryRecipeRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryRecipeRepository {
    pub fn new() -> Self {
        Self(Mutex::new(HashMap::new()))
    }

    pub fn service(self) -> RecipeRepositoryService {
        Arc::new(Box::new(self))
    }
}

impl From<HashMap<uuid::Uuid, Recipe>> for InMemoryRecipeRepository {
    fn from(value: HashMap<uuid::Uuid, Recipe>) -> Self {
        Self(Mutex::new(value))
    }
}
