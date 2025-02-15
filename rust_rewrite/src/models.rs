use axum::response::IntoResponse;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct QueryParams {
  pub q: String,
  pub lang: Option<String>,
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

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, ToSchema)]
pub struct Data {
    pub data: Vec<Page>,
}