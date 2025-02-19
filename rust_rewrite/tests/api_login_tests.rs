use std::sync::Arc;

use axum::{
    body::to_bytes,
    extract::State,
    response::IntoResponse, Json,
};
use hyper::StatusCode;
use rust_rewrite::{api::api_login, models::LoginRequest};
use serde_json::{json, Value};
use tokio_rusqlite::{params, Connection};
use tower_cookies::{Cookie, Cookies};

#[tokio::test]
async fn test_login_success() {
    // Create an in-memory database.
    let db = Arc::new(Connection::open_in_memory().await.unwrap());

    // Create the 'users' table.
    let create_table = db
        .call(|conn| {
            conn.execute(
                "CREATE TABLE users (
                        username TEXT,
                        password TEXT
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
                "INSERT INTO users (username, password)
                     VALUES (?1, ?2)",
                params![
                    "admin",
                    "password",
                ],
            )
            .map_err(|err| err.into())
        })
        .await;
    assert!(insert.is_ok(), "Failed to insert test data");

    let login_request = LoginRequest {
        username: "admin".to_string(),
        password: "password".to_string(),
    };

    // create dummy cookies
    let cookies = Cookies::default();
    cookies.add(Cookie::new("token", "test_token"));

    // Call the API function.
    let response = api_login(State(db.clone()), cookies, Json(login_request))
        .await
        .into_response();

    // Check that the response is OK.
    assert_eq!(response.status(), StatusCode::OK);

    // Check that the response body is correct.
    let body = to_bytes(response.into_body(), 1000).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({
        "message": "Login successful",
        "status_code": 200,
    }));
}