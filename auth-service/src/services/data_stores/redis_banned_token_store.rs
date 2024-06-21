use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;
use color_eyre::eyre::Context;
use secrecy::{Secret, ExposeSecret};

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::constants::TTL_SECONDS_I64,
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
    #[tracing::instrument(name = "add_banned_token", skip_all)]
    async fn add_banned_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError> {
        let token_key = get_key(token.expose_secret());
        let ttl: u64 = TTL_SECONDS_I64
        .try_into()
        .wrap_err("failed to cast TOKEN_TTL_SECONDS to u64") // New!
        .map_err(BannedTokenStoreError::UnexpectedError)?; // Updated!

    let _: () = self
        .conn
        .write()
        .await
        .set_ex(&token_key, 0, ttl)
        .wrap_err("failed to set banned token in Redis") // New!
        .map_err(BannedTokenStoreError::UnexpectedError)?;
        Ok(())
    }

    #[tracing::instrument(name = "is_banned_token", skip_all)]
    async fn is_banned_token(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError> {
        let token_key = get_key(token.expose_secret());
        let is_banned: bool = self
            .conn
            .write()
            .await
            .exists(&token_key)
            .wrap_err("failed to check if token exists in Redis") // New!
            .map_err(BannedTokenStoreError::UnexpectedError)?; // Updated!

        Ok(is_banned)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}