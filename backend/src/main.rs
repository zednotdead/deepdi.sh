#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use backend::{api::AppBuilder, configuration::Settings, tracing::init_tracing};
use color_eyre::Result;

use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // TODO: add more log points
    init_tracing()?;

    let config = Settings::get().unwrap();
    let db = PgPool::connect_lazy_with(config.database.with_db());
    let app = AppBuilder::new().with_postgres_database(db).build()?;
    let listener = config.application.get_listener().await?;
    app.serve(listener).await?;

    Ok(())
}
