use std::env;
use dotenv::dotenv;

use qdrant_client::Qdrant;
use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};

// use crate::errors::{Result,Error};
use anyhow::Result;

pub async fn quadrant_check() {

        dotenv().ok();

  let quadrant_api=env::var("QUADRANT_API_KEY").expect("failed to load quadrant api key");
  let quadrant_url=env::var("QUADRANT_URL").expect("failed to load qdt url");

  let client = Qdrant::from_url(&quadrant_url)
      .api_key(quadrant_api)
      .build()
      .unwrap();
      
  let collections_list = client.list_collections().await;
  let _ = dbg!(collections_list);
}


pub async fn create_collection(quadrant_api_key:&str,quadrant_url:&str)->Result<()>{
            dotenv().ok();


  let client = Qdrant::from_url(quadrant_url)
      .api_key(quadrant_api_key)
      .build()
      .unwrap();

let collection=client
    .create_collection(
        CreateCollectionBuilder::new("{lektos}")
            .vectors_config(VectorParamsBuilder::new(100, Distance::Cosine)),
    )
    .await?;

    let _ = dbg!(collection);

    Ok(())
}