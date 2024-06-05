use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn new() -> Self {
        Self( Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for LoginAttemptId{
    fn as_ref(&self) -> &str {
        &self.0
    }
}
