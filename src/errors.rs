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
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::ValidationError(_) => {
                tracing::info!("Bad request: - {self:?}");
                StatusCode::BAD_REQUEST.into_response()
            }
            _ => {
                tracing::error!("Internal Server: - {self:?}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
