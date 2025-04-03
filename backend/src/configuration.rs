use eyre::Context;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use tokio::net::TcpListener;

#[derive(strum::Display, Debug)]
pub enum Environment {
    #[strum(serialize = "dev")]
    Development,
    #[strum(serialize = "prod")]
    Production,
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "dev" => Ok(Self::Development),
            "prod" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a valid environment\nUse either `dev` or `prod`.",
                other
            )),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub session: SessionSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    #[serde(default)]
    pub require_ssl: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SessionSettings {
    pub key: Secret<String>,
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
    #[serde(default)]
    pub ssl: bool,
}

impl Settings {
    pub fn get() -> color_eyre::Result<Self> {
        let base_path = std::env::current_dir().wrap_err("Could not get current directory")?;
        let config_path = base_path.join("config");

        let environment: Environment = std::env::var("APP_ENV")
            .unwrap_or_else(|_| "dev".into())
            .try_into()
            .map_err(|e| eyre::eyre!("{}", e))?;

        let env_config = format!("{}.yaml", environment);

        let settings = config::Config::builder()
            .add_source(config::File::from(config_path.join("base.yaml")))
            .add_source(config::File::from(config_path.join(env_config)))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;
        Ok(settings.try_deserialize::<Self>()?)
    }
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
        // .log_statements(tracing_log::log::LevelFilter::Trace)
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Disable
        };
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .ssl_mode(mode)
            .password(self.password.expose_secret())
    }
}

impl SessionSettings {
    pub fn get_redis_connection_string(&self) -> String {
        let connection_prefix = if self.ssl { "rediss://" } else { "redis://" };

        format!(
            "{}{}:{}@{}:{}",
            connection_prefix,
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        )
        .to_string()
    }
}

impl ApplicationSettings {
    pub async fn get_listener(&self) -> color_eyre::Result<TcpListener> {
        Ok(TcpListener::bind((self.host.clone(), self.port)).await?)
    }
}
