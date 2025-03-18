use std::sync::Arc;
// import models from models.rs

use crate::auth::{self, create_token, hash_password};
use crate::error::Error;
use crate::models::{
    ApiErrorResponse, Data, LoginRequest, LoginResponse, LogoutResponse, QueryParams,
    RegisterRequest, RegisterResponse, WeatherResponse,
};
use axum::extract::State;
use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::repository::PageRepository;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::json;
use tokio_rusqlite::{params, Connection};
use tower_cookies::{Cookie, Cookies};

use reqwest::Client;

pub const TOKEN: &str = "auth_token";

// god i hate regex
// i hope chatgpt wrote this correctly
lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
    )
    .unwrap();
}

// squashes all the routes into one function
// so we can merge them into the main router
/* pub fn routes() -> Router<Arc<Connection>> {
    Router::new()
        .route("/login", post(api_login))
        .route("/register", post(api_register))
        .route("/logout", get(api_logout))
        .route("/search", get(api_search))
} */

pub fn routes(db: Arc<Connection>, repo: Arc<PageRepository>) -> Router {
    Router::new()
        .route("/login", post(api_login))
        .route("/register", post(api_register))
        .route("/logout", get(api_logout))
        .route("/weather", get(api_weather))
        .route("/search", get(api_search).with_state(repo)) // Only `search` uses PageRepository
        .with_state(db) // Other routes still use Connection
}

// ---------------------------------------------------
// Search
// ---------------------------------------------------
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
    State(repo): State<Arc<PageRepository>>,
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

    /* let result = db
    .call(move |conn| { // .call is async way to execute database operations it takes conn which is self-supplied (it's part of db)  move makes sure q and lang variables stay in scope.
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
      })?; */

    // return results as a vector (like ArrayList in Java)
    // if we wanted to .push or .pop we would have to use a mutable variable
    // like: let mut results = Vec::new();
    /* let results: Vec<Page> = rows.filter_map(|res| res.ok()).collect();
      Ok(results)
    })
    .await;

    match result {
        Ok(data) => Json(json!({ "data": data })).into_response(),

        Err(_err) => {
            Error::GenericError.into_response()
        }
    } */

    match repo.search(lang, q).await {
        Ok(data) => Json(json!({ "data": data })).into_response(),
        Err(_err) => Error::GenericError.into_response(),
    }
}

// ---------------------------------------------------
// Login
// ---------------------------------------------------
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

    let db_result = db
        .call(move |conn| {
            let mut stmt =
                conn.prepare("SELECT username, password FROM users WHERE username = ?1")?;

            let rows = stmt.query_map(params![&username], |row| Ok((row.get(0)?, row.get(1)?)))?;

            let results: Vec<(String, String)> = rows.filter_map(|res| res.ok()).collect();
            Ok(results)
        })
        .await;

    // extract password from first row, second column of db_result
    let db_password = &db_result.unwrap()[0].1;

    // verify password
    let is_correct = auth::verify_password(&payload.password, db_password).await?;

    println!("->> password match: {:?}", is_correct);

    if !is_correct {
        return Err(Error::InvalidCredentials);
    }

    // create token, using function in auth.rs
    // it returns a Result<String>, so we unwrap it
    let token = create_token(&payload.username).unwrap();
    // build cookie with token
    let cookie = Cookie::build((TOKEN, token))
        .http_only(true)
        .secure(true)
        .path("/")
        .build();
    // add cookie to response
    cookies.add(cookie);

    let res = LoginResponse {
        message: "Login successful".to_string(),
        status_code: 200,
    };

    Ok(Json(res))
}

