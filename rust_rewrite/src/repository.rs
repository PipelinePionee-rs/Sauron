use sqlx::Row;
use crate::db::DbPool;
use crate::models::Page;

pub struct PageRepository{
    pub connection: DbPool,
}


impl PageRepository {
  // Create a new repository with a database pool
  pub async fn new(pool: DbPool) -> Self {
      Self { connection: pool }
  }

  pub async fn search(&self, lang: String, q: String) -> Result<Vec<Page>, sqlx::Error> {
    let query_string = format!("%{}%", q);

    let rows = sqlx::query(
        "SELECT 
            title, 
            url, 
            language, 
            TO_CHAR(last_updated, 'YYYY-MM-DD HH24:MI:SS') as last_updated, 
            content 
         FROM pages 
         WHERE language = $1 AND content LIKE $2"
    )
    .bind(lang)
    .bind(query_string)
    .map(|row: sqlx::postgres::PgRow| {
        Page {
            title: row.get("title"),
            url: row.get("url"),
            language: row.get("language"),
            last_updated: row.get("last_updated"),
            content: row.get("content"),
        }
    })
    .fetch_all(&self.connection)
    .await?;

    Ok(rows)
}
}

// impl PageRepository {
//     //Laver sin egen connection, med path som parameter.
//     pub async fn new(db_path: &str) -> db_result<Self> {
//         let connection = Connection::open(db_path).await?;
//         Ok(Self {connection})
//     }

//     pub async fn search(&self, lang: String, q: String) -> db_result<Vec<Page>> {
//         let query_string = format!("%{}%",q);

//         self.connection
//             .call(move |conn| {
//                 let mut stmt = conn.prepare(
//                     "SELECT title, url, language, last_updated, content FROM pages WHERE language = ?1 AND content LIKE ?2",
//                 )?;

//                 let rows = stmt.query_map(params![&lang, &query_string], |row| {
//                     Ok(Page {
//                       title: row.get(0)?,
//                       url: row.get(1)?,
//                       language: row.get(2)?,
//                       last_updated: row.get(3)?,
//                       content: row.get(4)?,
//                     })
//                   })?;
//                   let results: Vec<Page> = rows.filter_map(|res| res.ok()).collect();
//                   Ok(results)
                  
//                   //let results: Vec<Page> = rows
//                 //.filter_map(|res| res.map_err(|e| e.into()).ok());

//             //Ok(results)
//             })
//             .await
        
//     }
// }

//let myrepo = pageRepository::new("sauron.db")
