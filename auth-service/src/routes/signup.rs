use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use crate::{app_state::AppState, domain::user::User};

pub async fn signup(State(state): State<AppState>, 
                    Json(request): Json<SignupRequest>) -> impl IntoResponse {
// Create a new `User` instance using data in the `request`
    let user = request.create_user();

    let mut user_store = state.user_store.write().await;

// TODO: Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    user_store.add_user(user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response)
    //    StatusCode::OK.into_response()
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