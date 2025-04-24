use eyre::eyre;
use figment::{
    providers::{Env, Format, Yaml},
    Figment, Profile,
};
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

impl From<Environment> for Profile {
    #[coverage(off)]
    fn from(val: Environment) -> Self {
        Profile::new(&val.to_string())
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    #[coverage(off)]
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
    pub database: Option<DatabaseSettings>,
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
    #[coverage(off)]
    pub fn get() -> color_eyre::Result<Self> {
        let environment: Environment = std::env::var("APP_ENV")
            .unwrap_or_else(|_| "dev".into())
            .try_into()
            .map_err(|e| eyre::eyre!("{}", e))?;

        let figment = Figment::new()
            .merge(Yaml::file("config/base.yaml"))
            .merge(Env::prefixed("DEEPDISH_"))
            .merge(Yaml::file("config/dev.yaml").profile("dev"))
            .merge(Yaml::file("config/prod.yaml").profile("prod"));

        figment
            .select(environment)
            .extract()
            .map_err(|_| eyre!("Could not load config file"))
    }
}

impl DatabaseSettings {
    #[coverage(off)]
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }

    #[coverage(off)]
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
    #[coverage(off)]
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
