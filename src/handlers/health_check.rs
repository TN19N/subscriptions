use crate::model::ModelManager;
use crate::{AppState, Result};
use axum::extract::State;
use reqwest::StatusCode;
use std::sync::Arc;

#[axum::debug_handler(state = AppState)]
#[tracing::instrument(skip(mm))]
pub async fn health(State(mm): State<Arc<ModelManager>>) -> Result<StatusCode> {
    mm.db().await?.health().await.map_err(Box::new)?;

    Ok(StatusCode::OK)
}
