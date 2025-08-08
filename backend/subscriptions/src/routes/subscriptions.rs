use crate::Result;
use axum::{Form, extract::State, http::StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(skip(mm))]
pub async fn subscribe(
    State(mm): State<model::Manager>,
    Form(form): Form<FormData>,
) -> Result<StatusCode> {
    mm.db()
        .await
        .map_err(Box::new)?
        .query("CREATE subscriptions SET name = $name, email = $email")
        .bind(("name", form.name))
        .bind(("email", form.email))
        .await
        .map_err(Box::new)?
        .check()
        .map_err(Box::new)?;

    Ok(StatusCode::CREATED)
}
