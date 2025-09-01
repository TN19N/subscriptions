use crate::{
    Config, Result,
    email_client::EmailClient,
    model::{self, ModelManager},
};
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub mm: Arc<ModelManager>,
    pub email_client: Arc<EmailClient>,
}

impl AppState {
    pub async fn new(config: Config) -> Result<Self> {
        Ok(Self {
            mm: Arc::new(model::ModelManager::new(config.database.clone())),
            email_client: Arc::new(EmailClient::new(config.email_client.clone())?),
            config: Arc::new(config),
        })
    }
}

impl FromRef<AppState> for Arc<model::ModelManager> {
    fn from_ref(input: &AppState) -> Self {
        input.mm.clone()
    }
}

impl FromRef<AppState> for Arc<EmailClient> {
    fn from_ref(input: &AppState) -> Self {
        input.email_client.clone()
    }
}

impl FromRef<AppState> for Arc<Config> {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}
