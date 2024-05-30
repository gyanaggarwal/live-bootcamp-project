use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::domain::AuthAPIError;
use crate::routes::RouteResponse;
use crate::utils::auth::validate_token;

pub async fn verify_token(Json(request): Json<VerifyTokenRequest>) -> 
    Result<impl IntoResponse, AuthAPIError> {
    let _ = validate_token(&request.token)
                .await
                .map_err(|_| AuthAPIError::InvalidToken);

    let response = Json(RouteResponse {
        message: "Token has been validated!".to_string(),
    });
                                                
    Ok((StatusCode::OK, response))
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String
}