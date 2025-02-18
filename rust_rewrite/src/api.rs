use std::sync::Arc;
// import models from models.rs
use crate::models::{self, Data, ErrorResponse, LoginRequest, LoginResponse, Page, QueryParams, RegisterRequest, RegisterResponse};
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
use tokio_rusqlite::{params, Connection, Result as SQLiteResult};
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
   (status = 422, description = "Invalid search query", body = ErrorResponse),
    ),
)]
/// Will need to expand when we have a database
pub async fn api_search(State(db): State<Arc<Connection>>, Query(query): Query<QueryParams>) -> impl IntoResponse {
    println!("->> Search endpoint hit with query: {:?}", query);
    // accepts 'q' and 'lang' query parameters
    let q = query.q.clone().unwrap_or_default();

    if q.trim().is_empty() {
        let error_response = ErrorResponse {
            status_code: 422,
            message: "Query parameter 'q' cannot be empty or absent.".to_string(),
        };
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response();
    }


    let lang = query.lang.clone().unwrap_or("en".to_string());

    let result = db
        .call(move |conn| { /// .call is async way to execute database operations it takes conn which is self-supplied (it's part of db)  move makes sure q and lang variables stay in scope.
            let mut stmt = conn.prepare(
                "SELECT title, url, language, last_updated, content FROM pages WHERE language = ?1 AND content LIKE ?2"
            )?;

            let rows = stmt.query_map(params![&lang, format!("%{}%", q)], |row| {
                Ok(Page {
                    title: row.get(0)?,
                    url: row.get(1)?,
                    language: row.get(2)?,
                    last_updated: row.get(3)?,
                    content: row.get(4)?,
                })
            })?;

            let results: Vec<Page> = rows.filter_map(|res| res.ok()).collect();
            Ok(results)
        })
        .await;

    match result {
        Ok(data) => Json(json!({ "data": data })).into_response(),
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            Json(json!({ "error": "Internal server error" })).into_response()
        }
    }

    //  let data = json!({
    //    "data": [],
    //  });
    // Json(data)
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