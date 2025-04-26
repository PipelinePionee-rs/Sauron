use std::sync::Arc;
use axum::{
    body::to_bytes,
    extract::State,
    response::IntoResponse, Json,
};
use hyper::StatusCode;
use rust_rewrite::{api::api_login, auth::hash_password, models::LoginRequest};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};

#[sqlx::test]
async fn test_login_success(pool: PgPool) {
    // Create users table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    let test_password = "password";
    let hashed_password = hash_password(test_password).await.unwrap();

    // Insert a test user
    sqlx::query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)")
        .bind("admin")
        .bind("admin@example.com")
        .bind(&hashed_password)
        .execute(&pool)
        .await
        .expect("Failed to insert test user");

    let login_request = LoginRequest {
        username: "admin".to_string(),
        password: "password".to_string(),
    };

    // Create dummy cookies
    let cookies = Cookies::default();
    cookies.add(Cookie::new("token", "test_token"));

    // Call the API function
    let response = api_login(State(Arc::new(pool)), cookies, Json(login_request))
        .await
        .into_response();

    // Check that the response is OK
    assert_eq!(response.status(), StatusCode::OK);

    // Check that the response body is correct
    let body = to_bytes(response.into_body(), 1000).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        json!({
            "message": "Login successful",
            "status_code": 200,
        })
    );
}

#[sqlx::test]
async fn test_login_fail(pool: PgPool) {
    // Create users table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    let test_password = "password";
    let hashed_password = hash_password(test_password).await.unwrap();

    // Insert a test user
    sqlx::query("INSERT INTO users (username, email, password) VALUES ($1, $2, $3)")
        .bind("admin")
        .bind("admin@example.com")
        .bind(&hashed_password)
        .execute(&pool)
        .await
        .expect("Failed to insert test user");

    let login_request = LoginRequest {
        username: "admin".to_string(),
        password: "wrong_password".to_string(),
    };

    // Create dummy cookies
    let cookies = Cookies::default();
    cookies.add(Cookie::new("token", "test_token"));

    // Call the API function
    let response = api_login(State(Arc::new(pool)), cookies, Json(login_request))
        .await
        .into_response();

    // Check that the response is Unauthorized
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}