use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use secrecy::{ExposeSecret, Secret};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password, LoginAttemptId, TwoFACode},
    utils::auth::generate_auth_cookie
};

#[tracing::instrument(name = "Login", skip_all)]
pub async fn login(State(state): State<AppState>,
                   jar: CookieJar,
                   Json(request): Json<LoginRequest>) -> 
                   (CookieJar, Result<impl IntoResponse, AuthAPIError>) {

    let email = if let Ok(oemail) = Email::parse(request.email.clone()) {
        oemail
    } else {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    };

    let password = if let Ok(opassword) = Password::parse(request.password.clone()) {
        opassword
    } else {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    };

    let user_store = state.user_store.read().await;

    if let Err(_) = user_store.validate_user(&email, &password).await {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return (jar, Err(AuthAPIError::IncorrectCredentials)),
    };

    // Handle request based on user's 2FA configuration
    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

#[tracing::instrument(name = "handle_2fa", skip_all)]
async fn handle_2fa(email: &Email, state: &AppState, jar: CookieJar) -> 
    (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    if let Err(e) = state.two_fa_code_store
        .write()
        .await
        .add_two_fa_code(email, login_attempt_id.to_owned(), two_fa_code.to_owned())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

    if let Err(e) = state.email_client
        .send_email(email, "2fa_code", two_fa_code.as_ref().expose_secret())
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(e)));
    }  

    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
            message: "2FA required".to_owned(),
            login_attempt_id: login_attempt_id.as_ref().expose_secret().to_owned() // This is the issue
    }));
    (jar, Ok((StatusCode::PARTIAL_CONTENT, response)))
}

#[tracing::instrument(name = "handle_no_2fa", skip_all)]
async fn handle_no_2fa(email: &Email, jar: CookieJar) -> 
    (CookieJar, Result<(StatusCode, Json<LoginResponse>), AuthAPIError>) {
    let result = generate_auth_cookie(email);
    let auth_cookie = match result {
        Ok(cookie) => cookie,
        Err(_) =>  return (jar, Err(AuthAPIError::InvalidCookie))
    };
    let response = Json(LoginResponse::RegularAuth);
    let updated_jar = jar.add(auth_cookie);
    (updated_jar, Ok((StatusCode::OK, response)))
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: Secret<String>,
    pub password: Secret<String>,
}
    
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}