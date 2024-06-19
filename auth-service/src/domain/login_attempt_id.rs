use serde::{Deserialize, Serialize};
use color_eyre::eyre::{Context, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self> {
        let parsed_id =
            uuid::Uuid::parse_str(&id).wrap_err("Invalid login attempt id")?;
        Ok(Self(parsed_id.to_string()))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}
impl AsRef<str> for LoginAttemptId{
    fn as_ref(&self) -> &str {
        &self.0
    }
}
