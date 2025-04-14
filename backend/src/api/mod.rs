mod errors;
mod extract;
mod routes;

use std::sync::Arc;

use crate::domain::{
    repositories::{
        ingredients::{
            in_memory::InMemoryIngredientRepository, postgres::PostgresIngredientRepository,
            IngredientRepository, IngredientRepositoryService,
        },
        recipe::{
            in_memory::InMemoryRecipeRepository, postgres::PostgresRecipeRepository,
            RecipeRepository, RecipeRepositoryService,
        },
    },
    services::message::{
        kafka::KafkaMessageService, stub::StubMessageService, MessageService, MessageServiceImpl,
    },
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use color_eyre::Result;
use sqlx::PgPool;

use self::routes::{ingredients::*, recipes::*};

pub struct App {
    router: Router,
}

#[derive(Clone)]
pub struct AppState {
    pub ingredient_repository: IngredientRepositoryService,
    pub recipe_repository: RecipeRepositoryService,
    pub message_service: MessageServiceImpl,
}

impl App {
    fn get_router() -> Router<AppState> {
        Router::new()
            .route("/ingredient/:id", put(update_ingredient_route))
            .route("/ingredient/:id", get(get_ingredient_by_id_route))
            .route("/ingredient/:id", delete(delete_ingredient_route))
            .route("/ingredient", post(create_ingredient_route))
            .route("/ingredient", get(get_all_ingredients_route))
            .route("/recipe", post(create_recipe_route))
            .route("/recipe/:id", get(get_recipe_by_id_route))
            .route("/recipe/:id", delete(delete_recipe_route))
            .route("/recipe/:id", put(update_recipe_route))
            .route(
                "/recipe/:id/ingredient",
                post(add_ingredient_to_recipe_route),
            )
            .route(
                "/recipe/:recipe_id/ingredient/:ingredient_id",
                delete(delete_ingredient_from_recipe_route),
            )
            .route(
                "/recipe/:recipe_id/ingredient/:ingredient_id",
                put(update_ingredient_in_recipe_route),
            )
            .layer(OtelInResponseLayer)
            .layer(OtelAxumLayer::default())
    }

    pub fn new(
        irs: Arc<Box<dyn IngredientRepository>>,
        rrs: Arc<Box<dyn RecipeRepository>>,
        ms: MessageServiceImpl,
    ) -> Result<Self> {
        let state = AppState {
            ingredient_repository: irs,
            recipe_repository: rrs,
            message_service: ms,
        };
        let router = Self::get_router().with_state(state);

        Ok(App { router })
    }

    pub async fn serve(self, listener: tokio::net::TcpListener) -> Result<()> {
        let addr = listener.local_addr()?;
        tracing::info!("Serving on {}:{}", addr.ip(), addr.port());
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}

#[derive(Default)]
pub struct AppBuilder {
    postgres_db: Option<PgPool>,
    kafka: Option<String>,
}

impl AppBuilder {
    pub fn with_postgres_database(mut self, pool: PgPool) -> Self {
        self.postgres_db = Some(pool);

        self
    }

    pub fn with_kafka(mut self, address: &str) -> Self {
        self.kafka = Some(address.to_string());

        self
    }

    pub fn build(self) -> Result<App> {
        let irs: Box<dyn IngredientRepository> = if let Some(postgres_db) = self.postgres_db.clone()
        {
            Box::new(PostgresIngredientRepository::new(postgres_db.clone()))
        } else {
            Box::new(InMemoryIngredientRepository::new())
        };

        let rrs: Box<dyn RecipeRepository> = if let Some(postgres_db) = self.postgres_db.clone() {
            Box::new(PostgresRecipeRepository::new(postgres_db.clone()))
        } else {
            Box::new(InMemoryRecipeRepository::new())
        };

        let ms: Box<dyn MessageService> = if let Some(kafka_url) = self.kafka {
            Box::new(KafkaMessageService::new(&kafka_url)?)
        } else {
            Box::new(StubMessageService)
        };

        App::new(Arc::new(irs), Arc::new(rrs), Arc::new(ms))
    }

    pub fn new() -> Self {
        Self::default()
    }
}
