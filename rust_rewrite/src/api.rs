use std::sync::Arc;
// import models from models.rs

use crate::auth::{self, create_token, hash_password};
use crate::error::Error;
use crate::models::{
    ApiErrorResponse, ChangePasswordRequest, ChangePasswordResponse, Data, LoginRequest,
    LoginResponse, LogoutResponse, QueryParams, RegisterRequest, RegisterResponse, WeatherResponse,
};
use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap};
use axum::{
    Router,
    routing::get,
    extract::MatchedPath,
    middleware::{self, Next},
    extract::{Query,Request},
    response::{IntoResponse,Response},
    routing::{post, put},
    Json,
    body::Body
};
use prometheus::{CounterVec, HistogramVec, GaugeVec, register_counter_vec, register_histogram_vec, register_gauge_vec};
use prometheus::{Encoder, TextEncoder};
use std::time::Instant;



use crate::repository::PageRepository;
use hyper::StatusCode;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::json;
use crate::db::DbPool;
use tower_cookies::{Cookie, Cookies};

use reqwest::Client;

use tracing::{error, info};

// Uses for metrics
use metrics_exporter_prometheus::PrometheusHandle;


pub const TOKEN: &str = "auth_token";

// god i hate regex
// i hope chatgpt wrote this correctly
lazy_static! {
    static ref EMAIL_REGEX: Regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})"
    )
    .unwrap();
}

async fn metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}


pub fn routes(db: Arc<DbPool>, repo: Arc<PageRepository>, prometheus_handle: PrometheusHandle) -> Router { 
    Router::new()
        .route("/login", post(api_login))
        .route("/register", post(api_register))
        .route("/change_password", put(api_change_password))
        .route("/logout", get(api_logout))
        .route("/weather", get(api_weather))
        .route("/search", get(api_search).with_state(repo))
        .route("/metrics", get(metrics)) 
        .with_state(db)
        .layer(middleware::from_fn(track_metrics))
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
    info!(
        "Search endpoint hit with query: {:?} and lang: {:?}",
        query.q, query.lang
    );

    let timer = DB_QUERY_DURATION.with_label_values(&["search"]).start_timer();

    // accepts 'q' and 'lang' query parameters
    let q = query.q.clone().unwrap_or_default();

    if q.trim().is_empty() {
        return Error::UnprocessableEntity.into_response();
    }

    let lang = query.lang.clone().unwrap_or("en".to_string());

    match repo.search(lang, q).await {
        Ok(data) => {
            timer.observe_duration();
            Json(json!({ "data": data })).into_response()
        },
        Err(_err) => {
            timer.observe_duration();
            Error::GenericError.into_response()
        },
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
  State(db): State<Arc<DbPool>>,
  cookies: Cookies,
  payload: Json<ChangePasswordRequest>,
) -> Result<Json<ChangePasswordResponse>, Error> {
  info!("Change password endpoint hit");

  // Get token from cookie
  let token = cookies.get(TOKEN).map(|c| c.value().to_string());
  if token.is_none() {
      info!("No auth token found");
      return Err(Error::InvalidCredentials);
  }

  let token = token.unwrap();
  let masked_token = format!("{}...{}", &token[..4], &token[token.len() - 4..]);
  info!("Found token: {:?}", masked_token);
  
  // Decode token to get username
  let claims = match auth::decode_token(&token) {
      Ok(claims) => {
          info!("Successfully decoded token");
          claims
      }
      Err(e) => {
          error!("Failed to decode token: {:?}", e);
          return Err(Error::InvalidCredentials);
      }
  };

  let username = claims.sub;
  info!("Decoded username from token: {:?}", username);

  // Hash new password
  let hashed_password = match hash_password(&payload.new_password).await {
      Ok(hash) => {
          info!("Successfully hashed new password");
          hash
      }
      Err(e) => {
          error!("Failed to hash password: {:?}", e);
          return Err(Error::GenericError);
      }
  };

  // Update password in database
  let result = sqlx::query("UPDATE users SET password = $1 WHERE username = $2")
      .bind(hashed_password)
      .bind(username)
      .execute(db.as_ref())
      .await;

  match result {
      Ok(_) => {
          info!("->> Successfully updated password in database");
          Ok(Json(ChangePasswordResponse {
              status_code: 200,
              message: "Password changed successfully".to_string(),
          }))
      }
      Err(e) => {
          error!("->> Failed to update password in database: {:?}", e);
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
    State(db): State<Arc<DbPool>>,
    cookies: Cookies,
    payload: Json<LoginRequest>,
) -> impl IntoResponse {
    info!("->> Login endpoint hit with payload: {:?}", payload);

    let timer = DB_QUERY_DURATION.with_label_values(&["get_user"]).start_timer();

    // Get user from database
    let user = sqlx::query_as::<_, (String, String)>("SELECT username, password FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(db.as_ref())
        .await;

    // Handle database errors
    let user = match user {
      Ok(Some(user)) => user,
      Ok(None) => return Err(Error::InvalidCredentials),
      Err(e) => {
          error!("->> Database error: {:?}", e);
          return Err(Error::GenericError);
      }
    };

    // extract password from user tuple
    let db_password = &user.1;

    // verify password
    let is_correct = auth::verify_password(&payload.password, db_password).await?;

    info!("password match: {:?}", is_correct);

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
    timer.observe_duration();

    Ok(Json(res))
}

// ---------------------------------------------------
// Register
// ---------------------------------------------------
#[utoipa::path(
    post,
    path = "/api/register",
    request_body(content = RegisterRequest, content_type = "application/json"),
    request_body(content = RegisterRequest, content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 200, description = "User registered successfully", body = RegisterResponse),
        (status = 401, description = "Invalid credentials", body = ApiErrorResponse),
        (status = 409, description = "Username already exists", body = ApiErrorResponse),
    )
)]

pub async fn api_register(
    db: State<Arc<DbPool>>,
    cookies: Cookies,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {

    // Check the content-type header to see if it's JSON or url-encoded form data.
    if let Some(content_type) = headers.get("Content-Type") {
        if content_type == "application/json" {
            // Parse the body as JSON.
            match serde_json::from_slice::<RegisterRequest>(&body) {
                Ok(payload) => {
                    return api_register_logic(db, cookies, payload)
                        .await
                        .into_response()
                }
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        "Invalid request body (parsed as JSON)",
                    )
                        .into_response()
                }
            }
        } else if content_type == "application/x-www-form-urlencoded" {
            // Parse the body as url-encoded form data.
            match serde_urlencoded::from_bytes::<RegisterRequest>(&body) {
                Ok(payload) => {
                    return api_register_logic(db, cookies, payload)
                        .await
                        .into_response()
                }
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        "Invalid request body (parsed as url-encoded)",
                    )
                        .into_response()
                }
            }
        }
    }

    // If the content type isn't either of the above, return an error.
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "error": "Invalid Content-Type" })),
    )
        .into_response()
}

