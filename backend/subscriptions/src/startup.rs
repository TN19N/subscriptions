use std::net::Ipv4Addr;
use crate::{
    Config,
    routes::{health, subscribe},
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

pub async fn run(config: Config) {
    let state = AppState::new(&config).await.unwrap();

    let router = Router::new()
        .route("/health", get(health))
        .route("/subscriptions", post(subscribe))
        .with_state(state);

    let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, config.port))
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
