use crate::models::{QueryParams, Page, LoginRequest, LoginResponse};

use axum::Json;
use axum::extract::Query;
use axum::response::IntoResponse;
use hyper::StatusCode;
use serde_json::json;

#[utoipa::path(get,
  path = "/api/search", responses(
   (status = 200, description = "Search results", body = Page),
 )
)]
/// this is a dummy search function that returns a dummy page
/// with the search query as the title and content
/// Will need to expand when we have a database
pub async fn api_search(Query(query): Query<QueryParams>) -> impl IntoResponse {
 let page = Page {
     title: format!("Title for page {}", query.q),
     url: format!("https://example.com/page/{}", query.q),
     language: "en".to_string(),
     last_updated: "2025-02-15".to_string(),
     content: format!("Content of page {}", query.q),
 };
 Json(page)
}


#[utoipa::path(post,
  path = "/api/login", responses(
   (status = 200, description = "Login successful", body = LoginResponse),
   (status = 401, description = "Invalid credentials", body = String),
 )
)]
/// for now, this just accepts a hardcoded username and password
/// and returns a dummy token in json format
/// TODO: will need to hash the password and check against a database
/// TODO: will need to generate a real token
pub async fn api_login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
  if payload.username == "admin" && payload.password == "password" {
      let response = json!({
        "token": "dummy-token".to_string(),
      });
      (StatusCode::OK, Json(response))
  } else {
      (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"})))
  }
}
     