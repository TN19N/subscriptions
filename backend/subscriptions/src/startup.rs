use crate::{
    Config, Result,
    routes::{health, subscribe},
    state::AppState,
};
use axum::{
    Router,
    http::{HeaderName, Request},
    routing::{get, post},
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

pub async fn init(config: &Config) -> Result<Router> {
    let state = AppState::new(config).await?;

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(REQUEST_ID_HEADER, MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the request id as generated.
                let request_id = request.headers().get(REQUEST_ID_HEADER);
                tracing::info_span!("http_request", method = ?request.method(), request_id = ?request_id, uri = ?request.uri())
            }),
        )
        // send headers from request to response headers
        .layer(PropagateRequestIdLayer::new(REQUEST_ID_HEADER));

    let router = Router::new()
        .route("/health", get(health))
        .route("/subscriptions", post(subscribe))
        .layer(middleware)
        .with_state(state);

    Ok(router)
}

pub async fn run(config: Config) -> Result {
    let router = init(&config).await?;

    let listener = TcpListener::bind((config.application.host, config.application.port)).await?;

    tracing::info!(
        "Start listening on: http://localhost:{}",
        config.application.port
    );
    Ok(axum::serve(listener, router).await?)
}
