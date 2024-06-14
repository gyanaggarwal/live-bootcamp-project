use axum_extra::extract::cookie::{Cookie, SameSite};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{app_state::BannedTokenStoreType, domain::email::Email};

use super::constants::{JWT_COOKIE_NAME, TTL_SECONDS_I64, JWT_SECRET};
// This is definitely NOT a good secret. We will update it soon!
// const JWT_SECRET: &str = "secret";

#[derive(Debug)]
pub enum GenerateTokenError {
    TokenError(jsonwebtoken::errors::Error),
    UnexpectedError
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_auth_cookie(email: &Email) -> Result<Cookie<'static>, GenerateTokenError> {
    let token = generate_auth_token(email)?;
    Ok(create_auth_cookie(token))
}

fn create_auth_cookie(token: String) -> Cookie<'static> {
    let cookie = Cookie::build((JWT_COOKIE_NAME, token))
        .path("/") // apple cookie to all URLs on the server
        .http_only(true) // prevent JavaScript from accessing the cookie
        .same_site(SameSite::Lax) // send cookie with "same-site" requests, and with "cross-site" top-level navigations.
        .build();

    cookie
}

fn generate_auth_token(email: &Email) -> Result<String, GenerateTokenError> {
    let delta = chrono::Duration::try_seconds(TTL_SECONDS_I64)
        .ok_or(GenerateTokenError::UnexpectedError)?;

    // Create JWT expiration time
    let exp = Utc::now()
        .checked_add_signed(delta)
        .ok_or(GenerateTokenError::UnexpectedError)?
        .timestamp();

    // Cast exp to a usize, which is what Claims expects
    let exp: usize = exp
        .try_into()
        .map_err(|_| GenerateTokenError::UnexpectedError)?;

    let sub = email.as_ref().to_owned();

    let claims = Claims { sub, exp };

    create_token(&claims).map_err(GenerateTokenError::TokenError)
}

fn create_token(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

pub async fn validate_token(
    token: &str,
    banned_token_store: BannedTokenStoreType,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    match banned_token_store.read().await.is_banned_token(token).await {
        Ok(value) => {
            if value {
                return Err(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::InvalidToken,
                ));
            }
        }
        Err(_) => {
            return Err(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            ));
        }
    }

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
