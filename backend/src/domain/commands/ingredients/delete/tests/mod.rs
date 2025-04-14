mod __tests__;
mod in_memory {
    use super::__tests__;
    use crate::domain::{
        repositories::{
            ingredients::in_memory::InMemoryIngredientRepository,
            recipe::in_memory::InMemoryRecipeRepository,
        },
        services::message::stub::StubMessageService,
    };

    #[tokio::test]
    async fn deleting_works() {
        let repo = InMemoryIngredientRepository::new();
        let recipe_repo = InMemoryRecipeRepository::new();
        __tests__::deleting_works(repo, recipe_repo, StubMessageService).await
    }

    #[tokio::test]
    async fn deleting_nonexistent_ingredient_errors() {
        let repo = InMemoryIngredientRepository::new();
        let recipe_repo = InMemoryRecipeRepository::new();
        __tests__::deleting_nonexistent_ingredient_errors(repo, recipe_repo, StubMessageService)
            .await
    }

    #[tokio::test]
    async fn deleting_an_ingredient_still_in_use_by_recipes_errors() {
        let repo = InMemoryIngredientRepository::new();
        let recipe_repo = InMemoryRecipeRepository::new();
        __tests__::deleting_an_ingredient_still_in_use_by_recipes_errors(
            repo,
            recipe_repo,
            StubMessageService,
        )
        .await
    }
}

mod sql {
    use super::__tests__;
    use crate::domain::{
        repositories::{
            ingredients::postgres::PostgresIngredientRepository,
            recipe::postgres::PostgresRecipeRepository,
        },
        services::message::stub::StubMessageService,
    };

    use sqlx::PgPool;

    #[sqlx::test]
    async fn deleting_works(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool.clone());
        let recipe_repo = PostgresRecipeRepository::new(pool);
        __tests__::deleting_works(repo, recipe_repo, StubMessageService).await
    }

    #[sqlx::test]
    async fn deleting_nonexistent_ingredient_errors(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool.clone());
        let recipe_repo = PostgresRecipeRepository::new(pool);
        __tests__::deleting_works(repo, recipe_repo, StubMessageService).await
    }

    #[sqlx::test]
    async fn deleting_an_ingredient_still_in_use_by_recipes_errors(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool.clone());
        let recipe_repo = PostgresRecipeRepository::new(pool);
        __tests__::deleting_an_ingredient_still_in_use_by_recipes_errors(
            repo,
            recipe_repo,
            StubMessageService,
        )
        .await
    }
}
