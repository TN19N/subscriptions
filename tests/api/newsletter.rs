use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::{Method, StatusCode};
use serde_json::json;
use wiremock::{
    Mock, ResponseTemplate,
    matchers::{any, method},
};

use crate::helpers::{ConfirmationLinks, Credentials, TestApp};

async fn create_unconfirmed_subscriber(app: &TestApp) -> ConfirmationLinks {
    let body = [("name", "let guin"), ("email", "ursula_le_guin@gmail.com")];

    let _mock_guard = Mock::given(any())
        .respond_with(ResponseTemplate::new(StatusCode::OK))
        .named("Create unconfirmed subscriber")
        .expect(1)
        .mount_as_scoped(&app.email_server)
        .await;

    app.server.post("/subscriptions").form(&body).await;

    let email_request = app
        .email_server
        .received_requests()
        .await
        .unwrap()
        .pop()
        .unwrap();
    app.get_conformation_links(&email_request)
}

async fn create_confirmed_subscriber(app: &TestApp) {
    let confirmation_links = create_unconfirmed_subscriber(app).await;

    app.server
        .get(&format!(
            "{}?{}",
            confirmation_links.html.path(),
            confirmation_links.html.query().unwrap()
        ))
        .await
        .assert_status_success();
}

fn get_basic_authorization_header(user: &Credentials) -> String {
    format!(
        "Basic {}",
        BASE64_STANDARD.encode(format!("{}:{}", user.username, user.password))
    )
}

#[tokio::test]
async fn newsletter_are_not_delivered_to_unconfirmed_subscribers() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected the app to be inisilized!");
    create_unconfirmed_subscriber(&app).await;

    // Act
    let newsletter = serde_json::json!({
       "title": "Newsletter title",
       "content": {
           "text": "Newsletter body as plain text",
           "html": "<p>Newsletter body as html</p>",
       },
    });
    let response = app
        .server
        .post("/newsletter")
        .authorization(get_basic_authorization_header(&app.test_user))
        .json(&newsletter)
        .await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn newsletter_are_delivered_to_confirmed_subscribers() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected the app to be inisilized!");
    create_confirmed_subscriber(&app).await;

    Mock::given(any())
        .and(method(Method::POST))
        .respond_with(ResponseTemplate::new(StatusCode::OK))
        .expect(1)
        .mount(&app.email_server)
        .await;

    // Act
    let newsletter = serde_json::json!({
       "title": "Newsletter title",
       "content": {
           "text": "Newsletter body as plain text",
           "html": "<p>Newsletter body as html</p>",
       },
    });
    let response = app
        .server
        .post("/newsletter")
        .authorization(get_basic_authorization_header(&app.test_user))
        .json(&newsletter)
        .await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::OK);
}

#[tokio::test]
async fn newsletter_return_400_for_invalid_data() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected the app to be inisilized!");
    let test_cases = [
        (
            json!({
               "content": {
                   "text": "Newsletter body as plain text",
                   "html": "<p>Newsletter body as html</p>",
               },
            }),
            "messing title",
        ),
        (
            json!({
               "title": "Newsletter title",
            }),
            "messing content",
        ),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = app
            .server
            .post("/newsletter")
            .authorization(get_basic_authorization_header(&app.test_user))
            .json(&invalid_body)
            .await;

        // Assert
        assert_eq!(
            StatusCode::UNPROCESSABLE_ENTITY,
            response.status_code(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

#[tokio::test]
async fn requests_messing_authorization_are_rejected() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected the app to be inisilized!");

    // Act
    let response = app
        .server
        .post("/newsletter")
        .json(&json!({
            "title": "Newsletter title",
            "content": {
                "text": "Newsletter body as plain text",
                "html": "<p>Newsletter body as html</p>",
            },
        }))
        .await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        r#"Basic realm="publish""#,
        response.header("WWW-Authenticate")
    );
}

#[tokio::test]
async fn non_existing_user_is_rejected() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected the app to be inisilized!");

    // Act
    let response = app
        .server
        .post("/newsletter")
        .authorization(get_basic_authorization_header(&Credentials {
            username: "what".into(),
            password: "password-non".into(),
        }))
        .json(&json!({
            "title": "Newsletter title",
            "content": {
                "text": "Newsletter body as plain text",
                "html": "<p>Newsletter body as html</p>",
            },
        }))
        .await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        r#"Basic realm="publish""#,
        response.header("WWW-Authenticate")
    );
}

#[tokio::test]
async fn invalid_password_is_rejected() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected the app to be inisilized!");

    // Act
    let response = app
        .server
        .post("/newsletter")
        .authorization(get_basic_authorization_header(&Credentials {
            username: app.test_user.username.clone(),
            password: "invalid-password".into(),
        }))
        .json(&json!({
            "title": "Newsletter title",
            "content": {
                "text": "Newsletter body as plain text",
                "html": "<p>Newsletter body as html</p>",
            },
        }))
        .await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    assert_eq!(
        r#"Basic realm="publish""#,
        response.header("WWW-Authenticate")
    );
}
