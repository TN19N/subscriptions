use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub database: model::Config,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        // load environment variables from .env file if it exists
        dotenvy::dotenv_override().ok();

        config::Config::builder()
            .add_source(config::Environment::with_prefix(env!("CARGO_PKG_NAME")).separator("__"))
            .build()?
            .try_deserialize()
    }
}
