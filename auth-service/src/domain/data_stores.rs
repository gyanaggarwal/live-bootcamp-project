use super::{Email, Password, User, LoginAttemptId, TwoFACode};

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password) -> Result<(), UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn is_banned_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
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

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Debug)]
pub enum BannedTokenStoreError {
    UnexpectedError
}

#[derive(Debug)]
pub enum TwoFACodeStoreError {
    LoginAttamptIdNotFound,
    UnexpectedError
}
