use tokio_rusqlite::{Connection, Result};
use tracing::{error, info};
pub async fn create_db_connection() -> Result<Connection> {
    info!("->> Attempting to connect to database at data/sauron.db");
    match Connection::open("data/sauron.db").await {
        Ok(conn) => {
            info!("->> Successfully connected to database");
            Ok(conn)
        }
        Err(e) => {
            error!("->> Failed to connect to database: {:?}", e);
            Err(e)
        }
    }
}
