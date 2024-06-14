use std::sync::Arc;

use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, Password, User,
};

use super::postgres_user_store::{compute_password_hash, verify_password_hash};

pub struct RedisUserStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisUserStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

const USER_PREFIX: &str = "user:";

fn get_key(email: &Email) -> String {
    format!("{}{}", USER_PREFIX, email.as_ref())
}

#[async_trait::async_trait]
impl UserStore for RedisUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        todo!()
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        todo!()
    }

    async fn validate_user(&self, email: &Email, password: &Password) -> 
        Result<(), UserStoreError> {
        todo!()
    }
}    

