use crate::{
    Result,
    config::Config,
    model::{self, ModelManager},
};
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub mm: Arc<ModelManager>,
}

impl AppState {
    pub async fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            mm: Arc::new(model::ModelManager::new(config.database.clone())),
        })
    }
}

impl FromRef<AppState> for Arc<model::ModelManager> {
    fn from_ref(input: &AppState) -> Self {
        input.mm.clone()
    }
}
