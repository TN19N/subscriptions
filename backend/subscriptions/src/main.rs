use subscriptions::{Config, Result};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result {
    // Load environment variables from .env file if exists
    dotenvy::dotenv_override().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load().map_err(Box::new)?;

    subscriptions::run(config).await
}
