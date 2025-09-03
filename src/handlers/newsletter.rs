use std::sync::Arc;

use crate::{Result, email_client::EmailClient, model::ModelManager};
use axum::{Json, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BodyData {
    title: String,
    content: Content,
}

#[derive(Debug, Deserialize)]
struct Content {
    html: String,
    text: String,
}

#[tracing::instrument(skip(mm, email_client))]
pub async fn publish_newsletter(
    State(mm): State<Arc<ModelManager>>,
    State(email_client): State<Arc<EmailClient>>,
    Json(body): Json<BodyData>,
) -> Result<impl IntoResponse> {
    let subscribers = mm.get_confirmed_subscribers().await?;

    for subscriber in subscribers {
        email_client
            .send_email(
                &subscriber.email.try_into()?,
                &body.title,
                &body.content.html,
                &body.content.text,
            )
            .await?;
    }

    Ok(StatusCode::OK)
}
