use axum::{body::to_bytes, response::IntoResponse};
use hyper::StatusCode;
use rust_rewrite::api::{api_logout, TOKEN};
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[sqlx::test]
async fn test_logout_success() {

    // Create cookies with an auth token
    let cookies = Cookies::default();
    cookies.add(Cookie::new(TOKEN, "test_auth_token"));

    // Verify cookie exists before logout
    assert!(
        cookies.get(TOKEN).is_some(),
        "Auth token should exist before logout"
    );

    // Call the logout endpoint
    let response = api_logout(cookies.clone()).await.into_response();

    // Check status code is OK
    assert_eq!(response.status(), StatusCode::OK);

    // Check response body
    let body = to_bytes(response.into_body(), 1000).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        json!({
            "message": "Logout successful",
            "status_code": 200,
        })
    );

    // Verify the auth token cookie was removed
    assert!(
        cookies.get(TOKEN).is_none(),
        "Auth token should be removed after logout"
    );
}