pub async fn api_register_logic(
    db: State<Arc<DbPool>>,
    cookies: Cookies,
    payload: RegisterRequest, // Request body extracted by the helper method above.
) -> impl IntoResponse {
    info!("Register endpoint hit with payload: {:?}", payload);

    let timer = DB_QUERY_DURATION.with_label_values(&["insert_user"]).start_timer();

    // Validate email format
    if !EMAIL_REGEX.is_match(&payload.email.to_lowercase()) {
        return Err(Error::InvalidCredentials);
    }

    // Check if username or email already exists
    let user_exists = sqlx::query_scalar::<_, bool>(
      "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 OR email = $2)"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .fetch_one(db.as_ref())
    .await;

    // If username exists, return error
    match user_exists {
      Ok(true) => return Err(Error::UsernameOrEmailExists),
      Err(e) => {
          info!("->> Database error: {:?}", e);
          return Err(Error::GenericError);
      }
      _ => {}
    }

    // clone username for token generation
    let username_token = payload.username.clone();

    // hash password
    let hashed_password = hash_password(&payload.password).await?;

    // Insert new user into database
    let result = sqlx::query(
      "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)"
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&hashed_password)
    .execute(db.as_ref())
    .await;

    timer.observe_duration();

    match result {
      Ok(pg_result) => {
          if pg_result.rows_affected() == 1 {
              // Create token
              let token = create_token(&username_token).unwrap();
              // Build cookie with token
              let cookie = Cookie::build((TOKEN, token))
                  .http_only(true)
                  .secure(true)
                  .build();
              // Add cookie to response
              cookies.add(cookie);

              // Redirect to "/"
              Ok(axum::response::Redirect::to("/").into_response())
          } else {
              Err(Error::InvalidCredentials)
          }
      }
      Err(e) => {
          error!("Failed to insert user: {:?}", e);
          Err(Error::GenericError)
      }
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
pub async fn api_logout(cookies: Cookies) -> impl IntoResponse {
    info!("Logout endpoint hit");

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
    info!("->> Weather endpoint hit");

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
// Metrics 
// ---------------------------------------------------
lazy_static! {
    static ref HTTP_REQUESTS: CounterVec = register_counter_vec!(
        "http_requests_total",
        "Total HTTP requests",
        &["method", "path", "status"]
    ).unwrap();
    static ref HTTP_REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds",
        &["method", "path"]
    ).unwrap();
    static ref HTTP_IN_FLIGHT: GaugeVec = register_gauge_vec!(
        "http_in_flight_requests",
        "In-flight HTTP requests",
        &["method", "path"]
    ).unwrap();
    // DB query metrics
    static ref DB_QUERY_DURATION: HistogramVec = register_histogram_vec!(
        "db_query_duration_seconds",
        "Database query durations",
        &["query"]
    ).unwrap();
}

async fn track_metrics<B>(req: Request<B>, next: Next) -> Response<Body>
where B: Send + 'static, Body: From<B>
{
    let method = req.method().as_str().to_owned();
    let path    = req
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str().to_string())
        .unwrap_or_else(|| "unknown".into());

    HTTP_IN_FLIGHT.with_label_values(&[&method, &path]).inc();
    let start = Instant::now();

    let (parts, body) = req.into_parts();
    let response = next.run(Request::from_parts(parts, Body::from(body))).await;
    let status = response.status().as_u16().to_string();
    let elapsed = start.elapsed().as_secs_f64();

    HTTP_REQUESTS.with_label_values(&[&method, &path, &status]).inc();
    HTTP_REQUEST_DURATION.with_label_values(&[&method, &path, &status]).observe(elapsed);
    HTTP_IN_FLIGHT.with_label_values(&[&method, &path]).dec();

    response
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
