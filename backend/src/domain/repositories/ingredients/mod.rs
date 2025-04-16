pub mod errors;
pub mod in_memory;
pub mod postgres;

use std::sync::Arc;

use async_trait::async_trait;
use mockall::mock;
use uuid::Uuid;

use crate::domain::entities::ingredient::{Ingredient, IngredientChangeset};

use self::errors::{
    DeleteIngredientError, GetAllIngredientsError, GetIngredientByIdError, InsertIngredientError,
    UpdateIngredientError,
};

#[async_trait]
pub trait IngredientRepository: Send + Sync + 'static {
    async fn insert(&self, ingredient: Ingredient) -> Result<Ingredient, InsertIngredientError>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Ingredient, GetIngredientByIdError>;
    async fn get_all_by_id(&self, ids: &[Uuid]) -> Result<Vec<Ingredient>, GetAllIngredientsError>;
    async fn get_all(&self) -> Result<Vec<Ingredient>, GetAllIngredientsError>;
    async fn update(
        &self,
        ingredient: &Ingredient,
        changeset: IngredientChangeset,
    ) -> Result<(), UpdateIngredientError>;
    async fn delete(&self, ingredient: Ingredient) -> Result<(), DeleteIngredientError>;
}
mock! {
    pub IngredientRepository {}

    #[async_trait]
    impl IngredientRepository for IngredientRepository {
        async fn insert(&self, ingredient: Ingredient) -> Result<Ingredient, InsertIngredientError>;
        async fn get_by_id(&self, id: &Uuid) -> Result<Ingredient, GetIngredientByIdError>;
        async fn get_all_by_id(&self, ids: &[Uuid]) -> Result<Vec<Ingredient>, GetAllIngredientsError>;
        async fn get_all(&self) -> Result<Vec<Ingredient>, GetAllIngredientsError>;
        async fn update(
            &self,
            ingredient: &Ingredient,
            changeset: IngredientChangeset,
        ) -> Result<(), UpdateIngredientError>;
        async fn delete(&self, ingredient: Ingredient) -> Result<(), DeleteIngredientError>;
    }
}

pub type IngredientRepositoryService = Arc<Box<dyn IngredientRepository>>;
