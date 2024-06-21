use color_eyre::eyre::Result; // For improved error handling and reporting
use reqwest::{Client, Url}; // For making HTTP requests
use secrecy::{ExposeSecret, Secret}; // For securely handling sensitive data

use crate::domain::{Email, EmailClient}; // Import domain-specific modules

// Define the PostmarkEmailClient struct
pub struct PostmarkEmailClient {
    http_client: Client, // HTTP client for making requests
    base_url: String, // Base URL for the email service
    sender: Email, // Email address of the sender
    authorization_token: Secret<String>, // Authorization token for the email service, wrapped in Secret for security
}

impl PostmarkEmailClient {
    // Constructor for creating a new PostmarkEmailClient instance
    pub fn new(
        base_url: String,
        sender: Email,
        authorization_token: Secret<String>,
        http_client: Client,
    ) -> Self {
        Self {
            http_client,
            base_url,
            sender,
            authorization_token,
        }
    }
}

#[async_trait::async_trait]
impl EmailClient for PostmarkEmailClient {
    #[tracing::instrument(name = "Sending email", skip_all)] // Trace this function, skipping logging its parameters
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()> {
        // Parse the base URL and join it with the email endpoint
        let base = Url::parse(&self.base_url)?;
        let url = base.join("/email")?;

        // Create the request body for sending the email
        let request_body = SendEmailRequest {
            from: self.sender.as_ref().expose_secret(),
            to: recipient.as_ref().expose_secret(),
            subject,
            html_body: content,
            text_body: content,
            message_stream: MESSAGE_STREAM,
        };

        // Build the HTTP POST request
        let request = self
            .http_client
            .post(url)
            .header(
                POSTMARK_AUTH_HEADER,
                self.authorization_token.expose_secret(), // Securely expose the authorization token
            )
            .json(&request_body);

        // Send the request and handle the response
        request.send().await?.error_for_status()?;

        Ok(())
    }
}

// Constants for message stream and authorization header
const MESSAGE_STREAM: &str = "outbound";
const POSTMARK_AUTH_HEADER: &str = "X-Postmark-Server-Token";

// Define the structure of the email request body
// For more information about the request structure, see the API docs: https://postmarkapp.com/developer/user-guide/send-email-with-api
#[derive(serde::Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
    message_stream: &'a str,
}
