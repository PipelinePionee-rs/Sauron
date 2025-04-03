use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use hyper::StatusCode;
use rust_rewrite::{api::api_register, models::RegisterRequest};
use tokio_rusqlite::{params, Connection};
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
                    email TEXT,
                    password TEXT
                )",
                [],
            )
            .map_err(|err| err.into())
        })
        .await;
    assert!(create_table.is_ok(), "Failed to create table");

    let register_request = RegisterRequest {
        username: "newuser".to_string(),
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    // Create dummy cookies.
    let cookies = Cookies::default();

    // Call the API function.
    let response = api_register(State(db.clone()), cookies, Json(register_request))
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    // Verify the user was actually inserted into the database
    let user_exists = db
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT username FROM users WHERE username = ?1")?;
            let rows = stmt.query_map(params!["newuser"], |row| row.get::<_, String>(0))?;
            let results: Vec<String> = rows.filter_map(|r| r.ok()).collect();
            Ok(results.len() == 1)
        })
        .await
        .unwrap();

    assert!(user_exists, "User was not found in database");

}

#[tokio::test]
async fn test_register_invalid_email() {
    let db = Arc::new(Connection::open_in_memory().await.unwrap());

    // Create the users table

    let create_table = db
        .call(|conn| {
            conn.execute(
                "CREATE TABLE users (
                    username TEXT,
                    email TEXT,
                    password TEXT
                )",
                [],
            )
            .map_err(|err| err.into())
        })
        .await;
    assert!(create_table.is_ok(), "Failed to create table");

    let register_request = RegisterRequest {
        username: "newuser".to_string(),
        email: "invalid-email".to_string(), // Invalid email format
        password: "password123".to_string(),
    };

    let cookies = Cookies::default();

    let response = api_register(State(db.clone()), cookies, Json(register_request))
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_register_username_exists() {
    let db = Arc::new(Connection::open_in_memory().await.unwrap());

    // Create the users table
    let create_table = db
        .call(|conn| {
            conn.execute(
                "CREATE TABLE users (
                    username TEXT,
                    email TEXT,
                    password TEXT
                )",
                [],
            )
            .map_err(|err| err.into())
        })
        .await;

    assert!(create_table.is_ok(), "Failed to create table");

    // Insert a test row
    let insert = db
        .call(move |conn| {
            conn.execute(
                "INSERT INTO users (username, email, password)
                VALUES (?1, ?2, ?3)",
                params!["existinguser", "test@example.com", "password123"],
            )
            .map_err(|err| err.into())
        })
        .await;
    assert!(insert.is_ok(), "Failed to insert test data");

    let register_request = RegisterRequest {
        username: "existinguser".to_string(),
        email: "nottest@example.com".to_string(), // Different email so we know it's the username that's the issue.
        password: "password123".to_string(),
    };

    let cookies = Cookies::default();

    let response = api_register(State(db.clone()), cookies, Json(register_request))
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_register_email_exists() {
    let db = Arc::new(Connection::open_in_memory().await.unwrap());

    // Create the users table
    let create_table = db
        .call(|conn| {
            conn.execute(
                "CREATE TABLE users (
                    username TEXT,
                    email TEXT,
                    password TEXT
                )",
                [],
            )
            .map_err(|err| err.into())
        })
        .await;

    assert!(create_table.is_ok(), "Failed to create table");

    // Insert a test row
    let insert = db
        .call(move |conn| {
            conn.execute(
                "INSERT INTO users (username, email, password)
                VALUES (?1, ?2, ?3)",
                params!["existinguser", "test@example.com", "password123"],
            )
            .map_err(|err| err.into())
        })
        .await;
    assert!(insert.is_ok(), "Failed to insert test data");

    let register_request = RegisterRequest {
        username: "notexistinguser".to_string(), // Different username so we know it's the email that's the issue.
        email: "test@example.com".to_string(),
        password: "password123".to_string(),
    };

    let cookies = Cookies::default();

    let response = api_register(State(db.clone()), cookies, Json(register_request))
        .await
        .into_response();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}
