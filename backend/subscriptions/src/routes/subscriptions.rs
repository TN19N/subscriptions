use axum::{Form, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use surrealdb::RecordId;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    State(mm): State<model::Manager>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    #[derive(Deserialize)]
    struct QueryResult {
        id: RecordId,
    }

    let result: Result<Option<QueryResult>, surrealdb::Error> = mm
        .db()
        .create("subscriptions")
        .content(json!({
            "email": form.email,
            "name": form.name,
        }))
        .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(error) => {
            dbg!(error);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
