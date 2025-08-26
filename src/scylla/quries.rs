use anyhow::{Ok, Result};
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use scylla::response::query_result::QueryResult;
use scylla::statement::prepared::PreparedStatement;
use std::error::Error;

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
    // the replication is for local test
    let quries =
        "CREATE TABLE IF NOT EXISTS lektos.urls (url text PRIMARY KEY, stored_at timestamp)";

    let result = session.query_unpaged(quries, []).await?;

    Ok(result)
}

pub async fn store_url_hash(session: &Session, url: &str) -> Result<QueryResult> {
    let prepared: PreparedStatement = session
        .prepare("INSERT INTO lektos.urls (url, stored_at) VALUES (?, toTimestamp(now()))")
        .await?;

    let result = session.execute_unpaged(&prepared, (url,)).await?;
    

    Ok(result)
}

pub async fn get_url_hash(session: &Session, url: &str) -> Result<QueryResult> {
    let prepared: PreparedStatement = session
        .prepare("SELECT url FROM lektos.urls WHERE url=?")
        .await?;

    let result = session.execute_unpaged(&prepared, (url,)).await?;
    Ok(result)
}
