use crate::domain::SubscriberEmail;
use config::ConfigError;
use secrecy::SecretString;
use serde::Deserialize;
use std::time::Duration;
use url::Url;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub base_url: Url,
    pub database: DatabaseConfig,
    pub email_client: EmailClientConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmailClientConfig {
    pub sender_email: SubscriberEmail,
    pub base_url: Url,
    pub auth_token: SecretString,
    #[serde(with = "serde_humantime")]
    pub timeout: Duration,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub base_url: Url,
    pub username: String,
    pub password: secrecy::SecretString,
    pub namespace: String,
    pub name: String,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::with_prefix(env!("CARGO_PKG_NAME")).separator("__"))
            .build()?
            .try_deserialize()
    }
}
