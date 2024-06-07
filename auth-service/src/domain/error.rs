pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    UnexpectedError,
    MissingToken,
    InvalidToken,
    Invalid2FACode,
    InvalidLoginAttamptId
}