use crate::helpers::TestApp;

#[tokio::test]
async fn health_works() {
    // Arrange
    let app = TestApp::new()
        .await
        .expect("Expected App to be initialized!");

    // Act
    let response = app.server.get("/health").await;

    // Assert
    assert!(response.status_code().is_success());
    assert!(response.as_bytes().is_empty());
}
