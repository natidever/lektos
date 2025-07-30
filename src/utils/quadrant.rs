use std::env;

use qdrant_client::Qdrant;

async fn quadrant() {
    dotenv.load();
  let quadrant_api=env::var("QUADRANT_API_KEY").expect("failed to load quadrant api key");
  let client = Qdrant::from_url("https://0ec46abd-5051-4520-9102-8a131d60bc49.us-east4-0.gcp.cloud.qdrant.io:6334")
      .api_key(quadrant_api)
      .build()
      .unwrap();
      
  let collections_list = client.list_collections().await;
  let _ = dbg!(collections_list);
}