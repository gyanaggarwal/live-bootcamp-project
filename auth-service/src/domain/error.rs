use color_eyre::eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthAPIError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Incorrect credentials")]
    IncorrectCredentials,
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
   #[error("Invalid cookie")]
    InvalidCookie,
    #[error("Invalid2FACode")]
    Invalid2FACode,
    #[error("InvalidLoginAttamptId")]
    InvalidLoginAttamptId,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report)
}