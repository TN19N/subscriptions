use crate::{Config, Result};
use axum::extract::FromRef;

#[derive(Debug, Clone)]
pub struct AppState {
    mm: model::Manager,
}

impl AppState {
    pub async fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            mm: model::Manager::new(config.database.clone()),
        })
    }
}

impl FromRef<AppState> for model::Manager {
    fn from_ref(input: &AppState) -> Self {
        input.mm.clone()
    }
}
