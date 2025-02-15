use axum::response::IntoResponse;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Deserialize)]
pub struct QueryParams {
  pub q: String,
}

// ToSchema generates a tab in /swagger-ui for the struct
#[derive(Serialize, ToSchema)]
pub struct Page {
  pub title: String,
  pub url: String,
  pub language: String,
  pub last_updated: String,
  pub content: String,
}