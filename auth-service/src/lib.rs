use axum::{routing::post, 
    serve::Serve, 
    Router, 
    Json,
    http::StatusCode,
    response::{Response, IntoResponse}
};
use tower_http::services::ServeDir;
use std::error::Error;
use serde::{Deserialize, Serialize};

pub mod routes;
pub mod services;
pub mod domain;
pub mod app_state;

use crate::routes::signup;
use crate::routes::logout;
use crate::routes::verify_2fa;
use crate::routes::verify_token;
use crate::routes::login;

use app_state::AppState;

use domain::error::AuthAPIError;

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        //let router= todo!();
        let router = Router::new()
                             .nest_service("/", ServeDir::new("assets"))
                             .route("/login", post(login))
                             .route("/logout", post(logout))
                             .route("/verify-2fa", post(verify_2fa))
                             .route("/verify-token", post(verify_token))
                             .route("/signup", post(signup))
                             .with_state(app_state);


        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        //todo!()
        let app = Application {
            server,
            address,
        };

        Ok(app)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
        };
        let body = Json(ErrorResponse{
            error: error_message.to_string()  
        });
        (status, body).into_response()
    }
}
