use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use scylla::response::query_result::QueryResult;
use std::error::Error;
use anyhow::Result;

pub async fn create_or_get_syclla_session() -> Result<Session > {
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1:9042")
        .build()
        .await?;

    Ok(session)
}

pub async fn create_scylla_keyspace(scyla_session: &Session) -> Result<QueryResult > {
    let quries = "CREATE KEYSPACE IF NOT EXISTS lektos
            WITH REPLICATION = {
            'class': 'SimpleStrategy',
            'replication_factor': 1
            };";

    let result = scyla_session.query_unpaged(quries, []).await?;

    Ok(result)
}


pub async fn create_scylla_table(scyla_session: &Session) -> Result<QueryResult> {
    // the replication is for local test
    let quries =  "CREATE TABLE IF NOT EXISTS lektos.urls (url text PRIMARY KEY, stored_at timestamp)";


    let result = scyla_session.query_unpaged(quries, []).await?;
    

    Ok(result)
}