// ---------------------------------------------------
// Register
// ---------------------------------------------------
#[utoipa::path(post,
  path = "/api/register", responses(
   (status = 200, description = "User registered successfully", body = RegisterResponse),
   (status = 401, description = "Invalid credentials", body = ApiErrorResponse),
   (status = 409, description = "Username already exists", body = ApiErrorResponse),
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

    // Log the registration attempt with username
    println!(
        "[REGISTER] Registration attempt for user: {}",
        payload.username
    );

    // Validate email format
    if !EMAIL_REGEX.is_match(&payload.email.to_lowercase()) {
        println!("[REGISTER] Invalid email format: {}", payload.email);
        return Err(Error::InvalidCredentials);
    }

    println!(
        "[REGISTER] Email validation passed for user: {}",
        payload.username
    );

    let username = payload.username.clone();

    // check if username already exists
    let res = db
        .call(move |conn| {
            let mut stmt = conn.prepare("SELECT username FROM users WHERE username = ?1")?;
            let mut rows = stmt.query(params![username])?;
            let exists = rows.next()?.is_some();
            println!("->> Username exists check: {:?}", exists);
            Ok(exists)
        })
        .await;

    // if username exists, return error
    if let Ok(true) = res {
        println!("[REGISTER] Username already exists: {}", payload.username);
        return Err(Error::UsernameExists);
    }

    println!(
        "[REGISTER] Username check passed, username is available: {}",
        payload.username
    );

    // since the username is being "used" twice, we have to clone it,
    // else the first usage in the db function will consume it.
    // the original payload.username is used for the db function, username_token is used for the token function

    // clone username for token generation
    let username_token = payload.username.clone();

    // hash password
    println!("[REGISTER] Hashing password for user: {}", payload.username);
    let hashed_password = match hash_password(&payload.password).await {
        Ok(hashed) => {
            println!("[REGISTER] Password hashing successful");
            hashed
        }
        Err(e) => {
            println!("[REGISTER] Password hashing failed: {:?}", e);
            return Err(e);
        }
    };

    // insert payload into db
    println!(
        "[REGISTER] Attempting database insertion for user: {}",
        payload.username
    );
    let res = db
        .call(move |conn| {
            conn.execute(
                "INSERT INTO users (username, email, password) VALUES (?1, ?2, ?3)",
                params![&payload.username, &payload.email, &hashed_password], // note we insert hashed password
            )
            .map_err(|e| {
                println!("[REGISTER] Database insertion error: {:?}", e);
                tokio_rusqlite::Error::from(e) // we have to convert the error type, else rust will complain
            })
        })
        .await;

    // Handle database insertion errors
    if let Err(e) = &res {
        println!("[REGISTER] Database operation failed: {:?}", e);
        return Err(Error::GenericError);
    }

    // sql returns number of affected rows, so we check if it's 1
    let affected_rows = res?;
    if affected_rows == 1 {
        println!(
            "[REGISTER] User registered successfully: {}",
            username_token
        );
        let res = RegisterResponse {
            message: "User registered successfully".to_string(),
            status_code: 200,
        };

        // create token, using function in auth.rs
        // it returns a Result<String>, so we unwrap it
        println!(
            "[REGISTER] Creating auth token for user: {}",
            username_token
        );
        let token = match create_token(&username_token) {
            Ok(token) => {
                println!("[REGISTER] Token created successfully");
                token
            }
            Err(e) => {
                println!("[REGISTER] Token creation failed: {:?}", e);
                return Err(Error::GenericError);
            }
        };

        // build cookie with token
        let cookie = Cookie::build((TOKEN, token))
            .http_only(true)
            .secure(true)
            .build();
        // add cookie to response
        cookies.add(cookie);
        println!("[REGISTER] Auth cookie added to response");

        println!(
            "[REGISTER] Registration complete for user: {}",
            username_token
        );
        Ok(Json(res).into_response()) // The compiler started throwing a fit over a type mismatch here; hopefully using into_response() fixes that without breaking anything else.
    } else {
        println!("[REGISTER] Database insertion did not affect any rows");
        Err(Error::InvalidCredentials)
    }
}

// ---------------------------------------------------
// Logout
// ---------------------------------------------------
#[utoipa::path(get,
  path = "/api/logout", responses(
  (status = 200, description = "Logout successful", body = LogoutResponse),
  ),
)]
pub async fn api_logout(State(_db): State<Arc<Connection>>, cookies: Cookies) -> impl IntoResponse {
    println!("->> Logout endpoint hit");

    let res = LogoutResponse {
        message: "Logout successful".to_string(),
        status_code: 200,
    };
    // removes auth_token from client
    cookies.remove(Cookie::from(TOKEN));
    Json(res)
}

// ---------------------------------------------------
// Weather
// ---------------------------------------------------

#[utoipa::path(get,
  path = "/api/weather", responses(
   (status = 200, description = "Weather data", body = Data),
  ),
)]
pub async fn api_weather() -> impl IntoResponse {
    println!("->> Weather endpoint hit");

    // Call the weather API.
    // Currently only fetches for Copenhagen. This can easily be changed, but I'm not sure how it'll interact with the simulation.
    // The API key is hardcoded, but that's basically a non-issue since it's a free subscription on a dummy account.
    // If we need more than a million requests per month, we can just add more keys and have it switch if the first one is rejected.

    let client = Client::new();
    let response = client
        .get("http://api.weatherapi.com/v1/forecast.json?key=d2f1555420344801b83193615252802&q=Copenhagen&days=5&aqi=no&alerts=no")
        .send()
        .await
        .unwrap()
        .json::<WeatherResponse>()
        .await
        .unwrap();

    // Should be wrapped as Data, but that causes the compiler to complain.
    Json(response)
}

// ---------------------------------------------------
// Dummy routes
// ---------------------------------------------------
#[allow(dead_code)]
#[utoipa::path(
    get,
    path = "/",
    summary = "Serve Root page",
    responses(
        (status = 200, description = "Successful Response", body = String, content_type = "text/html")
    )
)]
async fn root_dummy() {}

#[allow(dead_code)]
#[utoipa::path(
    get,
    path = "/register",
    summary = "Serve Register Page",
    responses(
        (status = 200, description = "Successful Response", body = String, content_type = "text/html")
    )
)]
async fn register_dummy() {}

#[allow(dead_code)]
#[utoipa::path(
    get,
    path = "/login",
    summary = "Serve Login Page",
    responses(
        (status = 200, description = "Successful Response", body = String, content_type = "text/html")
    )
)]
async fn login_dummy() {}

#[allow(dead_code)]
#[utoipa::path(
    get,
    path = "/weather",
    summary = "Serve Weather Page",
    responses(
        (status = 200, description = "Successful Response", body = String, content_type = "text/html")
    )
)]
async fn weather_dummy() {}
