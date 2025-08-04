use crate::Config;
use surrealdb::{
    Surreal,
    engine::any::Any,
    opt::auth::{Database, Root},
};

#[derive(Debug, Clone)]
pub struct Manager {
    db: Surreal<Any>,
}

impl Manager {
    pub async fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let db = Surreal::<Any>::init();

        db.connect(config.url.as_str()).await?;

        if let Some(namespace) = &config.namespace
            && let Some(name) = &config.name
        {
            db.signin(Database {
                username: &config.username,
                password: &config.password,
                database: name,
                namespace,
            })
            .await?;
        } else {
            db.signin(Root {
                username: &config.username,
                password: &config.password,
            })
            .await?;
        }

        Ok(Self { db })
    }

    pub fn db(&self) -> &Surreal<Any> {
        &self.db
    }
}
