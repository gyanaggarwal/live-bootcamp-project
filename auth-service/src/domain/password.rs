use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, Secret};

#[derive(Debug, Clone)]
pub struct Password(Secret<String>);

impl Password {
    pub fn parse(s: Secret<String>) -> Result<Password> {
        if validate_password(&s) {
            Ok(Self(s))
        } else {
            Err(eyre!("failed to parse string into a password typr"))
        }
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

fn validate_password(s: &Secret<String>) -> bool {
    s.expose_secret().chars().count() >= 8
}

impl AsRef<Secret<String>> for Password {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}
