use crate::Result;
use axum::response::{Html, IntoResponse};
use axum_messages::Messages;
use reqwest::StatusCode;

pub async fn login(
    messages: Messages,
    // Query(query): Option<Query<QueryParams>>,
) -> Result<impl IntoResponse> {
    let error_message = messages
        .into_iter()
        .map(|message| format!("<p><i>{}</i></p>", message.message))
        .collect::<Vec<_>>()
        .join("");

    let body = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta http-equiv="content-type" content="text/html; charset=utf-8">
            <title>Login</title>
        </head>
        <body>
            {error_message}
            <form action="/login" method="post">
                <label>Username
                    <input
                        type="text"
                        placeholder="Enter Username"
                        name="username"
                    >
                </label>
                <label>Password
                    <input
                        type="password"
                        placeholder="Enter Password"
                        name="password"
                    >
                </label>
                <button type="submit">Login</button>
            </form>
        </body>
        </html>
        "#
    );

    Ok((StatusCode::OK, Html(body)))
}
