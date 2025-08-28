use axum::{
    Router,
    body::Body,
    http::{self, Method, StatusCode},
};
use http_body_util::BodyExt;
use serde::Deserialize;
use subscriptions::{AppState, Config};
use surrealdb::RecordId;
use tokio::sync::OnceCell;
use tower::ServiceExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

async fn init() -> Result<(Router, AppState)> {
    static TRACING: OnceCell<()> = OnceCell::const_new();

    TRACING
        .get_or_init(async || {
            // Load environment variables from .env file if exists
            dotenvy::from_filename_override(".env.test").ok();

            // Initialize tracing
            tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::from_default_env())
                .with(tracing_subscriber::fmt::layer().with_test_writer())
                .init();
        })
        .await;

    let config = Config::load()?;
    Ok(subscriptions::init(&config).await?)
}

#[tokio::test]
async fn health_works() {
    // Arrange
    let (app, _) = init().await.expect("Expected App to be initialized!");

    // Act
    let response = app
        .oneshot(
            http::Request::builder()
                .method(Method::GET)
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("Expected Request to be successful");

    // Assert
    assert!(response.status().is_success());
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}

#[tokio::test]
async fn subscribe_works() {
    // Arrange
    let (app, state) = init().await.expect("Expected App to be initialized!");

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = app
        .oneshot(
            http::Request::builder()
                .method(Method::POST)
                .uri("/subscriptions")
                .header(
                    http::header::CONTENT_TYPE,
                    mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .expect("Expected Request to be successful");

    // Assert
    #[derive(Deserialize)]
    struct QueryResult {
        #[serde(rename = "id")]
        _id: RecordId,
    }
    let result: Option<QueryResult> = state
        .mm
        .db()
        .await
        .expect("Expected Database to be connected")
        .query("SELECT id FROM ONLY subscriptions WHERE email = 'ursula_le_guin@gmail.com'")
        .await
        .expect("query should be successful")
        .take(0)
        .expect("query result should be valid");

    assert!(
        result.is_some(),
        "Subscription with the email 'ursula_le_guin@gmail.com' not found"
    );
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    assert!(body.is_empty());
}

#[tokio::test]
async fn subscribe_failed() {
    // Arrange
    let (app, _) = init().await.expect("Expected App to be initialized!");
    let test_cases = [
        (
            "name=le%20guin",
            "missing the email",
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            "email=ursula_le_guin%40gmail.com",
            "missing the name",
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            "",
            "missing the name and email",
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            "name=&email=ursula_le_guin%40gmail.com",
            "empty name",
            StatusCode::BAD_REQUEST,
        ),
        ("name=Ursula&email=", "empty email", StatusCode::BAD_REQUEST),
        (
            "name=Ursula&email=definitely-not-an-email",
            "invalid email",
            StatusCode::BAD_REQUEST,
        ),
    ];

    for (invalid_body, error_message, expected_status) in test_cases {
        // Act
        let response = app
            .clone()
            .oneshot(
                http::Request::builder()
                    .method(Method::POST)
                    .uri("/subscriptions")
                    .header(
                        http::header::CONTENT_TYPE,
                        mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                    )
                    .body(Body::from(invalid_body))
                    .unwrap(),
            )
            .await
            .expect("Expected Request to be successful");

        // Assert
        assert_eq!(
            response.status(),
            expected_status,
            "The Api did not fail with status {expected_status} when payload was {error_message}."
        );
    }
}
