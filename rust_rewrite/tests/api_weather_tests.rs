use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tokio_rusqlite::Connection;
use std::sync::Arc;
use serde_json::Value;
use tower::ServiceExt;

#[tokio::test]
async fn test_api_weather() {
    // Create a mock state and router.
    let db = Arc::new(Connection::open_in_memory().await.unwrap());
    let app = axum::Router::new().route("/weather", axum::routing::get(rust_rewrite::api::api_weather)).layer(axum::Extension(db));

    // Create a request to the /api/weather endpoint.
    let request = Request::builder()
        .uri("/weather")
        .body(Body::empty())
        .unwrap();

    // Send the request.
    let response = app.clone().oneshot(request).await.unwrap();

    // Check the status code.
    assert_eq!(response.status(), StatusCode::OK);

    // Extract the body.
    let body = axum::body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Check if the JSON contains data for three days.
    let forecastdays = json["forecast"]["forecastday"].as_array().unwrap();
    assert_eq!(forecastdays.len(), 3);

    // Check that all the keys have values.
    for forecastday in forecastdays {
        for (key, value) in forecastday.as_object().unwrap() {
            assert!(!value.is_null(), "Key {} has no value", key);
        }
    }
}