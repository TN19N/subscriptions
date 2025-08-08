use crate::Result;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn health(State(mm): State<model::Manager>) -> Result<StatusCode> {
    mm.db()
        .await
        .map_err(Box::new)?
        .health()
        .await
        .map_err(Box::new)?;

    Ok(StatusCode::OK)
}
