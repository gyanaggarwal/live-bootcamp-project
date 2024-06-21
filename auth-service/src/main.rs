#![allow(unused_imports)]
#![allow(dead_code)]

use std::sync::Arc;
use tokio::sync::RwLock;
use dotenvy::dotenv;
use sqlx::PgPool;
use reqwest::Client;
use secrecy::Secret;

use auth_service::{
    app_state::AppState, 
    get_postgres_pool,
    get_redis_client,
    domain::Email,
    services::{data_stores::{hashmap_user_store::HashmapUserStore, 
                             redis_banned_token_store::RedisBannedTokenStore,
                             redis_two_fa_store::RedisTwoFACodeStore},
               postmark_email_client::PostmarkEmailClient,
               mock_email_client::MockEmailClient}, 
               utils::tracing::init_tracing,
               utils::constants::{prod, DATABASE_URL, REDIS_HOST_NAME, POSTMARK_AUTH_TOKEN},
               Application
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    color_eyre::install().expect("Failed to install color_eyre");
    init_tracing().expect("Failed to initialize tracing");
//    let pg_pool = configure_postgresql().await;
//    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let redis_connection = Arc::new(RwLock::new(configure_redis()));
    let user_store = 
        Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_connection.clone()
    )));
    let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));
    let email_client = Arc::new(configure_postmark_email_client()); 
//    let email_client = Arc::new(MockEmailClient::default());
    
    let app_state = AppState::new(user_store, 
                                            banned_token_store, 
                                            two_fa_code_store,
                                            email_client);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let database_url = DATABASE_URL.to_owned();
    let pg_pool = get_postgres_pool(&database_url)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database! 
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");
    pg_pool
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}

fn configure_postmark_email_client() -> PostmarkEmailClient {
    let http_client = Client::builder()
        .timeout(prod::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(
        prod::email_client::BASE_URL.to_owned(),
        Email::parse(Secret::new(prod::email_client::SENDER.to_owned())).unwrap(),
        POSTMARK_AUTH_TOKEN.to_owned(),
        http_client,
    )
}






