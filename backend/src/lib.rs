pub mod configuration;
pub mod modules;
pub mod telemetry;

use actix_session::{config::PersistentSession, storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration as CookieDuration, Key},
    dev::Server,
    web, App, HttpServer,
};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::modules::user::middleware::{LoginStatus, LoginStatusChecker};

pub fn run(
    listener: std::net::TcpListener,
    database: PgPool,
    session: RedisSessionStore,
    session_key: Key,
) -> Result<Server, std::io::Error> {
    let addr = listener.local_addr()?;
    tracing::info!("Starting listening on {}:{}", addr.ip(), addr.port());
    let database = web::Data::new(database);
    let session_lifecycle = PersistentSession::default().session_ttl(CookieDuration::weeks(1));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(
                SessionMiddleware::builder(session.clone(), session_key.clone())
                    .session_lifecycle(session_lifecycle.clone())
                    .build(),
            )
            .route("/health", web::get().to(modules::health::health_check))
            .route(
                "/user/signup",
                web::post()
                    .to(modules::user::log_in)
                    .wrap(LoginStatusChecker::new(LoginStatus::LoggedOut)),
            )
            .route(
                "/user/login",
                web::post()
                    .to(modules::user::log_in)
                    .wrap(LoginStatusChecker::new(LoginStatus::LoggedOut)),
            )
            .route(
                "/user/logout",
                web::post()
                    .to(modules::user::log_out)
                    .wrap(LoginStatusChecker::new(LoginStatus::LoggedIn)),
            )
            .app_data(database.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
