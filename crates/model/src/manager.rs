use crate::{Config, Error, Result};
use include_dir::{Dir, include_dir};
use secrecy::ExposeSecret;
use surrealdb::{Surreal, engine::any::Any, opt::auth::Database};
use surrealdb_migrations::MigrationRunner;
use tokio::sync::OnceCell;

const MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/surrealdb");

#[derive(Debug, Clone)]
pub struct Manager {
    config: Config,
    db: OnceCell<Surreal<Any>>,
}

impl Manager {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            db: OnceCell::new(),
        }
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
            .load_files(&MIGRATIONS_DIR)
            .up()
            .await
            .map_err(|e| Error::Migrations(e.to_string()))?;

        Ok(db)
    }

    pub async fn db(&self) -> Result<&Surreal<Any>> {
        self.db.get_or_try_init(async || self.connect().await).await
    }
}
