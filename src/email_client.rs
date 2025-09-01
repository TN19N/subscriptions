use crate::{config::EmailClientConfig, domain::SubscriberEmail};
use axum::http::HeaderName;
use reqwest::Client;
use secrecy::ExposeSecret;
use serde::Serialize;

const EMAIL_CLIENT_AUTH_HEADER: HeaderName = HeaderName::from_static("x-postmark-server-token");

#[derive(Debug)]
pub struct EmailClient {
    http_client: Client,
    config: EmailClientConfig,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

impl EmailClient {
    pub fn new(config: EmailClientConfig) -> Result<Self, reqwest::Error> {
        let http_client = Client::builder().timeout(config.timeout).build()?;
        Ok(Self {
            http_client,
            config,
        })
    }

    pub async fn send_email(
        &self,
        recipeint: &SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let request_body = SendEmailRequest {
            from: self.config.sender_email.as_ref(),
            to: recipeint.as_ref(),
            subject,
            html_body: html_content,
            text_body: text_content,
        };

        self.http_client
            .post(self.config.base_url.as_str())
            .header(
                EMAIL_CLIENT_AUTH_HEADER,
                self.config.auth_token.expose_secret(),
            )
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::EmailClientConfig;
    use crate::domain::SubscriberEmail;
    use claims::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use reqwest::Method;
    use reqwest::header::CONTENT_TYPE;
    use secrecy::SecretString;
    use std::str::FromStr;
    use std::time::Duration;
    use url::Url;
    use wiremock::matchers::{any, header, header_exists, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    struct SendEmailBodyMatcher;

    impl wiremock::Match for SendEmailBodyMatcher {
        fn matches(&self, request: &wiremock::Request) -> bool {
            let result: std::result::Result<serde_json::Value, _> =
                serde_json::from_slice(&request.body);

            if let Ok(body) = result {
                // Check that all the mandatory fields are populated
                // without inspecting the field values
                body.get("From").is_some()
                    && body.get("To").is_some()
                    && body.get("Subject").is_some()
                    && body.get("HtmlBody").is_some()
                    && body.get("TextBody").is_some()
            } else {
                false
            }
        }
    }

    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(&mock_server.uri());

        Mock::given(header_exists(EMAIL_CLIENT_AUTH_HEADER))
            .and(header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref()))
            .and(method(Method::POST))
            .and(SendEmailBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        _ = email_client
            .send_email(&email(), &subject(), &content(), &content())
            .await;

        // Assert
        // Mock expectations are checked on drop
    }

    #[tokio::test]
    async fn send_email_succeeds_if_the_server_returns_200() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(&mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(&email(), &subject(), &content(), &content())
            .await;

        // Assert
        assert_ok!(outcome);
    }

    #[tokio::test]
    async fn send_email_fails_if_the_server_returns_500() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(&mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(500))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(&email(), &subject(), &content(), &content())
            .await;

        // Assert
        assert_err!(outcome);
    }

    #[tokio::test]
    async fn send_email_timeout_if_the_server_takes_to_long() {
        // Arrange
        let mock_server = MockServer::start().await;
        let email_client = email_client(&mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_secs(60 * 3)))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let outcome = email_client
            .send_email(&email(), &subject(), &content(), &content())
            .await;

        // Assert
        assert_err!(outcome);
    }

    /// Generate a random email subject
    fn subject() -> String {
        Sentence(1..2).fake()
    }

    /// Generate a random email content
    fn content() -> String {
        Paragraph(1..10).fake()
    }

    /// Generate a random subscriber email
    fn email() -> SubscriberEmail {
        SubscriberEmail::try_from(SafeEmail().fake::<String>())
            .expect("Expect to get valid subscriber email!")
    }

    /// Get a test instance of `EmailClient`.
    fn email_client(base_url: &str) -> EmailClient {
        let config = EmailClientConfig {
            sender_email: email(),
            base_url: Url::from_str(base_url).expect("Expect to get valid email client base url"),
            auth_token: SecretString::new(Faker.fake::<String>().into()),
            timeout: Duration::from_millis(200),
        };
        EmailClient::new(config).expect("Expect email client to be initialized.")
    }
}
