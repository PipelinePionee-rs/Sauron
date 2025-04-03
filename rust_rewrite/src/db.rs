use tokio_rusqlite::{Connection, Result};

pub async fn create_db_connection() -> Result<Connection> {
    let conn = Connection::open("data/sauron.db").await?;
    Ok(conn)
}