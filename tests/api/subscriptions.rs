use crate::helpers::TestApp;
use reqwest::{Method, StatusCode};
use serde::Deserialize;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{any, method},
};

#[tokio::test]
async fn subscribe_works() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected App to be initialized!");
    let body = [("name", "let guin"), ("email", "ursula_le_guin@gmail.com")];

    Mock::given(any())
        .and(method(Method::POST))
        .respond_with(ResponseTemplate::new(StatusCode::OK))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    let response = app.server.post("/subscriptions").form(&body).await;

    // Assert
    #[derive(Deserialize)]
    struct QueryResult {}
    let result = app
        .state
        .mm
        .db()
        .await
        .expect("Expected Database to be connected")
        .query("SELECT id FROM ONLY subscriptions WHERE email = 'ursula_le_guin@gmail.com'")
        .await
        .expect("query should be successful")
        .take::<Option<QueryResult>>(0)
        .expect("query result should be valid");

    assert!(
        result.is_some(),
        "Subscription with the email 'ursula_le_guin@gmail.com' not found"
    );
    assert_eq!(response.status_code(), StatusCode::CREATED);
    assert!(response.as_bytes().is_empty());
}

#[tokio::test]
async fn subscribe_failed() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected App to be initialized!");
    let test_cases = [
        (
            vec![("name", "le guin")],
            "missing the email",
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            vec![("email", "ursula_le_guin@gmail.com")],
            "missing the name",
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            vec![],
            "missing the name and email",
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            vec![("name", ""), ("email", "ursula_le_guin@gmail.com")],
            "empty name",
            StatusCode::BAD_REQUEST,
        ),
        (
            vec![("name", "wow"), ("email", "")],
            "empty email",
            StatusCode::BAD_REQUEST,
        ),
        (
            vec![("name", "Ursula"), ("email", "definitely-not-an-email")],
            "invalid email",
            StatusCode::BAD_REQUEST,
        ),
    ];

    for (invalid_body, error_message, expected_status) in test_cases {
        // Act
        let response = app.server.post("/subscriptions").form(&invalid_body).await;

        // Assert
        assert_eq!(
            response.status_code(),
            expected_status,
            "The Api did not fail with status {expected_status} when payload was {error_message}."
        );
    }
}

#[tokio::test]
pub async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected App to be initialized!");
    let body = [("name", "let guin"), ("email", "ursula_le_guin@gmail.com")];

    Mock::given(any())
        .and(method(Method::POST))
        .respond_with(ResponseTemplate::new(StatusCode::OK))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    _ = app.server.post("/subscriptions").form(&body).await;

    // Assert
    // Mock assert on drop
}

#[tokio::test]
pub async fn subscribe_sends_a_confirmation_email_with_a_link() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected App to be initialized!");
    let body = [("name", "let guin"), ("email", "ursula_le_guin@gmail.com")];

    Mock::given(any())
        .and(method(Method::POST))
        .respond_with(ResponseTemplate::new(StatusCode::OK))
        .mount(&app.email_server)
        .await;

    // Act
    _ = app.server.post("/subscriptions").form(&body).await;

    // Assert
    // Get the first interapted request
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_conformation_links(email_request);
    // the two link should be adentical
    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[tokio::test]
pub async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected App to be initialized!");
    let body = [("name", "le guin"), ("email", "ursula_le_guin@gmail.com")];

    Mock::given(any())
        .and(method(Method::POST))
        .respond_with(ResponseTemplate::new(StatusCode::OK))
        .mount(&app.email_server)
        .await;

    // Act
    let response = app.server.post("/subscriptions").form(&body).await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::CREATED);

    #[derive(Debug, Deserialize)]
    struct QueryResult {
        email: String,
        name: String,
        status: String,
    }
    let result = app
        .state
        .mm
        .db()
        .await
        .expect("Expected Database to be connected")
        .query("SELECT * FROM ONLY subscriptions WHERE email = 'ursula_le_guin@gmail.com'")
        .await
        .expect("query should be successful")
        .take::<Option<QueryResult>>(0)
        .expect("query result should be valid")
        .expect("query result should to not be empty");

    assert_eq!(result.email, "ursula_le_guin@gmail.com");
    assert_eq!(result.name, "le guin");
    assert_eq!(result.status, "PENDING");
}
