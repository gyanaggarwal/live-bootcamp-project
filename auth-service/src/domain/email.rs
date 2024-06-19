use validator::validate_email;
use color_eyre::eyre::{eyre, Result};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(s: String) -> Result<Email> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(eyre!("Invalid email"))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}