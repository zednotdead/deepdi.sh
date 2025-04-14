pub mod kafka;
pub mod stub;

use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::entities::ingredient::Ingredient;

#[async_trait]
pub trait MessageService: Send + Sync + 'static {
    async fn ingredient_added(&self, ing: &Ingredient) -> eyre::Result<()>;
    async fn ingredient_deleted(&self, ing: &Ingredient) -> eyre::Result<()>;
    async fn ingredient_updated(&self, old_ing: &Ingredient, new_ing: &Ingredient) -> eyre::Result<()>;
}

pub type MessageServiceImpl = Arc<Box<dyn MessageService>>;
