use std::time::Duration;

use axum::async_trait;
use common::IngredientDTO;
use eyre::eyre;
use rdkafka::producer::{FutureProducer, FutureRecord};

use crate::domain::entities::ingredient::Ingredient;

use super::MessageService;

pub struct KafkaMessageService {
    producer: FutureProducer,
}

impl KafkaMessageService {
    pub fn new(url: &str) -> eyre::Result<Self> {
        let kafka_client: FutureProducer = rdkafka::ClientConfig::new()
            .set("bootstrap.servers", url)
            .set("message.timeout.ms", "50000")
            .create()?;

        Ok(Self {
            producer: kafka_client,
        })
    }
}

#[async_trait]
impl MessageService for KafkaMessageService {
    async fn ingredient_added(&self, ing: &Ingredient) -> eyre::Result<()> {
        let ing: IngredientDTO = ing.into();
        let _res = self
            .producer
            .send(
                FutureRecord::to("deepdish.ingredient.added")
                    .key(&serde_json::json!({ "id": &ing.id.to_string() }).to_string())
                    .payload(&serde_json::to_string(&ing)?),
                Duration::from_secs(0),
            )
            .await
            .map_err(|e| eyre!("Could not send message to Kafka, {:#?}", e))?;

        Ok(())
    }
    async fn ingredient_deleted(&self, ing: &Ingredient) -> eyre::Result<()> {
        let ing: IngredientDTO = ing.into();
        let _res = self
            .producer
            .send(
                FutureRecord::to("deepdish.ingredient.deleted")
                    .key(&serde_json::json!({ "id": &ing.id.to_string() }).to_string())
                    .payload(&serde_json::json!(ing).to_string()),
                Duration::from_secs(0),
            )
            .await
            .map_err(|e| eyre!("Could not send message to Kafka, {:#?}", e))?;

        Ok(())
    }
    async fn ingredient_updated(
        &self,
        old_ing: &Ingredient,
        new_ing: &Ingredient,
    ) -> eyre::Result<()> {
        let old_ing: IngredientDTO = old_ing.into();
        let new_ing: IngredientDTO = new_ing.into();

        let _res = self
            .producer
            .send(
                FutureRecord::to("deepdish.ingredient.deleted")
                    .key(&serde_json::json!({ "id": &old_ing.id.to_string() }).to_string())
                    .payload(&serde_json::json!({ "old": &old_ing, "new": &new_ing }).to_string()),
                Duration::from_secs(0),
            )
            .await
            .map_err(|e| eyre!("Could not send message to Kafka, {:#?}", e))?;

        Ok(())
    }
}
