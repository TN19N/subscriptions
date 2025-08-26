use crate::Result;
use crate::model::ModelManager;
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
    mm.db()
        .await?
        .query("CREATE subscriptions SET name = $name, email = $email")
        .bind(("name", form.name))
        .bind(("email", form.email))
        .await?
        .check()?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(skip(mm))]
pub async fn health(State(mm): State<Arc<ModelManager>>) -> Result<StatusCode> {
    mm.db().await?.health().await?;

    Ok(StatusCode::OK)
}
