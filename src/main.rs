use subscriptions::{Config, Error};
use tokio::net::TcpListener;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load environment variables from .env file if exists
    dotenvy::dotenv_override().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize configuration
    let config = Config::load()?;

    // Initialize application
    let (router, _) = subscriptions::init(config.clone()).await?;

    // Bind address
    let listener = TcpListener::bind((config.host.clone(), config.port)).await?;

    // Start server
    tracing::info!("Start listening on: http://{}:{}", config.host, config.port);
    axum::serve(listener, router).await?;

    tracing::info!("Server gracefully shutdown");

    Ok(())
}
