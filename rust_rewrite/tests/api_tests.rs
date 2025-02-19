use std::sync::Arc;

use axum::{body::to_bytes, extract::{Query, State}, response::IntoResponse};
use hyper::StatusCode;
use rust_rewrite::models::QueryParams;
use rust_rewrite::api::api_search;
use serde_json::Value;
use tokio_rusqlite::{params, Connection};




// just an example test, so 'cargo test' actually runs something.
#[tokio::test]
async fn test_example() {
  assert_eq!(2 + 2, 4);
}

#[tokio::test]
    async fn test_api_search_success() {
        // Create an in-memory database.
        let db = Arc::new(Connection::open_in_memory().await.unwrap());

        // Create the 'pages' table.
        let create_table = db
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
        let insert = db
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
        let response = api_search(State(db.clone()), Query(query_params))
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