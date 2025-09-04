use crate::helpers::TestApp;
use reqwest::{StatusCode, header::LOCATION};
use serde_json::json;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrenge
    let app = TestApp::new()
        .await
        .expect("Expected the app to be inissilized!");

    // Act
    let response = app
        .server
        .post("/login")
        .form(&json!({
            "username": "random-username",
            "password": "random-password",
        }))
        .await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::SEE_OTHER);
    assert_eq!(response.header(LOCATION), "/login");
    let html_page = app.server.get("/login").await;
    assert!(
        html_page
            .text()
            .contains(r#"<p><i>Authentication Failed</i></p>"#)
    );

    // reload the page
    let html_page = app.server.get("/login").await;
    assert!(
        !html_page
            .text()
            .contains(r#"<p><i>Authentication Failed</i></p>"#)
    );
}

#[tokio::test]
async fn redirect_to_admin_dashboard_after_login_login_success() {
    // Arrenge
    let app = TestApp::new()
        .await
        .expect("Expected the app to be inissilized!");

    // Act
    let response = app
        .server
        .post("/login")
        .form(&json!({
            "username": app.test_user.username,
            "password": app.test_user.password,
        }))
        .await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::SEE_OTHER);
    assert_eq!(response.header(LOCATION), "/admin/dashboard");
    let response = app.server.get("/admin/dashboard").await;
    assert!(
        response
            .text()
            .contains(&format!("Welcome {}", app.test_user.username))
    );
}
