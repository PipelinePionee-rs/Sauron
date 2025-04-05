use std::sync::Arc;
// import models from models.rs

use crate::auth::{self, create_token, hash_password};
use crate::error::Error;
use crate::models::{
    ApiErrorResponse, ChangePasswordRequest, ChangePasswordResponse, Data, LoginRequest,
    LoginResponse, LogoutResponse, QueryParams, RegisterRequest, RegisterResponse, WeatherResponse,
};
use axum::extract::State;
use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{get, post, put},
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

pub fn routes(db: Arc<Connection>, repo: Arc<PageRepository>) -> Router {
    Router::new()
        .route("/login", post(api_login))
        .route("/register", post(api_register))
        .route("/change_password", put(api_change_password))
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

  match repo.search(lang, q).await {
      Ok(data) => Json(json!({ "data": data })).into_response(),
      Err(_err) => Error::GenericError.into_response(),
  }
}

// ---------------------------------------------------
// Change password
// ---------------------------------------------------
#[utoipa::path(put,
  path = "/api/change_password", responses(
   (status = 200, description = "Password changed successfully", body = ChangePasswordResponse),
   (status = 401, description = "Unauthorized", body = ApiErrorResponse),
   (status = 422, description = "Invalid password", body = ApiErrorResponse),
  ),
  request_body = ChangePasswordRequest,
)]
pub async fn api_change_password(
    State(db): State<Arc<Connection>>,
    cookies: Cookies,
    payload: Json<ChangePasswordRequest>,
) -> Result<Json<ChangePasswordResponse>, Error> {
    println!("->> Change password endpoint hit");

    // Get token from cookie
    let token = cookies.get(TOKEN).map(|c| c.value().to_string());
    if token.is_none() {
        println!("->> No auth token found");
        return Err(Error::InvalidCredentials);
    }

    let token = token.unwrap();
    let masked_token = format!("{}...{}", &token[..4], &token[token.len()-4..]);
    println!("->> Found token: {:?}", masked_token);
    // Decode token to get username
    let claims = match auth::decode_token(&token) {
        Ok(claims) => {
            println!("->> Successfully decoded token");
            claims
        }
        Err(e) => {
            println!("->> Failed to decode token: {:?}", e);
            return Err(Error::InvalidCredentials);
        }
    };

    let username = claims.sub;
    println!("->> Decoded username from token: {:?}", username);

    // Hash new password
    let hashed_password = match hash_password(&payload.new_password).await {
        Ok(hash) => {
            println!("->> Successfully hashed new password");
            hash
        }
        Err(e) => {
            println!("->> Failed to hash password: {:?}", e);
            return Err(Error::GenericError);
        }
    };

    // Update password in database
    match db
        .call(move |conn| {
            let mut stmt = conn.prepare("UPDATE users SET password = ?1 WHERE username = ?2")?;
            stmt.execute(params![&hashed_password, &username])?;
            Ok(())
        })
        .await
    {
        Ok(_) => {
            println!("->> Successfully updated password in database");
            Ok(Json(ChangePasswordResponse {
                status_code: 200,
                message: "Password changed successfully".to_string(),
            }))
        }
        Err(e) => {
            println!("->> Failed to update password in database: {:?}", e);
            Err(Error::GenericError)
        }
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

    // Validate email format
    if !EMAIL_REGEX.is_match(&payload.email.to_lowercase()) {
        return Err(Error::InvalidCredentials);
    }

    let username = payload.username.clone();
    let email = payload.email.clone();

    // check if username already exists
    let res = db
        .call(move |conn| {
            let mut stmt = conn.prepare("SELECT 1 FROM users WHERE username = ?1 OR email = ?2")?;
            let mut rows = stmt.query(params![username, email])?;
            Ok(rows.next()?.is_some())
        })
        .await;

    // if username exists, return error
    if let Ok(true) = res {
        return Err(Error::UsernameOrEmailExists);
    }

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

    // sql returns number of affected rows, so we check if it's 1
    if res == 1 {
        // create token, using function in auth.rs
        // it returns a Result<String>, so we unwrap it
        let token = create_token(&username_token).unwrap();
        // build cookie with token
        let cookie = Cookie::build((TOKEN, token))
            .http_only(true)
            .secure(true)
            .build();
        // add cookie to response
        cookies.add(cookie);

        // Redirect to "/"
        Ok(axum::response::Redirect::to("/").into_response())
    } else {
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
