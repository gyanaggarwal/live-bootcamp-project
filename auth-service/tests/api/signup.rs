// use axum::response::IntoResponse;
// use reqwest::StatusCode;
//use auth_service::routes::signup;
use crate::helpers::TestApp;
#[tokio::test]

async fn signup_returns_200() {
    let app = TestApp::new().await;

    let response = app.post_signup().await;

    assert_eq!(response.status().as_u16(), 200);

}
