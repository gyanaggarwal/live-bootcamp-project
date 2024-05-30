use axum::{http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

use crate::{
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let ocookie = jar.get(JWT_COOKIE_NAME);
    let cookie = match ocookie {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value().to_owned();

    let result = validate_token(&token).await;
    if result.is_err() {
        return (jar, Err(AuthAPIError::InvalidToken))
    }
    
    let updated_jar = jar.remove(JWT_COOKIE_NAME);
    (updated_jar, Ok(StatusCode::OK))
}

