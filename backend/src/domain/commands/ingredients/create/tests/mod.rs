mod __tests__;
mod in_memory {
    use super::__tests__;
    use crate::domain::{
        repositories::ingredients::in_memory::InMemoryIngredientRepository,
        services::message::stub::StubMessageService,
    };

    #[tokio::test]
    async fn creates_an_ingredient() {
        __tests__::creates_an_ingredient().await;
    }

    #[tokio::test]
    async fn incorrect_diets_do_not_get_included() {
        __tests__::incorrect_diets_do_not_get_included(
            InMemoryIngredientRepository::new(),
            StubMessageService,
        )
        .await;
    }

    #[tokio::test]
    async fn empty_name_fails() {
        __tests__::empty_name_fails(InMemoryIngredientRepository::new(), StubMessageService).await;
    }

    #[tokio::test]
    async fn empty_description_fails() {
        __tests__::empty_description_fails(InMemoryIngredientRepository::new(), StubMessageService)
            .await;
    }

    #[tokio::test]
    async fn incorrect_ingredient_is_not_persisted() {
        __tests__::incorrect_ingredient_is_not_persisted(
            InMemoryIngredientRepository::new(),
            StubMessageService,
        )
        .await;
    }

    #[tokio::test]
    async fn inserting_an_ingredient_with_a_name_that_already_exists_fails() {
        let repo = InMemoryIngredientRepository::new();
        __tests__::inserting_an_ingredient_with_a_name_that_already_exists_fails(
            repo,
            StubMessageService,
        )
        .await
    }
}

mod sql {
    use super::__tests__;
    use crate::domain::{
        repositories::ingredients::postgres::PostgresIngredientRepository,
        services::message::stub::StubMessageService,
    };

    use sqlx::PgPool;

    #[sqlx::test]
    async fn incorrect_diets_do_not_get_included(pool: PgPool) {
        __tests__::incorrect_diets_do_not_get_included(
            PostgresIngredientRepository::new(pool),
            StubMessageService,
        )
        .await;
    }

    #[sqlx::test]
    async fn empty_name_fails(pool: PgPool) {
        __tests__::empty_name_fails(PostgresIngredientRepository::new(pool), StubMessageService)
            .await;
    }

    #[sqlx::test]
    async fn empty_description_fails(pool: PgPool) {
        __tests__::empty_description_fails(
            PostgresIngredientRepository::new(pool),
            StubMessageService,
        )
        .await;
    }

    #[sqlx::test]
    async fn incorrect_ingredient_is_not_persisted(pool: PgPool) {
        __tests__::incorrect_ingredient_is_not_persisted(
            PostgresIngredientRepository::new(pool),
            StubMessageService,
        )
        .await;
    }

    #[sqlx::test]
    async fn inserting_an_ingredient_with_a_name_that_already_exists_fails(pool: PgPool) {
        let repo = PostgresIngredientRepository::new(pool);
        __tests__::inserting_an_ingredient_with_a_name_that_already_exists_fails(
            repo,
            StubMessageService,
        )
        .await
    }
}
