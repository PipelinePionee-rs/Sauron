use tracing::{error, info};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn create_db_connection() -> std::result::Result<Pool<Postgres>, sqlx::Error> {
    // Load the database URL from an environment variable
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@db/sauron".to_string());

    info!("->> Attempting to connect to database at {}", database_url);
    match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("->> Successfully connected to database");
            Ok(pool)
        }
        Err(e) => {
            error!("->> Failed to connect to database: {:?}", e);
            Err(e)
        }
    }
}

// pub async fn create_db_connection() -> Result<Connection> {
//     info!("->> Attempting to connect to database at data/sauron.db");
//     match Connection::open("data/sauron.db").await {
//         Ok(conn) => {
//             info!("->> Successfully connected to database");
//             Ok(conn)
//         }
//         Err(e) => {
//             error!("->> Failed to connect to database: {:?}", e);
//             Err(e)
//         }
//     }
// }

