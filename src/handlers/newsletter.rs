use crate::{Error, Result, email_client::EmailClient, model::ModelManager};
use axum::{Json, extract::State, http::HeaderMap, response::IntoResponse};
use base64::{Engine, prelude::BASE64_STANDARD};
use reqwest::{StatusCode, header::AUTHORIZATION};
use secrecy::SecretString;
use serde::Deserialize;
use std::sync::Arc;

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
    headers: HeaderMap,
    Json(body): Json<BodyData>,
) -> Result<impl IntoResponse> {
    let credentials = basic_authentication(headers).await?;
    _ = mm
        .validate_credientials(credentials)
        .await
        .map_err(|err| Error::Auth(err.to_string()))?;
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

pub struct Credentials {
    pub username: String,
    pub password: SecretString,
}

pub async fn basic_authentication(headers: HeaderMap) -> Result<Credentials> {
    let authorization_header = headers
        .get(AUTHORIZATION)
        .ok_or(Error::Auth("The `Authorization` header is messing!".into()))?
        .to_str()
        .map_err(|err| Error::Auth(err.to_string()))?;

    let base64encoded_segment = authorization_header
        .strip_prefix("Basic ")
        .ok_or(Error::Auth(
            "The Authrization schema is not `Basic` ?".into(),
        ))?;

    let decoded_bytes = BASE64_STANDARD
        .decode(base64encoded_segment)
        .map_err(|err| Error::Auth(err.to_string()))?;

    let decoded_credentials =
        String::from_utf8(decoded_bytes).map_err(|err| Error::Auth(err.to_string()))?;

    let mut credentials = decoded_credentials.splitn(2, ":");
    let username = credentials
        .next()
        .ok_or(Error::Auth(
            "A username must be provided in 'Basic' Auth.".into(),
        ))?
        .to_string();
    let password = credentials
        .next()
        .ok_or(Error::Auth(
            "A password must be provided in 'Basic' Auth.".into(),
        ))?
        .to_string();

    Ok(Credentials {
        username,
        password: SecretString::new(password.into()),
    })
}
