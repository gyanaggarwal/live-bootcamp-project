use std::error::Error;
use serde::{Deserialize, Serialize};
use tower_http::{services::ServeDir, cors::CorsLayer, trace::TraceLayer};
use http::Method;
use utils::tracing::{make_span_with_request_id, on_request, on_response};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, 
    Router
};
use app_state::AppState;
use domain::AuthAPIError;
use routes::{signup, login, verify_token, logout, verify_2fa};

use sqlx::{PgPool, postgres::PgPoolOptions};
use redis::{Client, RedisResult};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

pub struct Application {
    pub server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = ["http://localhost:8000".parse()?];
  
        let cors = CorsLayer::new()
                                        // Allow GET and POST requests
                                        .allow_methods([Method::GET, Method::POST])
                                        // Allow cookies to be included in requests
                                        .allow_credentials(true)
                                        .allow_origin(allowed_origins);

        let trac = TraceLayer::new_for_http()
                                                        .make_span_with(make_span_with_request_id)
                                                        .on_request(on_request)
                                                        .on_response(on_response);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors)
            .layer(trac);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        let app = Application {
            server,
            address,
        };

        Ok(app)
      }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        log_error_chain(&self); // New!
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Incorrect credentials"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid Token"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing Token"),
            AuthAPIError::InvalidCookie => (StatusCode::BAD_REQUEST, "Invalid Cookie"),
            AuthAPIError::Invalid2FACode => (StatusCode::BAD_REQUEST, "Invalid 2FA code"),
            AuthAPIError::InvalidLoginAttamptId => (StatusCode::BAD_REQUEST, "Invalid login attempt id"),
            AuthAPIError::UnexpectedError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

fn log_error_chain(e: &(dyn Error + 'static)) {
    let separator =
        "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}
pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}

