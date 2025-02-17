use std::sync::Arc;
// import models from models.rs
use crate::models::{
  self, Data, LoginRequest, LoginResponse, Page, QueryParams, RegisterRequest, RegisterResponse
};
use crate::{Error, Result};

use axum::{
  routing::{get, post},
  extract::Query,
  response::IntoResponse,
  Json, Router,
};
use axum::extract::State;
use hyper::StatusCode;
use serde_json::{json, Value};
use tokio_rusqlite::Connection;
use tower_cookies::{Cookie, Cookies};
use utoipa::openapi::request_body::RequestBody;
use crate::auth::{self, hash_password, create_token};
use jsonwebtoken::{encode, Header, EncodingKey};

pub const TOKEN: &str = "token";

// squashes all the routes into one function
// so we can merge them into the main router
pub fn routes() -> Router<Arc<Connection>> {
  Router::new()
  .route("/login", post(api_login))
  .route("/register", post(api_register))
  .route("/logout", get(api_logout))
  .route("/search", get(api_search))
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
pub async fn api_search(State(db): State<Arc<Connection>>, Query(query): Query<QueryParams>) -> impl IntoResponse {
  println!("->> Search endpoint hit with query: {:?}", query);
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
pub async fn api_login(State(db): State<Arc<Connection>>, cookies: Cookies, payload: Json<LoginRequest>) -> impl IntoResponse {
  println!("->> Login endpoint hit with payload: {:?}", payload);

  let hashed_password = hash_password(&payload.password).await?;
  println!("hashed_password: {:?}", hashed_password);
  let is_correct = auth::verify_password(&payload.password, &hashed_password).await?;
  if payload.username != "admin" || payload.password != "password" {
    return Err(Error::LoginFail);
  }

  // create token, using function in auth.rs
  // it returns a Result<String>, so we unwrap it
  let token = create_token(&payload.username).unwrap();
  // build cookie with token
  let cookie = Cookie::build(token).http_only(true).secure(true).build();
  // add cookie to response
  cookies.add(cookie);

  let res = LoginResponse {
    message: "Login successful".to_string(),
    status_code: 200,
  };
  
  Ok(Json(res))
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
  println!("->> Register endpoint hit with payload: {:?}", payload);
  // TODO: will need to hash the password and save to a database

  // dummy function to check if credentials are valid
  // will need to check against db when its working
  fn valid_credentials() -> bool {
    true
  }

  if (valid_credentials()) {
    let res = RegisterResponse {
      message: "User registered successfully".to_string(),
      status_code: 200,
    };
    
    // create token, using function in auth.rs
    // it returns a Result<String>, so we unwrap it
    let token = create_token(&payload.username).unwrap();
    // build cookie with token
    let cookie = Cookie::build(token).http_only(true).secure(true).build();
    // add cookie to response
    cookies.add(cookie);

    (Json(res))
  } else {
    let res = RegisterResponse {
      message: "Invalid credentials".to_string(),
      status_code: 401,
    };
    ;
    (Json(res))
  }

}

#[utoipa::path(get, 
path = "/api/logout", responses(
  (status = 200, description = "Logout successful", body = String),
),
)]
pub async fn api_logout(State(db): State<Arc<Connection>>) -> impl IntoResponse {
  println!("->> Logout endpoint hit");
  (StatusCode::OK, Json(json!({"message": "Logout successful"})))
  // maybe remove token or smth here??
}