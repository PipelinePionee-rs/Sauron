#![allow(unused)]  // Silence warnings for dev purposes
use anyhow::Result;

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

const ADDRESS: &str = "http://127.0.0.1:8080";
const PATH: &str = "/api/login"; // the path you're currently working on

#[tokio::test]
async fn quick_dev() -> Result<()> {

  let hc = httpc_test::new_client(ADDRESS)?;
  hc.do_get(PATH).await?.print().await?;
  Ok(())
  
}