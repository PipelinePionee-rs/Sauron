use std::sync::Arc;

use axum::{
    body::to_bytes,
    extract::State,
    response::IntoResponse, Json,
};
use hyper::StatusCode;
use rust_rewrite::{api::api_register, models::RegisterRequest};
use serde_json::{json, Value};
use tokio_rusqlite::Connection;
use tower_cookies::Cookies;

#[tokio::test]
async fn test_register_success() {
    // Create an in-memory database.
    let db = Arc::new(Connection::open_in_memory().await.unwrap());

    // Create the 'users' table.
    let create_table = db
        .call(|conn| {
            conn.execute(
                "CREATE TABLE users (
                        username TEXT,
                        password TEXT,
                        email TEXT
                    )",
                [],
            )
            .map_err(|err| err.into())
        })
        .await;
    assert!(create_table.is_ok(), "Failed to create table");

    let register_request = RegisterRequest {
        username: "admin".to_string(),
        password: "password".to_string(),
        email: "email@e.mail".to_string(),
    };

    // Create dummy cookies.
    let cookies = Cookies::default();

    // Call the API function.
    let response = api_register(
        State(db.clone()),
        cookies,
        Json(register_request),
    )
    .await
    .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1000).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body, json!({
        "message": "User registered successfully",
        "status_code": 200,
    }));
}

#[tokio::test]
async fn test_register_username_taken() {
    // Create an in-memory database.
    let db = Arc::new(Connection::open_in_memory().await.unwrap());

    // Create the 'users' table.
    let create_table = db
        .call(|conn| {
            conn.execute(
                "CREATE TABLE users (
                        username TEXT,
                        password TEXT,
                        email TEXT
                    )",
                [],
            )
            .map_err(|err| err.into())
        })
        .await;
    assert!(create_table.is_ok(), "Failed to create table");

    // Insert a test row.

    let insert = db
        .call(|conn| {
            conn.execute(
                "INSERT INTO users (username, password, email) VALUES (?1, ?2, ?3)",
                &[
                    "admin",
                    "password",
                    "email@e.mail",
                ],
            )
            .map_err(|err| err.into())
        })
        .await;
    assert!(insert.is_ok(), "Failed to insert row");

    let register_request = RegisterRequest {
        username: "admin".to_string(),
        password: "password".to_string(),
        email: "email@e.mail".to_string(),
    };

    // Create dummy cookies.
    let cookies = Cookies::default();

    // Call the API function.
    let response = api_register(
        State(db.clone()),
        cookies,
        Json(register_request),
    )
    .await
    .into_response();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), 1000).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();

    // Print the actual response body for debugging
    println!("Actual response body: {:?}", body);

    assert_eq!(body, json!({
        "message": "Username already exists",
        "status_code": 409,
    }));
}