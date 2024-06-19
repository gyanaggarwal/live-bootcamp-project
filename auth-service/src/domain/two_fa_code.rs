use rand::prelude::*;
use serde::{Deserialize, Serialize};
use color_eyre::eyre::{eyre, Context, Result};

const LOW_RANGE_VALUE:u32 = 100000;
const HIGH_RANGE_VALUE:u32 = 999999;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self> {

        let code_as_u32 = code
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
        Self(rand::thread_rng().gen_range(LOW_RANGE_VALUE..=HIGH_RANGE_VALUE).to_string())
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
