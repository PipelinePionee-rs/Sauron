#![allow(unused)]  // Silence warnings for dev purposes
use anyhow::Result;
use axum::Json;
use serde_json::json;

/// quick development test that will hit PATH on ADDRESS
/// and print the response to the console
/// to watch the output, run the server:
/// 
/// cargo watch -q -c -w src/ -x run
/// 
/// this will recompile the server when changes are made
/// 
/// and run the test with a separate terminal:
/// 
/// cargo watch -q -c -w src/ -x "test -q quick_dev -- --nocapture"
/// 
/// this will run the test and print the output to the console
/// every time you change/save the quick_dev file 

const ADDRESS: &str = "http://localhost:8080";
const GET_PATH: &str = "/api/logout";
const POST_PATH: &str = "/api/login";

#[tokio::test]
async fn quick_get() -> Result<()> {
  let hc = httpc_test::new_client(ADDRESS)?;
  hc.do_get(GET_PATH).await?.print().await?;
  Ok(()) 
}

#[tokio::test]
async fn quick_post() -> Result<()> {
  let hc = httpc_test::new_client(ADDRESS)?;
  let body = json!({
    "username": "admin",
    "password": "password",
  });
    hc.do_post(POST_PATH, body).await?.print().await?;
    Ok(())
}