use color_eyre::eyre::Result;
use secrecy::ExposeSecret;

use crate::domain::{Email, EmailClient};

#[derive(Default)]
pub struct MockEmailClient;

#[async_trait::async_trait]
impl EmailClient for MockEmailClient {
    async fn send_email (
        &self,
        recipient: &Email,
        subject: &str,
        content: &str
    ) -> Result<()> {
        println!("Sending email to {} with subject: {} and content: {}",
                 recipient.as_ref().expose_secret(), subject, content);
        Ok(())
    }
}
