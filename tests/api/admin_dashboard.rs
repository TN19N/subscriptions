use reqwest::{StatusCode, header};

use crate::helpers::TestApp;

#[tokio::test]
async fn you_must_be_logged_to_access_admin_dashboard() {
    // Arrange
    let app = TestApp::new().await.expect("Failed to start test app");

    // Act
    let response = app.server.get("/admin/dashboard").await;

    // Assert
    assert_eq!(response.status_code(), StatusCode::SEE_OTHER);
    assert_eq!(response.header(header::LOCATION), "/login");
}
