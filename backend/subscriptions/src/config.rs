use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub application: ApplicationConfig,
    pub database: model::Config,
}
// hhhhhhhhhhh--
impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::with_prefix(env!("CARGO_PKG_NAME")).separator("__"))
            .build()?
            .try_deserialize()
    }
}
