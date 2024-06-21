use std::sync::Arc;

use redis::{Commands, Connection};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use color_eyre::eyre::Context;
use secrecy::Secret;

use crate::domain::{Email, LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use crate::utils::constants::TTL_SECONDS_U64;

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}


#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref().expose_secret())
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    #[tracing::instrument(name = "add_two_fa_code", skip_all)]
    async fn add_two_fa_code(&mut self, email: &Email, login_attempt_id: LoginAttemptId, code: TwoFACode) -> 
        Result<(), TwoFACodeStoreError> {
        let key = get_key(&email);

        let data = TwoFATuple(
            login_attempt_id.as_ref().expose_secret().to_owned(),
            code.as_ref().expose_secret().to_owned(),
        );
        let serialized_data = serde_json::to_string(&data)
                .wrap_err("failed to serialize 2FA tuple")
                .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
    
        let _: () = self
                .conn
                .write()
                .await
                .set_ex(&key, serialized_data, TTL_SECONDS_U64)
                .wrap_err("failed to set 2FA code in redis")
                .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
    
            Ok(())
    }

    #[tracing::instrument(name = "delete_two_fa_code", skip_all)]
    async fn delete_two_fa_code(&mut self, email: &Email) -> 
        Result<(), TwoFACodeStoreError> {
        let key = get_key(email);
        let _: () = self
                        .conn
                        .write()
                        .await
                        .del(&key)
                        .wrap_err("failed to delete 2FA code from Redis")
                        .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        Ok(())
    }

    #[tracing::instrument(name = "get_two_fa_code", skip_all)]
    async fn get_two_fa_code(&self, email: &Email) -> 
        Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
            let key = get_key(email);

            match self.conn.write().await.get::<_, String>(&key) {
                Ok(value) => {
                    let data: TwoFATuple = serde_json::from_str(&value)
                        .wrap_err("failed to deserialize 2FA tuple")
                        .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
    
                    let login_attempt_id = LoginAttemptId::parse(Secret::new(data.0))
                        .wrap_err("failed to parse login_attempt_id")
                        .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
    
                    let email_code = TwoFACode::parse(Secret::new(data.1))
                        .wrap_err("failed to parse email_code")
                        .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;
    
                    Ok((login_attempt_id, email_code))
                }
                Err(_) => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
            }
    }
}