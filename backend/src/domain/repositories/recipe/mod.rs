pub mod errors;
pub mod in_memory;
pub mod postgres;

use crate::domain::entities::{
    ingredient::Ingredient,
    recipe::{IngredientUnit, IngredientWithAmount, Recipe, RecipeChangeset},
};
use async_trait::async_trait;
use errors::{AddIngredientIntoRecipeError, GetAllRecipesError};
use std::sync::Arc;
use uuid::Uuid;

use self::errors::{
    DeleteIngredientFromRecipeError, DeleteRecipeError, GetRecipeByIdError, InsertRecipeError,
    UpdateIngredientInRecipeError, UpdateRecipeError,
};

#[async_trait]
pub trait RecipeRepository: Send + Sync + 'static {
    // TODO: Include user information
    async fn insert(&self, input: Recipe) -> Result<(), InsertRecipeError>;

    async fn get_by_id(&self, id: &Uuid) -> Result<Recipe, GetRecipeByIdError>;
    async fn get_all(&self) -> Result<Vec<Recipe>, GetAllRecipesError>;

    async fn delete(&self, recipe: &Recipe) -> Result<(), DeleteRecipeError>;

    async fn update(
        &self,
        recipe: &Recipe,
        changeset: RecipeChangeset,
    ) -> Result<(), UpdateRecipeError>;

    async fn add_ingredient(
        &self,
        recipe: &Recipe,
        ingredient: IngredientWithAmount,
    ) -> Result<(), AddIngredientIntoRecipeError>;

    async fn delete_ingredient(
        &self,
        recipe: &Recipe,
        ingredient: &IngredientWithAmount,
    ) -> Result<(), DeleteIngredientFromRecipeError>;

    async fn update_ingredient_amount(
        &self,
        recipe: &Recipe,
        ingredient: &IngredientWithAmount,
        new_amount: &IngredientUnit,
    ) -> Result<(), UpdateIngredientInRecipeError>;

    async fn recipes_containing_ingredient_exist(
        &self,
        ingredient: Ingredient,
    ) -> eyre::Result<bool>;
}

pub type RecipeRepositoryService = Arc<Box<dyn RecipeRepository>>;
