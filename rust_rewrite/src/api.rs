use std::sync::Arc;
// import models from models.rs
use crate::models::{
    self, ApiErrorResponse, Data, LoginRequest, LoginResponse, LogoutResponse, Page, QueryParams,
    RegisterRequest, RegisterResponse,
};
use crate::{Error, Result};

use crate::auth::{self, create_token, hash_password};
use axum::extract::State;
use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use hyper::StatusCode;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::{json, Value};
use tokio_rusqlite::{params, Connection, Result as SQLiteResult};
use tower_cookies::{Cookie, Cookies};
use utoipa::openapi::request_body::RequestBody;

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
   (status = 422, description = "Invalid search query", body = ApiErrorResponse),
    ),
)]
pub async fn api_search(
    State(db): State<Arc<Connection>>,
    Query(query): Query<QueryParams>,
) -> impl IntoResponse {
    println!(
        "->> Search endpoint hit with query: {:?} and lang: {:?}",
        query.q, query.lang
    );
    // accepts 'q' and 'lang' query parameters
    let q = query.q.clone().unwrap_or_default();

    if q.trim().is_empty() {
        return Error::UnprocessableEntity.into_response();
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
            return Error::GenericError.into_response();
        }
    }
}

#[utoipa::path(post,
  path = "/api/login", responses( 
   (status = 200, description = "Login successful", body = LoginResponse),
   (status = 401, description = "Invalid credentials", body = ApiErrorResponse),
 ),
 request_body = LoginRequest,
)]
pub async fn api_login(
    State(db): State<Arc<Connection>>,
    cookies: Cookies,
    payload: Json<LoginRequest>,
) -> impl IntoResponse {
    println!("->> Login endpoint hit with payload: {:?}", payload);

    // get username from payload
    let username = payload.username.clone();
    // get password from payload
    let password = payload.password.clone();

    let username_result = db
        .call(move |conn| {
            let mut stmt =
                conn.prepare("SELECT username, password FROM users WHERE username = ?1")?;

            let rows = stmt.query_map(params![&username], |row| Ok((row.get(0)?, row.get(1)?)))?;

            let results: Vec<(String, String)> = rows.filter_map(|res| res.ok()).collect();
            Ok(results)
        })
        .await;

    let db_password = username_result.unwrap()[0].1.clone();

    // verify password
    let is_correct = auth::verify_password(&payload.password, &db_password).await?;

    println!("->> password match: {:?}", is_correct);

    if is_correct == false {
        return Err(Error::InvalidCredentials);
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
   (status = 401, description = "Invalid credentials", body = ApiErrorResponse),
  ),
   request_body = RegisterRequest,
)
]
pub async fn api_register(
    db: State<Arc<Connection>>,
    cookies: Cookies,
    payload: Json<RegisterRequest>,
) -> impl IntoResponse {
    println!("->> Register endpoint hit with payload: {:?}", payload);
    // TODO: will need to hash the password and save to a database

    // since the username is being "used" twice, we have to clone it,
    // else the first usage in the db function will consume it.
    // the original payload.username is used for the db function, username_token is used for the token function

    // clone username for token generation
    let username_token = payload.username.clone();

    // hash password
    let hashed_password = hash_password(&payload.password).await?;

    // insert payload into db
    let res = db
        .call(move |conn| {
            conn.execute(
                "INSERT INTO users (username, email, password) VALUES (?1, ?2, ?3)",
                params![&payload.username, &payload.email, &hashed_password], // note we insert hashed password
            )
            .map_err(tokio_rusqlite::Error::from) // we have to convert the error type, else rust will complain
        })
        .await?;

    // sql returns 1 if successful, 0 if not
    if (res == 1) {
        let res = RegisterResponse {
            message: "User registered successfully".to_string(),
            status_code: 200,
        };

        // create token, using function in auth.rs
        // it returns a Result<String>, so we unwrap it
        let token = create_token(&username_token).unwrap();
        // build cookie with token
        let cookie = Cookie::build(token).http_only(true).secure(true).build();
        // add cookie to response
        cookies.add(cookie);

        Ok(Json(res))
    } else {
        return Err(Error::InvalidCredentials);
    }
}

#[utoipa::path(get,
    path = "/api/logout", responses(
  (status = 200, description = "Logout successful", body = LogoutResponse),
    ),
)]
pub async fn api_logout(State(db): State<Arc<Connection>>) -> impl IntoResponse {
    println!("->> Logout endpoint hit");

    let res = LogoutResponse {
        message: "Logout successful".to_string(),
        status_code: 200,
    };
    Json(res)
    // maybe remove token or smth here??
}
