use axum_test::TestServer;
use std::str::FromStr;
use subscriptions::{AppState, Config};
use tokio::sync::OnceCell;
use tracing_subscriber::prelude::*;
use url::Url;
use wiremock::MockServer;

pub type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub struct TestApp {
    pub server: TestServer,
    pub state: AppState,
    pub email_server: MockServer,
    pub test_user: Credentials,
}

pub struct ConfirmationLinks {
    pub html: Url,
    pub plain_text: Url,
}

impl TestApp {
    pub async fn new() -> Result<TestApp> {
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

        let mut config = Config::load()?;
        let email_server = MockServer::start().await;
        config.email_client.base_url = Url::from_str(&email_server.uri())?;

        let test_user = Credentials {
            password: "password".into(),
            username: "username".into(),
        };
        let (router, state) = subscriptions::init(config).await?;

        // create test user for valid authentications
        state
            .mm
            .db()
            .await
            .unwrap()
            .query(
                r#"
                INSERT INTO users {
                    username: $username,
                    password: crypto::argon2::generate($password)
                }
            "#,
            )
            .bind(("username", test_user.username.clone()))
            .bind(("password", test_user.password.clone()))
            .await
            .unwrap()
            .check()
            .unwrap();

        Ok(TestApp {
            server: TestServer::builder().save_cookies().build(router)?,
            state,
            email_server,
            test_user,
        })
    }

    pub fn get_conformation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        // Parse the body as json
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

        // Extract the link from one of requests fields
        let get_link = |s: &str| -> Url {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            let raw_link = links[0].as_str().to_owned();
            Url::parse(&raw_link).unwrap()
        };

        let html = get_link(body["HtmlBody"].as_str().unwrap());
        let plain_text = get_link(body["TextBody"].as_str().unwrap());

        ConfirmationLinks { html, plain_text }
    }
}
