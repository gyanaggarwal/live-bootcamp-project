
use crate::helpers::TestApp;
use auth_service::{routes::SignupResponse, ErrorResponse};

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;
    let test_case = serde_json::json!({
        "email": "my.name@example.com",
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&test_case).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email(); // Call helper method to generate email 

     let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": random_email
        })
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
/*
#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters

    // Create an array of invalid inputs. Then, iterate through the array and 
    // make HTTP calls to the signup route. Assert a 400 HTTP status code is returned.
    let app = TestApp::new().await;
    let test_cases = [
        serde_json::json!({
            "email": "",
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "example.com",
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": "my.name@example.com",
            "password": "1234",
            "requires2FA": true
        })
    ];
    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await; // call `post_signup`
        assert_eq!(
            response
              .json::<ErrorResponse>()
              .await
              .expect("Could not deserialize response body to ErrorResponse")
              .error,
              "Invalid credentials".to_owned()
  
        );
    }   
}
*/
#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    // Call the signup route twice. The second request should fail with a 409 HTTP status code    
    let app = TestApp::new().await;
    let test_case1 = serde_json::json!({
        "email": "my.name@example.com",
        "password": "password123",
        "requires2FA": true
    });

    let test_case2 = test_case1.clone();
    let _ = app.post_signup(&test_case1).await;
    let response = app.post_signup(&test_case2).await;
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );

}
