use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
    utils::auth::generate_auth_cookie
};

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

    let user_store = state.user_store.write().await;

    if let Err(_) = user_store.validate_user(&email, &password).await {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    if let Err(_) = user_store.get_user(&email).await {
        return (jar, Err(AuthAPIError::IncorrectCredentials));
    }

    let result = generate_auth_cookie(&email);
    match result {
        Ok(cookie) => {let updated_jar = jar.add(cookie);
                                        (updated_jar, Ok(StatusCode::OK.into_response()))},

        Err(_) =>  (jar, Err(AuthAPIError::UnexpectedError))
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
    