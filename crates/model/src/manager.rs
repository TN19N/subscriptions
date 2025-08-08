use crate::{Config, Result};
use secrecy::ExposeSecret;
use surrealdb::{
    Surreal,
    engine::any::Any,
    opt::auth::{Database, Namespace, Root},
};
use tokio::sync::OnceCell;

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

    // #[tracing::instrument(skip_all)]
    async fn connect(&self) -> Result<Surreal<Any>> {
        let config = &self.config;
        let db = Surreal::<Any>::init();

        tracing::info!("Connecting to database: {}", config.url.as_str());
        db.connect(config.url.as_str()).await?;

        if let Some(namespace) = &config.namespace {
            if let Some(name) = &config.name {
                db.signin(Database {
                    username: &config.username,
                    password: config.password.expose_secret(),
                    database: name,
                    namespace,
                })
                .await?;
            } else {
                db.signin(Namespace {
                    username: &config.username,
                    password: config.password.expose_secret(),
                    namespace,
                })
                .await?;
            }
        } else {
            db.signin(Root {
                username: &config.username,
                password: config.password.expose_secret(),
            })
            .await?;
        };

        Ok(db)
    }

    pub async fn db(&self) -> Result<&Surreal<Any>> {
        self.db.get_or_try_init(async || self.connect().await).await
    }
}
