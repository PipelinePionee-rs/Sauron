use tokio_rusqlite::{Connection, Result};

pub async fn create_db_connection() -> Result<Connection> {
  println!("->> Attempting to connect to database at data/sauron.db");
  match Connection::open("data/sauron.db").await {
      Ok(conn) => {
          println!("->> Successfully connected to database");
          Ok(conn)
      }
      Err(e) => {
          println!("->> Failed to connect to database: {:?}", e);
          Err(e)
      }
  }
}