use dotenv::dotenv;
use std::env;

use qdrant_client::Qdrant;
use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};

// use crate::errors::{Result,Error};
use anyhow::Result;

pub async fn quadrant_check(client: &Qdrant) {
    let collections_list = client.list_collections().await;
    let _ = dbg!(collections_list);
}

pub async fn create_collection(client: &Qdrant, collection_name: &str) -> Result<()> {
    let collection = client
        .create_collection(
            CreateCollectionBuilder::new("blogs")
                .vectors_config(VectorParamsBuilder::new(768, Distance::Cosine)),
        )
        .await?;

    let _ = dbg!(collection);

    Ok(())
}
