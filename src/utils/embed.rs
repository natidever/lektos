// src/embedding.rs

use std::env;

// use anyhow::Context;
use anyhow::Error;
// use anyhow::Ok;
use anyhow::Result;
// use crate::errors::{Error, Result};
// use anyhow::Ok;
use dotenv::dotenv;
use qdrant_client::qdrant::qdrant_client::QdrantClient;
use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};
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

pub async fn embed_blog(blogs: &[String]) -> Result<Vec<Vec<f32>>> {
    dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY").expect("msg");
    let embeding_url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-001:batchEmbedContents";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        "x-goog-api-key",
        HeaderValue::from_str(&api_key).expect("Unable to find gemini keys"),
    );

    let requests: Vec<_> = blogs
        .iter()
        .map(|blog| {
            json!({
                "model": "models/gemini-embedding-001",
                 "task_type":"SEMANTIC_SIMILARITY",
                "content": {
                    "parts": [{
                        "text": blog
                    }]
                }
            })
        })
        .collect();
    let request_body = json!({ "requests": requests });

    let client = Client::new();
    let response = client
        .post(embeding_url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;

    // println!("response :{:?}", response);

    if response.status().is_success() {
        let response_body = response.json::<Value>().await?;

        // println!("Embedded Successfully:{}", response_body);

        // let values = response.json::serde_json<>
        let embeddings = response_body["embeddings"].as_array().unwrap();

        // Process each embedding in the batch
        let mut results = Vec::with_capacity(embeddings.len());
        for embedding in embeddings {
            if let Some(values) = embedding["values"].as_array() {
                let mut vec = Vec::with_capacity(values.len());
                for value in values {
                    // vec.push(value.as_f64().context("Invalid embedding value")? as f32) ;
                    let val = value
                        .as_f64()
                        .ok_or_else(|| anyhow::anyhow!("Invalid embedding value: {:?}", value))?;
                    vec.push(val as f32);

                    // vec.push(value);
                }
                results.push(vec);
            }

            // println!("values:{:?}",results)
            for (i, embedding) in results.iter().take(3).enumerate() {
                println!(
                    "Embedding {} ({} dims): {:?}",
                    i,
                    embedding.len(),
                    &embedding[..embedding.len().min(5)] // First 5 elements
                );
            }
        }

        return Ok(results);
    } else {
        let error_body = response.text().await?;
        return Err(Error::msg("message"));

        // return Err(Error::EmbeddingService(error_body));
    }
}

pub async fn bactch_embeding(all_blogs: Vec<String>) -> Result<()> {
    const BATCH_SIZE: usize = 1;
    let mut count = 1;

    for blogs in all_blogs.chunks(BATCH_SIZE) {
        println!("{} Batch", count);
        count += 1;
        let result = embed_blog(blogs).await?;
        // get the result

        // store it to qdrant
    }

    Ok(())
    //
}

// const COLLECTION_NAME: &str = "blogs";
// pub async fn store_embeddings(
//     client: &QdrantClient,
//     // blogs: &[String],
//     embeddings: Vec<Vec<f32>>,
// ) -> Result<()> {
//     // Prepare points (1:1 mapping of blogs to embeddings)
//     let points: Vec<_> = blogs
//         .iter()
//         .zip(embeddings)
//         .enumerate()
//         .map(|(id, (blog, vector))| {
//             PointStruct::new(
//                 id as u64,  // Simple sequential ID
//                 vector,
//                 serde_json::json!({ "content": blog }).try_into().unwrap(),
//             )
//         })
//         .collect();

//     // Upsert in single batch
//     // client
//     //     .upsert_points(COLLECTION_NAME, points, None)
//     //     .await?;

//     Ok(())
// }
