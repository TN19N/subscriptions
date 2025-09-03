use crate::{Error, Result, config::DatabaseConfig, domain};
use include_dir::include_dir;
use secrecy::ExposeSecret;
use serde::Deserialize;
use surrealdb::{Surreal, engine::any::Any, opt::auth::Database};
use surrealdb_migrations::MigrationRunner;
use tokio::sync::OnceCell;

#[derive(Debug, Clone)]
pub struct ModelManager {
    config: DatabaseConfig,
    db: OnceCell<Surreal<Any>>,
}

#[derive(Debug, Deserialize)]
pub struct ConfirmedSubscriber {
    pub email: String,
}

impl ModelManager {
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            config,
            db: OnceCell::new(),
        }
    }

    pub async fn db(&self) -> Result<&Surreal<Any>> {
        self.db.get_or_try_init(async || self.connect().await).await
    }

    pub async fn create_subscriber(
        &self,
        subscriber: &domain::Subscriber,
        token: &str,
    ) -> Result<()> {
        self.db()
            .await?
            .query(
                r#"
                BEGIN TRANSACTION;
                LET $subscription_token = (CREATE ONLY subscription_tokens CONTENT { token: $token_val });
                CREATE subscriptions CONTENT {
                    email: $email,
                    name: $name,
                    token: $subscription_token.id
                };
                COMMIT TRANSACTION;
            "#,
            )
            .bind(("token_val", token.to_string()))
            .bind(("email", subscriber.email.as_ref().to_string()))
            .bind(("name", subscriber.name.as_ref().to_string()))
            .await?
            .check()?;

        Ok(())
    }

    pub async fn confirm_subscriber(&self, token: String) -> Result<()> {
        self.db()
            .await?
            .query(
                r#"
                UPDATE subscriptions
                SET status = 'CONFIRMED'
                WHERE token = (
                    SELECT id
                    FROM ONLY subscription_tokens
                    WHERE token = $token_val
                ).id
                AND status = 'PENDING';
            "#,
            )
            .bind(("token_val", token))
            .await?;

        Ok(())
    }

    pub async fn get_confirmed_subscribers(&self) -> Result<Vec<ConfirmedSubscriber>> {
        Ok(self
            .db()
            .await?
            .query(
                r#"
                SELECT * FROM subscriptions
                WHERE status = 'CONFIRMED';
            "#,
            )
            .await?
            .take(0)?)
    }

    async fn connect(&self) -> Result<Surreal<Any>> {
        let config = &self.config;
        let db = Surreal::<Any>::init();

        tracing::info!("Connecting to database: {}", config.base_url.as_str());
        db.connect(config.base_url.as_str()).await?;

        if config.base_url.scheme() == "mem" {
            db.query(format!("DEFINE NAMESPACE {}", config.namespace))
                .query(format!("DEFINE DATABASE {}", config.name))
                .query(format!("USE NS {} DB {}", config.namespace, config.name))
                .query(format!(
                    "DEFINE USER {} ON DATABASE PASSWORD '{}' ROLES OWNER",
                    config.username,
                    config.password.expose_secret()
                ))
                .await?;
        }

        db.signin(Database {
            username: &config.username,
            password: config.password.expose_secret(),
            namespace: &config.namespace,
            database: &config.name,
        })
        .await?;

        // Apply Migrations
        MigrationRunner::new(&db)
            .load_files(&include_dir!("$CARGO_MANIFEST_DIR/surrealdb"))
            .up()
            .await
            .map_err(|e| Error::Migrations(e.to_string()))?;

        Ok(db)
    }
}
