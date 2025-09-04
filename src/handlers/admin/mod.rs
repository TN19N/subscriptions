use crate::{Result, model::ModelManager, session_state::TypedSession};
use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use reqwest::StatusCode;
use std::sync::Arc;

pub async fn admin_dashboard(
    State(mm): State<Arc<ModelManager>>,
    session: TypedSession,
) -> Result<impl IntoResponse> {
    let username = match session.get_user_id().await {
        Ok(Some(user_id)) => mm.get_username(user_id).await?,
        reason => {
            tracing::error!("Failed to authenticate: {reason:?}");
            return Ok(Redirect::to("/login").into_response());
        }
    };

    let body = format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <meta http-equiv="content-type" content="text/html; charset=utf-8">
                <title>Admin dashboard</title>
            </head>
            <body>
                <p>Welcome {username}</p>
            </body>
        </html>
        "#
    );

    Ok((StatusCode::OK, Html(body)).into_response())
}
