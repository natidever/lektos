// src/embedding.rs

use std::env;

use anyhow::Context;
use anyhow::Error;
use anyhow::Ok;
use anyhow::Result;
use dotenv::dotenv;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;

use crate::{models::blog::Blog, utils::analysis::BlogResult};

const EMBEDING_MODEL: &str = "models/embedding-001";

#[derive(Debug, Serialize)]
struct EmbedContentPart {
    text: String,
}

#[derive(Debug, Serialize)]
struct EmbedContent {
    parts: Vec<EmbedContentPart>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedRequest {
    pub model: String,
    content: EmbedContent,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingValues {
    pub values: Vec<f32>,
}

#[derive(Debug, Deserialize)]
pub struct EmbedResponse {
    pub embedding: Option<EmbeddingValues>,
}
pub struct EmbedingResult {
    title: String,
    embedding: Vec<f32>,
    status: String,
}

pub async fn embed_blog(blogs:Vec<String>) -> Result<(), Error> {
    dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY").expect("msg");
    let embeding_url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-001:batchEmbedContents";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "x-goog-api-key",
        HeaderValue::from_str(&api_key).expect("Unable to find gemini keys"),
    );
    
    
    let request_body =blogs.iter().map(|blog| {
        json!({
            "requests":[
                {
      "model": "models/gemini-embedding-001",
      "content": {
      "parts":[{
        "text": blog}]}, }

            ]
            

        })
    }).collect::<Vec<_>>();


//     let request_body = json!(
//         {
//         "requests":[
//         {
//       "model": "models/gemini-embedding-001",
//       "content": {
//       "parts":[{
//         "text": "What is the meaning of life?"}
//         ]
//     } 
// },

//       {
//       "model": "models/gemini-embedding-001",
//       "content": {
//       "parts":[{
//         "text": "How much wood would a woodchuck chuck?"}]}, },
//       {
//       "model": "models/gemini-embedding-001",
//       "content": {
//       "parts":[{
//         "text": "How does the brain work?"}]}, }, ]
//     }
// );

    let client = Client::new();
    let response = client
        .post(embeding_url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let response_body = response.json::<Value>().await?;

        println!("Embedded Successfully:{}", response_body);
    } else {
        let error_body = response.text().await?;

        println!("Embedded Successfully:{}", error_body);
    }

    Ok(())
}


// blog_embedding.add(blogss)

pub async fn bactch_embeding(all_blogs:Vec<String>)->Result<(),Error>{

const BATCH_SIZE: usize = 50; 

 for blogs in all_blogs.chunks(BATCH_SIZE) {
 
 }
  
  




Ok(())


    
  


}


 