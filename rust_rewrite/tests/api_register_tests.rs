use axum::{body::Bytes, extract::State, http::HeaderMap, response::IntoResponse};
use hyper::StatusCode;
use rust_rewrite::{api::api_register, models::RegisterRequest};
use std::sync::Arc;
use tower_cookies::Cookies;
use sqlx::PgPool;

#[sqlx::test]
async fn test_register_success(pool: PgPool) {
    // No need to create the users table - migrations will handle that
    let register_request = RegisterRequest {
        username: "newuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    // Create a dummy header and body for the request.
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let body = Bytes::from(serde_json::to_string(&register_request).unwrap());

    // Create dummy cookies.
    let cookies = Cookies::default();

    // Clone the pool before moving it into the Arc
    let pool_for_api = pool.clone();
    
    // Call the API function.
    let response = api_register(State(Arc::new(pool_for_api)), cookies, headers, body)
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    // For a redirect response, we don't expect a body
    // Verify the user was actually inserted into the database
    let user_exists = sqlx::query(
      "SELECT username FROM users WHERE username = $1"
    )
    .bind("newuser")
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_some();

    assert!(user_exists, "User was not found in database");
}

#[sqlx::test]
async fn test_register_invalid_email(pool: PgPool) {

    let register_request = RegisterRequest {
        username: "newuser".to_string(),
        email: "invalid-email".to_string(), // Invalid email format
        password: "password123".to_string(),
    };

    let cookies = Cookies::default();

    // Create a dummy header and body for the request.
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let body = Bytes::from(serde_json::to_string(&register_request).unwrap());

    let response = api_register(State(Arc::new(pool)), cookies, headers, body)
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_register_username_exists(pool: PgPool) {

    // Insert a test row
    sqlx::query(
        "INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)",
    )
    .bind("existinguser")
    .bind("test@example.com")
    .bind("password123")
    .execute(&pool)
    .await
    .expect("Failed to insert test data");

    // Create a register request with the same username
    let register_request = RegisterRequest {
        username: "existinguser".to_string(),
        email: "nottest@example.com".to_string(), // Different email so we know it's the username that's the issue.
        password: "password123".to_string(),
    };

    // Create a dummy header and body for the request.
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let body = Bytes::from(serde_json::to_string(&register_request).unwrap());

    let cookies = Cookies::default();

    let response = api_register(State(Arc::new(pool)), cookies, headers, body)
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[sqlx::test]
async fn test_register_email_exists(pool: PgPool) {

    // Insert a test row
    sqlx::query(
        "INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)",
    )
    .bind("notexistinguser")
    .bind("test@example.com")
    .bind("password123")
    .execute(&pool)
    .await
    .expect("Failed to insert test data");

    // Create a register request with the same email
    let register_request = RegisterRequest {
        username: "notexistinguser".to_string(), // Different username so we know it's the email that's the issue.
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let cookies = Cookies::default();

    // Create a dummy header and body for the request.
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let body = Bytes::from(serde_json::to_string(&register_request).unwrap());

    let response = api_register(State(Arc::new(pool)), cookies, headers, body)
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[sqlx::test]
async fn test_register_url_encoded(pool: PgPool) {

    let register_request = RegisterRequest {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    // Create a dummy header for the request.
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    // Create the body as a url-encoded string.
    let body = Bytes::from(format!(
        "username={}&email={}&password={}",
        register_request.username, register_request.email, register_request.password
    ));

    let cookies = Cookies::default();

    let response = api_register(State(Arc::new(pool)), cookies, headers, body)
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);
}
