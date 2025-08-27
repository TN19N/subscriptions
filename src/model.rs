use secrecy::ExposeSecret;
use surrealdb::{Surreal, engine::any::Any, opt::auth::Database};
use surrealdb_migrations::MigrationRunner;
use tokio::sync::OnceCell;

use crate::{Error, Result, config::DatabaseConfig};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Subscription {
    name: String,
    email: String,
}

impl Subscription {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }
}

#[derive(Debug, Clone)]
pub struct ModelManager {
    config: DatabaseConfig,
    db: OnceCell<Surreal<Any>>,
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

    pub async fn create_subscription(&self, subscription: Subscription) -> Result<()> {
        self.db()
            .await?
            .query("CREATE subscriptions SET name = $name, email = $email")
            .bind(("name", subscription.name))
            .bind(("email", subscription.email))
            .await?;

        Ok(())
    }

    async fn connect(&self) -> Result<Surreal<Any>> {
        let config = &self.config;
        let db = Surreal::<Any>::init();

        tracing::info!("Connecting to database: {}", config.url.as_str());
        db.connect(config.url.as_str()).await?;

        if config.url.scheme() == "mem" {
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
            .up()
            .await
            .map_err(|e| Error::Migrations(e.to_string()))?;

        Ok(db)
    }
}
