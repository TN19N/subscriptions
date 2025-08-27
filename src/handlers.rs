use crate::model::ModelManager;
use crate::{Result, model::Subscription};
use axum::{Form, extract::State, http::StatusCode};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(skip(mm))]
pub async fn subscribe(
    State(mm): State<Arc<ModelManager>>,
    Form(form): Form<FormData>,
) -> Result<StatusCode> {
    let subscription = Subscription::new(form.name, form.email);
    mm.create_subscription(subscription).await?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(skip(mm))]
pub async fn health(State(mm): State<Arc<ModelManager>>) -> Result<StatusCode> {
    mm.db().await?.health().await?;

    Ok(StatusCode::OK)
}
