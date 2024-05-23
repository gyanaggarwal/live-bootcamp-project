use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, 
            domain::{error::AuthAPIError, user::User},
            services::hashmap_user_store::UserStoreError};

pub async fn signup(State(state): State<AppState>, 
                    Json(request): Json<SignupRequest>) -> 
                    Result<impl IntoResponse, AuthAPIError> {

    let email = &request.email;
    let password = &request.password; 
    println!("email: {} password: {}", email, password);

    if !valid_credentials(email, password) {
        return Err(AuthAPIError::InvalidCredentials);
    }   

    let user = request.create_user();

    let mut user_store = state.user_store.write().await;

    let result = user_store.add_user(user);
    match result {
        Ok(_) => Ok((StatusCode::CREATED, Json(SignupResponse {
            message: "User created successfully!".to_string(),
        }))),
        Err(UserStoreError::UserAlreadyExists) => Err(AuthAPIError::UserAlreadyExists),
        Err(UserStoreError::UnexpectedError) => Err(AuthAPIError::UnexpectedError),
        Err(UserStoreError::InvalidCredentials) => Err(AuthAPIError::InvalidCredentials),
        Err(UserStoreError::UserNotFound) => Err(AuthAPIError::UnexpectedError)
     }

}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl SignupRequest {
    pub fn create_user(self) -> User {
        User::new(self.email, self.password, self.requires_2fa)
    }
}


#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}

fn valid_password(password: &str, len: usize) -> bool {
    password.chars().count() >= len
}

fn valid_email(email: &str, ch: char) -> bool {
    !email.is_empty() && {
        let fvalue = email.find(ch);
        match fvalue {
            None => false,
            Some(0) => false,
            Some(_) => true,
        }
    }
}

fn valid_credentials(email: &str, password: &str) -> bool {
    valid_email(email, '@') && valid_password(password, 8)
}