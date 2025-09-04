use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SurrealDb(#[from] Box<surrealdb::Error>),
    #[error("{0:?}")]
    Migrations(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Config(#[from] config::ConfigError),
    #[error(transparent)]
    UrlParse(#[from] url::ParseError),
    #[error(transparent)]
    ValidationErrors(#[from] validator::ValidationErrors),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationError),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Session(#[from] tower_sessions::session::Error),
    #[error("{0:?}")]
    Auth(String),

    #[error("{0:?}")]
    Custom(String),
}

impl From<surrealdb::Error> for Error {
    fn from(value: surrealdb::Error) -> Self {
        Self::SurrealDb(Box::new(value))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::ValidationErrors(_) | Self::ValidationError(_) => {
                tracing::warn!("Bad request: - {self:?}");
                StatusCode::BAD_REQUEST.into_response()
            }
            Self::Auth(_) => {
                tracing::warn!("Unauthorized : - {self:?}");
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("WWW-Authenticate", r#"Basic realm="publish""#)
                    .body(Body::empty())
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR.into_response())
            }
            _ => {
                tracing::error!("Internal Server: - {self:?}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
