use axum::Router;
use axum::serve::Serve;
use axum::response::IntoResponse;
use axum::routing::post;
use reqwest::StatusCode;
use tower_http::services::ServeDir;
use std::error::Error;

pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        //let router= todo!();
        let router = Router::new()
                             .nest_service("/", ServeDir::new("assets"))
                             .route("/login", post(Self::login))
                             .route("/logout", post(Self::logout))
                             .route("/verify-2fa", post(Self::verify_2fa))
                             .route("/verify-token", post(Self::verify_token))
                             .route("/signup", post(Self::signup));
                             


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

    pub async fn signup() -> impl IntoResponse {
       StatusCode::OK.into_response()
    }

    pub async fn login() -> impl IntoResponse {
        StatusCode::OK.into_response()
    }
 
    pub async fn logout() -> impl IntoResponse {
        StatusCode::OK.into_response()
     }
 
    pub async fn verify_2fa() -> impl IntoResponse {
        StatusCode::OK.into_response()
    }

    pub async fn verify_token() -> impl IntoResponse {
        StatusCode::OK.into_response()
    }
  
    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}