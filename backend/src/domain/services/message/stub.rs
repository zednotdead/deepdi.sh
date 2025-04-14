use async_trait::async_trait;

use crate::domain::entities::ingredient::Ingredient;

use super::MessageService;

pub struct StubMessageService;

#[async_trait]
impl MessageService for StubMessageService {
    async fn ingredient_added(&self, _ing: &Ingredient) -> eyre::Result<()> {
        Ok(())
    }
    async fn ingredient_deleted(&self, _ing: &Ingredient) -> eyre::Result<()> {
        Ok(())
    }
    async fn ingredient_updated(&self, _old_ing: &Ingredient, _new_ing: &Ingredient) -> eyre::Result<()> {
        Ok(())
    }
}
