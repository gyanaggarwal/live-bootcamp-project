use color_eyre::eyre::{eyre, Result};

#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Password> {
        if validate_password(&s) {
            Ok(Self(s))
        } else {
            Err(eyre!("failed to parse string into a password typr"))
        }
    }
}

fn validate_password(s: &str) -> bool {
    s.chars().count() >= 8
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
