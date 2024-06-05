use core::fmt;

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

impl fmt::Display for LoginAttemptId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}