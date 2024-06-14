use std::collections::HashSet;

use crate::domain::data_stores::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token);
        Ok(())
    }

    async fn is_banned_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token))
    }
}
