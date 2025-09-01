use crate::helpers::TestApp;
use reqwest::{Method, StatusCode};
use serde::Deserialize;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{any, method},
};

#[tokio::test]
async fn confirmations_without_token_are_rejected_with_400() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected App to be initialized!");

    // Act
    let response = app.server.get("/subscriptions/confirm").await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST)
}

#[tokio::test]
async fn confirmation_works() {
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
    _ = app.server.post("/subscriptions").form(&body).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_conformation_links(email_request);

    let response = app
        .server
        .get(&format!(
            "{}?{}",
            confirmation_links.html.path(),
            confirmation_links.html.query().unwrap()
        ))
        .await;

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

    // Assert
    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(result.email, "ursula_le_guin@gmail.com");
    assert_eq!(result.name, "le guin");
    assert_eq!(result.status, "CONFIRMED");
}
