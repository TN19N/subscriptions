use std::sync::Arc;

use crate::{Config, Result};
use axum::extract::FromRef;

#[derive(Debug, Clone)]
pub struct AppState {
    pub mm: Arc<model::Manager>,
}

impl AppState {
    pub async fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            mm: Arc::new(model::Manager::new(config.database.clone())),
        })
    }
}

impl FromRef<AppState> for Arc<model::Manager> {
    fn from_ref(input: &AppState) -> Self {
        input.mm.clone()
    }
}
