use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SurrealDb(#[from] surrealdb::Error),
    #[error("{0:?}")]
    Migrations(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Internal Server: - {self:?}");

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
