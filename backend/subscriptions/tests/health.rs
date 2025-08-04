use rand::Rng;
use reqwest::StatusCode;
use serde::Deserialize;
use std::{error::Error, time::Duration};
use subscriptions::Config;
use surrealdb::{RecordId, Uuid};
use url::Url;

async fn spawn_app() -> Result<(Url, Config), Box<dyn Error>> {
    let mut rng = rand::rng();
    let mut config = Config::load()?;

    config.port = rng.random_range(1000..u16::MAX);
    config.database.name = Some(Uuid::new_v4().to_string());

    let url = Url::parse(&format!("http://localhost:{}", config.port))?;
    tokio::spawn(subscriptions::run(config.clone()));

    // wait for the server to start
    tokio::time::sleep(Duration::from_millis(100)).await;

    Ok((url, config))
}

#[tokio::test]
async fn health_works() -> Result<(), Box<dyn Error>> {
    // Arrange
    let (url, _) = spawn_app().await?;
    let client = reqwest::Client::new();

    // Act
    let response = client.get(url.join("/health")?).send().await?;

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    Ok(())
}

#[tokio::test]
async fn subscribe_works() -> Result<(), Box<dyn Error>> {
    // Arrange
    let (url, config) = spawn_app().await?;
    let mm = model::Manager::new(&config.database).await?;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(url.join("/subscriptions")?)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?;

    // Assert
    #[derive(Deserialize)]
    struct QueryResult {
        id: RecordId,
    }
    let result: Option<QueryResult> = mm
        .db()
        .query("SELECT id FROM ONLY subscriptions WHERE email = 'ursula_le_guin@gmail.com'")
        .await?
        .take(0)?;

    assert!(
        result.is_some(),
        "Subscription with the email 'ursula_le_guin@gmail.com' not found"
    );
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(Some(0), response.content_length());
    Ok(())
}

#[tokio::test]
async fn subscribe_failed() -> Result<(), Box<dyn Error>> {
    // Arrange
    let (url, _) = spawn_app().await?;
    let client = reqwest::Client::new();
    let test_cases = [
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing the name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(url.join("/subscriptions")?)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await?;

        // Assert
        assert_eq!(
            response.status(),
            StatusCode::UNPROCESSABLE_ENTITY,
            "The Api did not fail with 400 Bad Request when payload was {error_message}."
        );
    }

    Ok(())
}
