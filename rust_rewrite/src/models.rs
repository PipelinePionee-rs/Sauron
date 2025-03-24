use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema, Debug)]
pub struct ApiErrorResponse {
    pub status_code: i32,
    pub error: String,
    pub message: String,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, ToSchema, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub status_code: i32,
    pub message: String,
}

#[derive(Deserialize, ToSchema, Debug)]
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

// #[derive(Serialize, ToSchema)]
// pub struct ErrorResponse {
//     pub status_code: i32,
//     pub message: String,
// }

#[derive(Serialize, ToSchema)]
pub struct LogoutResponse {
    pub status_code: i32,
    pub message: String,
}

// Allows the JSON response from the weather API to be deserialized into Rust structs.

#[derive(Deserialize, Serialize)]
pub struct WeatherCondition {
    text: String,
    icon: String,
    code: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Day {
    maxtemp_c: f64,
    mintemp_c: f64,
    avgtemp_c: f64,
    maxwind_kph: f64,
    totalprecip_mm: f64,
    avghumidity: i32,
    condition: WeatherCondition,
}

#[derive(Deserialize, Serialize)]
pub struct ForecastDay {
    date: String,
    day: Day,
}

#[derive(Deserialize, Serialize)]
pub struct Forecast {
    forecastday: Vec<ForecastDay>,
}

#[derive(Deserialize, Serialize)]
pub struct WeatherResponse {
    forecast: Forecast,
}