use crate::models::{QueryParams, Page};

use axum::Json;
use axum::extract::Query;
use axum::response::IntoResponse;




#[utoipa::path(get,
  path = "/search", responses(
   (status = 200, description = "Search results", body = Page),
 )
)]
pub async fn api_search(Query(query): Query<QueryParams>) -> impl IntoResponse {
 // For demonstration purposes, we build a dummy Page based on the query.
 let page = Page {
     title: format!("Title for page {}", query.q),
     url: format!("https://example.com/page/{}", query.q),
     language: "en".to_string(),
     last_updated: "2025-02-15".to_string(),
     content: format!("Content of page {}", query.q),
 };
 Json(page)
}