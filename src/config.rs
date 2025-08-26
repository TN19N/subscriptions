use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: url::Url,
    pub username: String,
    pub password: secrecy::SecretString,
    pub namespace: String,
    pub name: String,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::with_prefix(env!("CARGO_PKG_NAME")).separator("__"))
            .build()?
            .try_deserialize()
    }
}
