use crate::{
    AppState, Config, Result, domain::Subscriber, email_client::EmailClient, model::ModelManager,
};
use axum::extract::Query;
use axum::{Form, extract::State};
use rand::Rng;
use rand::distr::Alphanumeric;
use reqwest::StatusCode;
use serde::Deserialize;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[axum::debug_handler(state = AppState)]
#[tracing::instrument(skip(mm, config, email_client))]
pub async fn subscribe(
    State(mm): State<Arc<ModelManager>>,
    State(config): State<Arc<Config>>,
    State(email_client): State<Arc<EmailClient>>,
    Form(form): Form<FormData>,
) -> Result<StatusCode> {
    let subscriber: Subscriber = form.try_into()?;

    let token = get_confirmation_token();

    mm.create_subscriber(&subscriber, &token).await?;

    send_confirmation_email(&email_client, &config, &subscriber, &token).await?;

    Ok(StatusCode::CREATED)
}

#[derive(Debug, Deserialize)]
pub struct Params {
    token: String,
}

#[axum::debug_handler(state = AppState)]
#[tracing::instrument(skip(mm))]
pub async fn confirm(
    State(mm): State<Arc<ModelManager>>,
    Query(params): Query<Params>,
) -> Result<StatusCode> {
    mm.confirm_subscriber(params.token.clone()).await?;

    Ok(StatusCode::OK)
}

fn get_confirmation_token() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(25)
        .collect()
}

async fn send_confirmation_email(
    email_client: &EmailClient,
    config: &Config,
    subscriber: &Subscriber,
    token: &str,
) -> Result<()> {
    let confirmation_link = get_confirmation_link(config, token)?;

    email_client
        .send_email(
            &subscriber.email,
            "Welcome!",
            &format!(
                "Welcome to our newsletter!<br />Click <a href=\"{}\">here</a> to confirm your subscription.",
                confirmation_link
            ),
            &format!(
                "Welcome to our newsletter!\nVisit {} to confirm your subscription.",
                confirmation_link
            ),
        )
        .await?;
    Ok(())
}

fn get_confirmation_link(config: &Config, token: &str) -> Result<Url> {
    let mut confirmation_link = config.base_url.join("subscriptions/confirm")?;
    confirmation_link.set_query(Some(&format!("token={token}")));

    Ok(confirmation_link)
}
