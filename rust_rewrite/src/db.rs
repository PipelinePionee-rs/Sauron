use tokio_rusqlite::{Connection, Result};
use rusqlite::params;

pub async fn create_db_connection() -> Result<Connection> {
    let conn = Connection::open("sauron.db").await?;
    Ok(conn)
}