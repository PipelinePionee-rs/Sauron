use std::sync::Arc;

use axum::{
    body::to_bytes,
    extract::{Query, State},
    response::IntoResponse,
};
use chrono::NaiveDateTime;
use hyper::StatusCode;
use rust_rewrite::{api::api_search, repository::PageRepository};
use rust_rewrite::models::QueryParams;
use serde_json::Value;

use sqlx::PgPool;

#[sqlx::test]
async fn test_api_search_success(pool: PgPool) {
    let repo = Arc::new(PageRepository { connection: pool }); // Wrap db in PageRepository
    

    // Insert a test row.
    sqlx::query(
        "INSERT INTO pages (title, url, language, last_updated, content)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind("Test Title")
    .bind("http://example.com")
    .bind("en")
    .bind(NaiveDateTime::parse_from_str("2025-02-19 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()) // Use NaiveDateTime to satisfy the SQLx type
    .bind("Some content here")
    .execute(&repo.connection)
    .await
    .expect("Failed to insert test data");

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


#[sqlx::test]
async fn test_api_search_empty_query(pool: PgPool) {
    let repo = Arc::new(PageRepository { connection: pool });

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

#[sqlx::test]
async fn test_api_search_db_error(pool: PgPool) {
    let repo = Arc::new(PageRepository { connection: pool });

    // drop the table to force a database error.
    sqlx::query("DROP TABLE IF EXISTS pages")
        .execute(&repo.connection)
        .await
        .expect("Failed to drop table");

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
