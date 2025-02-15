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
use crate::{Error, Result};

use axum::{
  routing::{get, post},
  extract::Query,
  response::IntoResponse,
  Json, Router,
};
use hyper::StatusCode;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use utoipa::openapi::request_body::RequestBody;


pub const AUTH_TOKEN: &str = "auth-token";

pub fn routes() -> Router {
  Router::new()
  .route("/api/login", post(api_login))
  .route("/api/register", post(api_register))
  .route("/api/logout", get(api_logout))
  .route("/api/search", get(api_search))
}


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
pub async fn api_login(cookies: Cookies, payload: Json<LoginRequest>) -> Result<Json<Value>> {
  if payload.username != "admin" || payload.password != "password" {
    return Err(Error::LoginFail);
  }

  cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));

  let body = json!({
    "result": {
      "success": true
    }
  });
  
  Ok(Json(body))
}


#[utoipa::path(post,
  path = "/api/register", responses(
   (status = 200, description = "User registered successfully", body = RegisterResponse),
   (status = 401, description = "Invalid credentials", body = String),
  ),
   request_body = RegisterRequest,
 )
]
pub async fn api_register(cookies: Cookies, payload: Json<RegisterRequest>) -> impl IntoResponse {
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
    cookies.add(Cookie::new(AUTH_TOKEN, "user-1.exp.sign"));
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