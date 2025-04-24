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

// #[tokio::test]
// async fn test_register_invalid_email() {
//     let db = Arc::new(Connection::open_in_memory().await.unwrap());

//     // Create the users table

//     let create_table = db
//         .call(|conn| {
//             conn.execute(
//                 "CREATE TABLE users (
//                     username TEXT,
//                     email TEXT,
//                     password TEXT
//                 )",
//                 [],
//             )
//             .map_err(|err| err.into())
//         })
//         .await;
//     assert!(create_table.is_ok(), "Failed to create table");

//     let register_request = RegisterRequest {
//         username: "newuser".to_string(),
//         email: "invalid-email".to_string(), // Invalid email format
//         password: "password123".to_string(),
//     };

//     let cookies = Cookies::default();

//     // Create a dummy header and body for the request.
//     let mut headers = HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse().unwrap());
//     let body = Bytes::from(serde_json::to_string(&register_request).unwrap());

//     let response = api_register(State(db.clone()), cookies, headers, body)
//         .await
//         .into_response();

//     assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
// }

// #[tokio::test]
// async fn test_register_username_exists() {
//     let db = Arc::new(Connection::open_in_memory().await.unwrap());

//     // Create the users table
//     let create_table = db
//         .call(|conn| {
//             conn.execute(
//                 "CREATE TABLE users (
//                     username TEXT,
//                     email TEXT,
//                     password TEXT
//                 )",
//                 [],
//             )
//             .map_err(|err| err.into())
//         })
//         .await;

//     assert!(create_table.is_ok(), "Failed to create table");

//     // Insert a test row
//     let insert = db
//         .call(move |conn| {
//             conn.execute(
//                 "INSERT INTO users (username, email, password)
//                 VALUES (?1, ?2, ?3)",
//                 params!["existinguser", "test@example.com", "password123"],
//             )
//             .map_err(|err| err.into())
//         })
//         .await;
//     assert!(insert.is_ok(), "Failed to insert test data");

//     let register_request = RegisterRequest {
//         username: "existinguser".to_string(),
//         email: "nottest@example.com".to_string(), // Different email so we know it's the username that's the issue.
//         password: "password123".to_string(),
//     };

//     // Create a dummy header and body for the request.
//     let mut headers = HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse().unwrap());
//     let body = Bytes::from(serde_json::to_string(&register_request).unwrap());

//     let cookies = Cookies::default();

//     let response = api_register(State(db.clone()), cookies, headers, body)
//         .await
//         .into_response();

//     assert_eq!(response.status(), StatusCode::CONFLICT);
// }

// #[tokio::test]
// async fn test_register_email_exists() {
//     let db = Arc::new(Connection::open_in_memory().await.unwrap());

//     // Create the users table
//     let create_table = db
//         .call(|conn| {
//             conn.execute(
//                 "CREATE TABLE users (
//                     username TEXT,
//                     email TEXT,
//                     password TEXT
//                 )",
//                 [],
//             )
//             .map_err(|err| err.into())
//         })
//         .await;

//     assert!(create_table.is_ok(), "Failed to create table");

//     // Insert a test row
//     let insert = db
//         .call(move |conn| {
//             conn.execute(
//                 "INSERT INTO users (username, email, password)
//                 VALUES (?1, ?2, ?3)",
//                 params!["existinguser", "test@example.com", "password123"],
//             )
//             .map_err(|err| err.into())
//         })
//         .await;
//     assert!(insert.is_ok(), "Failed to insert test data");

//     let register_request = RegisterRequest {
//         username: "notexistinguser".to_string(), // Different username so we know it's the email that's the issue.
//         email: "test@example.com".to_string(),
//         password: "password123".to_string(),
//     };

//     let cookies = Cookies::default();

//     // Create a dummy header and body for the request.
//     let mut headers = HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse().unwrap());
//     let body = Bytes::from(serde_json::to_string(&register_request).unwrap());

//     let response = api_register(State(db.clone()), cookies, headers, body)
//         .await
//         .into_response();

//     assert_eq!(response.status(), StatusCode::CONFLICT);
// }

// #[tokio::test]
// async fn test_register_url_encoded() {
//     let db = Arc::new(Connection::open_in_memory().await.unwrap());

//     // Create the users table
//     let create_table = db
//         .call(|conn| {
//             conn.execute(
//                 "CREATE TABLE users (
//                     username TEXT,
//                     email TEXT,
//                     password TEXT
//                 )",
//                 [],
//             )
//             .map_err(|err| err.into())
//         })
//         .await;

//     assert!(create_table.is_ok(), "Failed to create table");

//     let register_request = RegisterRequest {
//         username: "testuser".to_string(),
//         email: "test@example.com".to_string(),
//         password: "password123".to_string(),
//     };

//     // Create a dummy header for the request.
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         "Content-Type",
//         "application/x-www-form-urlencoded".parse().unwrap(),
//     );

//     // Create the body as a url-encoded string.
//     let body = Bytes::from(format!(
//         "username={}&email={}&password={}",
//         register_request.username, register_request.email, register_request.password
//     ));

//     let cookies = Cookies::default();

//     let response = api_register(State(db.clone()), cookies, headers, body)
//         .await
//         .into_response();

//     assert_eq!(response.status(), StatusCode::SEE_OTHER);
// }
