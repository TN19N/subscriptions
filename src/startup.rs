use crate::{
    Result,
    config::Config,
    handlers::{admin_dashboard, confirm, health, home, login, publish_newsletter, subscribe},
    state::AppState,
};
use axum::{
    Router,
    http::{HeaderName, Request},
    routing::{get, post},
};
use axum_messages::MessagesManagerLayer;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tower_sessions::SessionManagerLayer;
use tower_sessions_surrealdb_store::SurrealSessionStore;

const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");

pub async fn init(config: Config) -> Result<(Router, AppState)> {
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
        .layer(PropagateRequestIdLayer::new(REQUEST_ID_HEADER))
        .layer(CookieManagerLayer::new())
        .layer(SessionManagerLayer::new(SurrealSessionStore::new(state.mm.db().await?.clone(), "sessions".into())))
        .layer(MessagesManagerLayer);

    let router = Router::new()
        .route("/", get(home))
        .route("/health", get(health))
        .route("/subscriptions", post(subscribe))
        .route("/subscriptions/confirm", get(confirm))
        .route("/newsletter", post(publish_newsletter))
        .route("/login", get(login::get::login))
        .route("/login", post(login::post::login))
        .route("/admin/dashboard", get(admin_dashboard))
        .layer(middleware)
        .with_state(state.clone());

    Ok((router, state))
}
