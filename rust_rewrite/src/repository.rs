use tokio_rusqlite::{Connection, params, Result as db_result};
use crate::models::Page;

pub struct PageRepository{
    pub connection: Connection,
}


impl PageRepository {
    //Laver sin egen connection, med path som parameter.
    pub async fn new(db_path: &str) -> db_result<Self> {
        let connection = Connection::open(db_path).await?;
        Ok(Self {connection})
    }

    pub async fn search(&self, lang: String, q: String) -> db_result<Vec<Page>> {
        let query_string = format!("%{}%",q);

        self.connection
            .call(move |conn| {
                let mut stmt = conn.prepare(
                    "SELECT title, url, language, last_updated, content FROM pages WHERE language = ?1 AND content LIKE ?2",
                )?;

                let rows = stmt.query_map(params![&lang, &query_string], |row| {
                    Ok(Page {
                      title: row.get(0)?,
                      url: row.get(1)?,
                      language: row.get(2)?,
                      last_updated: row.get(3)?,
                      content: row.get(4)?,
                    })
                  })?;
                  let results: Vec<Page> = rows.filter_map(|res| res.ok()).collect();
                  Ok(results)
                  
                  //let results: Vec<Page> = rows
                //.filter_map(|res| res.map_err(|e| e.into()).ok());

            //Ok(results)
            })
            .await
        
    }
}

//let myrepo = pageRepository::new("sauron.db")
