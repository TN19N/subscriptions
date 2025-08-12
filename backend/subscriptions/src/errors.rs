use axum::http::StatusCode;
use axum::response::IntoResponse;
use config::ConfigError;
use thiserror::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Model(#[from] Box<model::Error>),
    #[error(transparent)]
    SurrealDb(#[from] Box<surrealdb::Error>),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Config(#[from] Box<ConfigError>),
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),

    #[error("{0:?}")]
    Custom(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Internal Server - Error: {self:?}");

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
