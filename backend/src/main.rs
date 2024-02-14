use actix_session::storage::RedisSessionStore;
use backend::telemetry;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("recipes".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let config =
        backend::configuration::Settings::get().expect("Could not read configuration file");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());

    let redis_conn = config.session.get_redis_connection_string();
    let session = RedisSessionStore::new(redis_conn)
        .await
        .expect("Could not connect to redis");
    let session_key = actix_web::cookie::Key::from(config.session.key.expose_secret().as_bytes());

    let server = backend::run(
        std::net::TcpListener::bind((config.application.host, config.application.port))?,
        connection_pool,
        session,
        session_key,
    )?;
    server.await
}
