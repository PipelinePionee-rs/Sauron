use axum::response::IntoResponse;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use std::collections::HashMap;
use jsonwebtoken::{decode, encode, Header, Validation, Algorithm, EncodingKey, DecodingKey};

#[derive(Deserialize)]
#[derive(Debug)]
pub struct QueryParams {
  pub q: Option<String>,
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
#[derive(Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub status_code: i32,
    pub message: String,
}

#[derive(Deserialize, ToSchema)]
#[derive(Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    
}

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
    pub message: String,
    pub status_code: i32,
}

#[derive(Serialize, ToSchema)]
pub struct Data {
    pub data: Vec<Page>,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub status_code: i32,
    pub message: String,
}