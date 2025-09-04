use crate::Result;
use axum::response::{Html, IntoResponse};
use reqwest::StatusCode;

pub async fn home() -> Result<impl IntoResponse> {
    Ok((StatusCode::OK, Html(include_str!("home.html"))))
}
