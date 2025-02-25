//TODO: Lav en pageRepository struckt. Den skal have 1 field som skal være en Connection type læs længere nede
use tokio_rusqlite::{Connection, Result};
use rusqlite::params;
use crate::models::Page;

struct PageRepository{
    connection: Connection,
}

//TODO: Lav en implementation af pageRepository impl
impl PageRepository {
    //husk public (også i struct)
    pub async fn new(db_path: &str) -> Result<Self> {
        let connection = Connection::open(db_path).await?;
        Ok(Self {connection})
    }

    pub async fn search(&self, lang: String, q: String) -> Result<Vec<Page>> {
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

                  let results: Vec<Page> = rows
                .filter_map(|res| res.map_err(|e| e.into()).ok()) // Convert rusqlite::Error into tokio_rusqlite::Error
                .collect();

            Ok(results)
            })
            .await
        
    }
}

// TODO: lav 1 assosiated function new() som er en slags construktor den skal nok tage en path til DB filen
    //så du kan lave en connetion. bliver kaldt ved at sige pageRepository::new()
    // TODO: new funtionen skal lave forbindelse til DB så den selv holder styr på sin egen forbindelse. du kan/skal bruge samme logik som
    // Lars burge i DB.rs

//TODO: lav en metode som er den logik Lars allerede har lavet i api 
//let myrepo = pageRepository::new("sauron.db")
