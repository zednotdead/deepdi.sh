use backend::{api::AppBuilder, configuration::Settings, tracing::init_tracing};
use color_eyre::Result;

use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // TODO: add more log points
    init_tracing()?;

    let config = Settings::get().unwrap();

    let mut app_builder = AppBuilder::new();

    if let Some(db) = config.database {
        app_builder = app_builder.with_postgres_database(PgPool::connect_lazy_with(db.with_db()));
    };

    let app = app_builder.with_kafka("localhost:9092").build()?;

    let listener = config.application.get_listener().await?;
    app.serve(listener).await?;

    Ok(())
}
