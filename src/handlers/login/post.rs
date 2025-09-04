use crate::{
    Error, Result, handlers::Credentials, model::ModelManager, session_state::TypedSession,
};
use axum::{
    Form,
    extract::State,
    response::{IntoResponse, Redirect},
};
use axum_messages::Messages;
use secrecy::SecretString;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct FormData {
    username: String,
    password: SecretString,
}

pub async fn login(
    State(mm): State<Arc<ModelManager>>,
    messages: Messages,
    session: TypedSession,
    Form(form): Form<FormData>,
) -> Result<impl IntoResponse> {
    let credentials = Credentials {
        username: form.username,
        password: form.password,
    };

    let result = mm
        .validate_credientials(credentials)
        .await
        .map_err(|err| Error::Auth(err.to_string()));

    match result {
        Ok(record_id) => {
            session.renew().await?;
            session.insert_user_id(record_id).await?;
            Ok(Redirect::to("/admin/dashboard"))
        }
        Err(err) => {
            tracing::warn!("Login failed! because: {err:?}");
            messages.warning("Authentication Failed");
            Ok(Redirect::to("/login"))
        }
    }
}
