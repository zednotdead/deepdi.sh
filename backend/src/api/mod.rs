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

    pub async fn serve(&self, listener: tokio::net::TcpListener) -> Result<()> {
        let addr = listener.local_addr()?;
        log::info!("Serving on {}:{}", addr.ip(), addr.port());
        axum::serve(listener, self.router.clone()).await?;
        Ok(())
    }
}

#[derive(Default, Clone)]
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

    fn get_ingredient_recipe_repository(&self) -> Box<dyn IngredientRepository> {
        if let Some(postgres_db) = &self.postgres_db {
            tracing::info!("Using Postgres for ingredients database");
            Box::new(PostgresIngredientRepository::new(postgres_db.clone()))
        } else {
            tracing::warn!("You are using a debug service, please move to something that is actually working.");
            Box::new(InMemoryIngredientRepository::new())
        }
    }

    fn get_recipe_repository(&self) -> Box<dyn RecipeRepository> {
        if let Some(postgres_db) = &self.postgres_db {
            tracing::info!("Using Postgres for recipe database");
            Box::new(PostgresRecipeRepository::new(postgres_db.clone()))
        } else {
            tracing::warn!("You are using a debug service, please move to something that is actually working.");
            Box::new(InMemoryRecipeRepository::new())
        }
    }

    fn get_message_service(&self) -> eyre::Result<Box<dyn MessageService>> {
        if let Some(kafka_url) = &self.kafka {
            tracing::info!("Using Kafka messaging service");
            Ok(Box::new(KafkaMessageService::new(kafka_url)?))
        } else {
            tracing::info!("Using a stub messaging service");
            tracing::warn!("You are using a debug service, please move to something that is actually working.");
            Ok(Box::new(StubMessageService))
        }
    }

    pub fn build(self) -> Result<App> {
        let irs = Arc::new(self.get_ingredient_recipe_repository());
        let rrs = Arc::new(self.get_recipe_repository());
        let ms = Arc::new(self.get_message_service()?);

        App::new(irs, rrs, ms)
    }

    pub fn new() -> Self {
        Self::default()
    }
}
