use std::sync::Arc;

use axum::{
    body::to_bytes,
    extract::{Query, State},
    response::IntoResponse,
};
use hyper::StatusCode;
use rust_rewrite::{api::api_search, repository::PageRepository};
use rust_rewrite::models::QueryParams;
use serde_json::Value;
use tokio_rusqlite::{params, Connection};

#[tokio::test]
async fn test_api_search_success() {
    // Create an in-memory database.
    let db = Connection::open_in_memory().await.unwrap();
    let repo = Arc::new(PageRepository { connection: db }); // Wrap db in PageRepository
    

    // Create the 'pages' table.
    let create_table = repo.connection
        .call(|conn| {
            conn.execute(
                "CREATE TABLE pages (
                        title TEXT,
                        url TEXT,
                        language TEXT,
                        last_updated TEXT,
                        content TEXT
                    )",
                [],
            )
            .map_err(|err| err.into())
        })
        .await;
    assert!(create_table.is_ok(), "Failed to create table");

    // Insert a test row.
    let insert = repo.connection
        .call(|conn| {
            conn.execute(
                "INSERT INTO pages (title, url, language, last_updated, content)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    "Test Title",
                    "http://example.com",
                    "en",
                    "2025-02-19",
                    "Some content here"
                ],
            )
            .map_err(|err| err.into())
        })
        .await;
    assert!(insert.is_ok(), "Failed to insert test data");

    // Build a query with a nonempty 'q' parameter.
    let query_params = QueryParams {
        q: Some("content".to_string()),
        lang: Some("en".to_string()),
    };

    // Call the handler directly.
    let response = api_search(State(repo.clone()), Query(query_params))
        .await
        .into_response();

    // Check that the response status is 200 OK.
    assert_eq!(response.status(), StatusCode::OK);

    // Convert the response body to bytes and then to JSON.
    let body_bytes = to_bytes(response.into_body(), 1000).await.unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    // Check that the "data" field exists and contains one page.
    let data = json
        .get("data")
        .expect("Response JSON missing `data` field")
        .as_array()
        .expect("`data` field is not an array");
    assert_eq!(data.len(), 1, "Expected one result");

    // Optionally, check that the returned page has the expected content.
    let page = &data[0];
    assert_eq!(
        page.get("title").unwrap(),
        "Test Title",
        "Returned page title does not match"
    );
} 


#[tokio::test]
async fn test_api_search_empty_query() {
    // Even though the DB isn’t used when 'q' is empty, we still create one.
    let db_connection = Connection::open_in_memory().await.unwrap();
    let repo = Arc::new(PageRepository {connection: db_connection});

    let query_params = QueryParams {
        q: Some("    ".to_string()), // q is empty after trimming.
        lang: Some("en".to_string()),
    };

    let response = api_search(State(repo), Query(query_params))
        .await
        .into_response();

    // Assuming that Error::UnprocessableEntity returns a 422 status.
    assert_eq!(
        response.status(),
        StatusCode::UNPROCESSABLE_ENTITY,
        "Expected 422 for empty query"
    );
}

#[tokio::test]
async fn test_api_search_db_error() {
    // Create an in-memory database but do NOT create the 'pages' table.
    let db_connection = Connection::open_in_memory().await.unwrap();
    let repo = Arc::new(PageRepository {connection: db_connection});

    let query_params = QueryParams {
        q: Some("test".to_string()),
        lang: Some("en".to_string()),
    };

    let response = api_search(State(repo), Query(query_params))
        .await
        .into_response();

    // assert that the response status is 500 Internal Server Error
    assert_eq!(
        response.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "Expected 500 when database operation fails"
    );
}
