use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use crate::{app_state::AppState,
            domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode},
            utils::auth::generate_auth_cookie};

#[tracing::instrument(name = "verify_2fa", skip_all)]
pub async fn verify_2fa(State(state): State<AppState>,
                        jar: CookieJar,
                        Json(request): Json<Verify2FARequest>) -> 
    (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let email = if let Ok(oemail) = Email::parse(request.email.clone()) {
        oemail
    } else {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    };

    let login_attempt_id = match LoginAttemptId::parse(request.login_attempt_id) {
        Ok(laid) => laid,
        Err(_) => return (jar, Err(AuthAPIError::InvalidLoginAttamptId))
    };
    
    let two_fa_code = match TwoFACode::parse(request.two_fa_code) {
        Ok(scode) => scode,
        Err(_) => return (jar, Err(AuthAPIError::Invalid2FACode))
    };

    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    let result = two_fa_code_store.get_two_fa_code(&email).await;
    let (slaid, stfc) = match result {
        Ok((l, t)) => (l, t),
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials))
    };

    if slaid != login_attempt_id || stfc != two_fa_code {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    if two_fa_code_store
         .delete_two_fa_code(&email)
         .await
         .is_err() {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }
    let result = generate_auth_cookie(&email);
    let auth_cookie = match result {
        Ok(cookie) => cookie,
        Err(e) =>  return (jar, Err(AuthAPIError::UnexpectedError(e.into())))
    };

    let updated_jar = jar.add(auth_cookie);
    (updated_jar, Ok(()))
}

#[derive(Deserialize)]
pub struct Verify2FARequest {
    email: String,
    #[serde(rename = "loginAttemptId")]
    login_attempt_id: String,
    #[serde(rename = "2FACode")]
    two_fa_code: String
}