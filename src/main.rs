use std::process;
use subscriptions::Config;
use tokio::net::TcpListener;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file if exists
    dotenvy::dotenv_override().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize configuration
    let config = Config::load().unwrap_or_else(|error| {
        tracing::error!(%error, "Failed to load configuration");
        process::exit(1);
    });

    // Initialize application
    let (router, _) = subscriptions::init(config.clone())
        .await
        .unwrap_or_else(|error| {
            tracing::error!(%error, "Failed to initialize application");
            process::exit(1);
        });

    // Bind address
    let listener = TcpListener::bind((config.host.clone(), config.port))
        .await
        .unwrap_or_else(|error| {
            tracing::error!(%error, "Failed to bind address `{}:{}`", config.host, config.port);
            process::exit(1);
        });

    // Start server
    tracing::info!("Start listening on: http://{}:{}", config.host, config.port);
    if let Err(error) = axum::serve(listener, router).await {
        tracing::error!(%error, "Failed to start server");
        process::exit(1);
    }

    tracing::info!("Server gracefully shutdown");
}
