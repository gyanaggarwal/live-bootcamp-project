use std::hash::Hash;
use validator::validate_email;
use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, Secret};

#[derive(Debug, Clone)]
pub struct Email(Secret<String>);

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

impl Eq for Email {}

impl Email {
    pub fn parse(s: Secret<String>) -> Result<Email> {
        if validate_email(s.expose_secret()) {
            Ok(Self(s))
        } else {
            Err(eyre!(format!("{} is not a valid email.", s.expose_secret())))
        }
    }
}

impl AsRef<Secret<String>> for Email {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}