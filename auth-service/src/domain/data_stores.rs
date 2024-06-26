use color_eyre::eyre::Report;
use thiserror::Error;
use secrecy::Secret;

use super::{Email, Password, User, LoginAttemptId, TwoFACode};

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_banned_token(&mut self, token: Secret<String>) -> Result<(), BannedTokenStoreError>;
    async fn is_banned_token(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError>;
}

#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_two_fa_code(&mut self, 
                             email: &Email, 
                             login_attempt_id: LoginAttemptId, 
                             two_fa_code: TwoFACode) -> Result<(), TwoFACodeStoreError>;
    async fn get_two_fa_code(&self, email: &Email) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
    async fn delete_two_fa_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
}

#[derive(Debug, Error)]
pub enum UserStoreError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl PartialEq for UserStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::UserAlreadyExists, Self::UserAlreadyExists)
                | (Self::UserNotFound, Self::UserNotFound)
                | (Self::InvalidCredentials, Self::InvalidCredentials)
                | (Self::UnexpectedError(_), Self::UnexpectedError(_))
        )
    }
}

#[derive(Debug, Error)]
pub enum BannedTokenStoreError {
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

#[derive(Debug, Error)]
pub enum TwoFACodeStoreError {
    #[error("LoginAttemptNotFound")]
    LoginAttemptIdNotFound,
    #[error("Unexpected error")]
    UnexpectedError,
}

impl PartialEq for TwoFACodeStoreError {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::LoginAttemptIdNotFound, Self::LoginAttemptIdNotFound)
                | (Self::UnexpectedError, Self::UnexpectedError)
        )
    }
}
