use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::constants::TTL_SECONDS_U64,
};

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}


#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn add_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let token_key = get_key(token.as_str());

        let _: () = self
                        .conn
                        .write()
                        .await
                        .set_ex(&token_key, 0, TTL_SECONDS_U64)
                        .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn is_banned_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let token_key = get_key(token);
        let is_banned: bool = self
                                .conn
                                .write()
                                .await
                                .exists(&token_key)
                                .map_err(|_| BannedTokenStoreError::UnexpectedError)?;
        Ok(is_banned)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}