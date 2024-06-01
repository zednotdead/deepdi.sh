mod routes;
mod errors;

use std::sync::Arc;

use crate::domain::repositories::{
    ingredients::{
        in_memory::InMemoryIngredientRepository, postgres::PostgresIngredientRepository,
        IngredientRepository, IngredientRepositoryService,
    },
    recipe::{
        in_memory::InMemoryRecipeRepository, postgres::PostgresRecipeRepository, RecipeRepository,
        RecipeRepositoryService,
    },
};
use axum::{
    routing::{get, post, put},
    Router,
};
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use color_eyre::Result;
use sqlx::PgPool;

use self::routes::{
    ingredients::{
        all_ingredients::get_all_ingredients_route, create_ingredient::create_ingredient_route,
        get_ingredient_by_id::get_ingredient_by_id_route,
        update_ingredient::update_ingredient_route,
    },
    recipes::{create_recipe::create_recipe_route, get_recipe_by_id::get_recipe_by_id_route},
};

pub struct App {
    router: Router,
}

#[derive(Clone)]
pub struct AppState {
    pub ingredient_repository: IngredientRepositoryService,
    pub recipe_repository: RecipeRepositoryService,
}

impl App {
    fn get_router() -> Router<AppState> {
        Router::new()
            .route("/ingredient/create", post(create_ingredient_route))
            .route("/ingredient/:id", put(update_ingredient_route))
            .route("/ingredient/:id", get(get_ingredient_by_id_route))
            .route("/ingredient", get(get_all_ingredients_route))
            .route("/recipe/create", post(create_recipe_route))
            .route("/recipe/:id", get(get_recipe_by_id_route))
            .layer(OtelInResponseLayer)
            .layer(OtelAxumLayer::default())
    }

    pub fn new<I: IngredientRepository + 'static, R: RecipeRepository + 'static>(
        irs: I,
        rrs: R,
    ) -> Result<Self> {
        let ingredient_repository: IngredientRepositoryService = Arc::new(Box::new(irs));
        let recipe_repository: RecipeRepositoryService = Arc::new(Box::new(rrs));
        let state = AppState {
            ingredient_repository,
            recipe_repository,
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
}

impl AppBuilder {
    pub fn with_postgres_database(mut self, pool: PgPool) -> Self {
        self.postgres_db = Some(pool);

        self
    }

    pub fn build(self) -> Result<App> {
        if let Some(postgres_db) = self.postgres_db {
            App::new(
                PostgresIngredientRepository::new(postgres_db.clone()),
                PostgresRecipeRepository::new(postgres_db),
            )
        } else {
            App::new(
                InMemoryIngredientRepository::new(),
                InMemoryRecipeRepository::new(),
            )
        }
    }

    pub fn new() -> Self {
        Self::default()
    }
}
