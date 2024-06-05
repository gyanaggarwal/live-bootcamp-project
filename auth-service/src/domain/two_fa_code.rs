use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self(rng.gen_range(100000..=999999).to_string())
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
