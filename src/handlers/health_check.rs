use crate::Result;
use crate::model::ModelManager;
use axum::extract::State;
use reqwest::StatusCode;
use std::sync::Arc;

#[tracing::instrument(skip(mm))]
pub async fn health(State(mm): State<Arc<ModelManager>>) -> Result<StatusCode> {
    mm.db().await?.health().await?;

    Ok(StatusCode::OK)
}
