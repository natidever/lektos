use anyhow::{Ok, Result};
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use scylla::frame::response::result;
use scylla::response::query_result::{QueryResult, QueryRowsResult};
use scylla::statement::prepared::PreparedStatement;
use std::error::Error;

// Implement the dedup system and write he test 

pub async fn create_or_get_syclla_session() -> Result<Session> {
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await?;

    Ok(session)
}

pub async fn create_scylla_keyspace(session: &Session) -> Result<QueryResult> {
    let quries = "CREATE KEYSPACE IF NOT EXISTS lektos
            WITH REPLICATION = {
            'class': 'SimpleStrategy',
            'replication_factor': 1
            };";

    let result = session.query_unpaged(quries, []).await?;

    Ok(result)
}



pub async fn create_scylla_table(session: &Session) -> Result<QueryResult> {
    let quries =
        "CREATE TABLE IF NOT EXISTS lektos.urls (url text PRIMARY KEY, stored_at timestamp)";

    let result = session.query_unpaged(quries, []).await?;

    Ok(result)
}


#[derive(Default,Debug)]
pub struct UrlStoreProcedures;

impl  UrlStoreProcedures {

    pub async fn store_url_hash(&self,session: &Session, url: &str) -> Result<QueryResult> {
    "Storing and retriving are prepared quries it's not neccessary to make paged since the parsing at least in url level is sequntial ";
    let prepared: PreparedStatement = session
        .prepare("INSERT INTO lektos.urls (url, stored_at) VALUES (?, toTimestamp(now()))")
        .await?;

    let result = session.execute_unpaged(&prepared, (url,)).await?;

    Ok(result)
}
pub async fn get_url_hash(&self,session: &Session, url: &str) -> Result<Option<String>> {
    "Storing and retriving are prepared quries(not paged) it's not neccessary to make paged since the parsing, at least in url level is sequntial ";

    let prepared: PreparedStatement = session
        .prepare("SELECT url FROM lektos.urls WHERE url=?")
        .await?;

    let results = session
        .execute_unpaged(&prepared, (url,))
        .await?
        .into_rows_result()?;

    if let Some((row,)) = results.maybe_first_row::<(String,)>()? {
        let url = row;
        return Ok(Some(url));
    }

    Ok(None)
}

    
}



