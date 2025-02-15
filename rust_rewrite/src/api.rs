// import models from models.rs
use crate::models::{
  QueryParams, 
  Page, 
  LoginRequest, 
  LoginResponse, 
  RegisterRequest, 
  RegisterResponse, 
  Data
};

use axum::Json;
use axum::extract::Query;
use axum::response::IntoResponse;
use hyper::StatusCode;
use serde_json::json;
use utoipa::openapi::request_body::RequestBody;



#[utoipa::path(get,
  path = "/api/search",
  params(
    ("q" = String, Query, description = "Search query parameter"),
    ("lang" = Option<String>, Query, description = "Language parameter"),
),
   responses(
   (status = 200, description = "Search successful", body = Data),
   (status = 422, description = "Invalid search query", body = String),
 ),
)]
/// Will need to expand when we have a database
pub async fn api_search(Query(query): Query<QueryParams>) -> impl IntoResponse {
  // accepts 'q' and 'lang' query parameters
  let data = json!({
    "data": [],
  });
 Json(data)
}


#[utoipa::path(post,
  path = "/api/login", responses(
   (status = 200, description = "Login successful", body = LoginResponse),
   (status = 401, description = "Invalid credentials", body = String),
 ),
 request_body = LoginRequest,
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


#[utoipa::path(post,
  path = "/api/register", responses(
   (status = 200, description = "User registered successfully", body = RegisterResponse),
   (status = 401, description = "Invalid credentials", body = String),
  ),
   request_body = RegisterRequest,
 )
]
pub async fn api_register(Json(payload): Json<RegisterRequest>) -> impl IntoResponse {
  // TODO: will need to hash the password and save to a database
  // TODO: will need to generate a real token

  // dummy function to check if credentials are valid
  // will need to check against db when its working
  fn valid_credentials() -> bool {
    true
  }

  if (valid_credentials()) {
    let response = json!({
      "message": "User registered successfully",
      "token": "dummy-token".to_string(),
    });
    (StatusCode::CREATED, Json(response))
  } else {
    (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"})))
  }
  
}

#[utoipa::path(get, 
path = "/api/logout", responses(
  (status = 200, description = "Logout successful", body = String),
),
)]
pub async fn api_logout() -> impl IntoResponse {
  (StatusCode::OK, Json(json!({"message": "Logout successful"})))
  // maybe remove token or smth here??
}