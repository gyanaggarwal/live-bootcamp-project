use rand::prelude::*;
use color_eyre::eyre::{eyre, Context, Result};
use secrecy::{Secret, ExposeSecret};

const LOW_RANGE_VALUE:u32 = 100000;
const HIGH_RANGE_VALUE:u32 = 999999;

#[derive(Debug, Clone)]
pub struct TwoFACode(Secret<String>);

impl TwoFACode {
    pub fn parse(code: Secret<String>) -> Result<Self> {
        let code_as_u32 = code.expose_secret()
                                    .parse::<u32>()
                                    .wrap_err("Invalid 2FA code")?;
        if (LOW_RANGE_VALUE..=HIGH_RANGE_VALUE).contains(&code_as_u32) {
            Ok(Self(code))
        } else {
            Err(eyre!("Invalid 2FA code"))
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        Self(Secret::new(rand::thread_rng().gen_range(LOW_RANGE_VALUE..=HIGH_RANGE_VALUE).to_string()))
    }
}

impl PartialEq for TwoFACode {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for TwoFACode {}

impl AsRef<Secret<String>> for TwoFACode {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}
