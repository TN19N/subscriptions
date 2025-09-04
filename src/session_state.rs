use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use surrealdb::RecordId;
use tower_sessions::Session;

use crate::Result;

pub struct TypedSession(Session);

impl TypedSession {
    const USER_ID_KEY: &str = "user_id";

    pub async fn renew(&self) -> Result<()> {
        Ok(self.0.cycle_id().await?)
    }

    pub async fn insert_user_id(&self, user_id: RecordId) -> Result<()> {
        Ok(self.0.insert(Self::USER_ID_KEY, user_id).await?)
    }

    pub async fn get_user_id(&self) -> Result<Option<RecordId>> {
        Ok(self.0.get(Self::USER_ID_KEY).await?)
    }
}

impl<S> FromRequestParts<S> for TypedSession
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|err| {
                tracing::error!("Something went wrong!: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to extract session",
                )
                    .into_response()
            })?;

        Ok(Self(session))
    }
}
