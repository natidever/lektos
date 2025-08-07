// src/embedding.rs

use std::env;
use std::time::Duration;

// use anyhow::Context;
use anyhow::Error;
// use anyhow::Ok;
use anyhow::Result;
// use crate::errors::{Error, Result};
// use anyhow::Ok;
use dotenv::dotenv;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::PointsSelector;
use qdrant_client::qdrant::Vectors;
use qdrant_client::qdrant::qdrant_client::QdrantClient;
use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};
use qdrant_client::qdrant::{Value as QdrantValue, Vector};
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlite::Connection;
use std::collections::HashMap;
use uuid::Uuid;

use crate::extractors::source_tracker::SourceTracker;
use crate::{models::blog::Blog, utils::analysis::BlogResult};

const EMBEDING_MODEL: &str = "models/embedding-001";

pub struct DbMetadata {
    pub title: String,
    pub author: String,
    pub date: String,
    pub publisher: String,
}

pub struct QdrantdbObject {
    pub id: String,
    pub metadata: DbMetadata,
    pub content: String,
}

// pub async fn embed_blog(blogs: Vec<&str>) -> Result<Vec<Vec<f32>>> {
//     dotenv().ok();

//     let api_key = env::var("GEMINI_API_KEY").expect("failed to get env");
//     println!("{}", api_key);
//     let embeding_url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-001:batchEmbedContents";

//     // More aggressive rate limiting settings
//     const MAX_RETRIES: u32 = 5; // Increased from 3 to 5
//     const INITIAL_BACKOFF: u64 = 10; // Start with 5 seconds instead of 1
//     let mut retry_count = 0;
//     let mut last_error = None;

//     // Process blogs in smaller batches
//     const BATCH_SIZE: usize = 1; // Reduced batch size
//     let mut all_results = Vec::with_capacity(blogs.len());

//     for chunk in blogs.chunks(BATCH_SIZE) {
//         let mut chunk_retry_count = 0;
//         let mut chunk_processed = false;

//         while chunk_retry_count <= MAX_RETRIES && !chunk_processed {
//             let mut headers = HeaderMap::new();
//             headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
//             headers.insert(
//                 "x-goog-api-key",
//                 HeaderValue::from_str(&api_key).expect("Unable to find gemini keys"),
//             );

//             let requests: Vec<_> = chunk
//                 .iter()
//                 .map(|blog| {
//                     json!({
//                         "model": "models/text-embedding-005",
//                         "task_type": "SEMANTIC_SIMILARITY",
//                         "content": {
//                             "parts": [{
//                                 "text": blog
//                             }]
//                         }
//                     })
//                 })
//                 .collect();
//             let request_body = json!({ "requests": requests });

//             let client = Client::new();
//             match client
//                 .post(embeding_url)
//                 .headers(headers)
//                 .json(&request_body)
//                 .send()
//                 .await
//             {
//                 Ok(response) => {
//                     if response.status().is_success() {
//                         let response_body = response.json::<Value>().await?;
//                         if let Some(embeddings) = response_body["embeddings"].as_array() {
//                             let mut results = Vec::with_capacity(embeddings.len());
//                             for embedding in embeddings {
//                                 if let Some(values) = embedding["values"].as_array() {
//                                     let mut vec = Vec::with_capacity(values.len());
//                                     for value in values {
//                                         if let Some(val) = value.as_f64() {
//                                             vec.push(val as f32);
//                                         }
//                                     }
//                                     results.push(vec);
//                                 }
//                             }
//                             all_results.extend(results);
//                             chunk_processed = true;
//                             // Reset retry count on success
//                             retry_count = 0;
//                             continue;
//                         }
//                     } else {
//                         let status = response.status();
//                         let error_body = response.text().await.unwrap_or_default();
//                         println!("Error body from gemini {}", error_body);
//                         last_error = Some(anyhow::anyhow!("HTTP {}: {}", status, error_body));

//                         if status == StatusCode::TOO_MANY_REQUESTS
//                             || status == StatusCode::INTERNAL_SERVER_ERROR
//                         {
//                             // More aggressive backoff for rate limits
//                             let backoff = INITIAL_BACKOFF * 2u64.pow(chunk_retry_count);
//                             println!(
//                                 "Rate limited. Waiting {} seconds before retry (attempt {}/{})...",
//                                 backoff,
//                                 chunk_retry_count + 1,
//                                 MAX_RETRIES
//                             );
//                             tokio::time::sleep(Duration::from_secs(backoff)).await;
//                             chunk_retry_count += 1;
//                         } else {
//                             break;
//                         }
//                     }
//                 }
//                 Err(e) => {
//                     last_error = Some(anyhow::anyhow!("Request failed: {}", e));
//                     break;
//                 }
//             }
//         }

//         if !chunk_processed {
//             return Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Failed to process chunk")));
//         }

//         // Add a small delay between chunks to avoid hitting rate limits
//         tokio::time::sleep(Duration::from_secs(2)).await;
//     }

//     if all_results.len() == blogs.len() {
//         Ok(all_results)
//     } else {
//         Err(anyhow::anyhow!("Failed to process all blogs"))
//     }
// }

pub async fn embed_blog(blogs: Vec<&str>) -> Result<Vec<Vec<f32>>> {
    dotenv().ok();

    let connection = sqlite::open("urls.db")?;

    // Define all available API keys
    let api_keys = [
        env::var("GEMINI_API_KEY_1").expect("GEMINI_API_KEY_1 not set"),
        env::var("GEMINI_API_KEY_2").expect("GEMINI_API_KEY_2 not set"),
        env::var("GEMINI_API_KEY_3").expect("GEMINI_API_KEY_3 not set"),
        env::var("GEMINI_API_KEY_4").expect("GEMINI_API_KEY_4 not set"),
        env::var("GEMINI_API_KEY_5").expect("GEMINI_API_KEY_5 not set"),
        env::var("GEMINI_API_KEY_6").expect("GEMINI_API_KEY_6 not set"),
    ];
    let mut current_key_index = 0;
    let embeding_url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-embedding-001:batchEmbedContents";

    const MAX_RETRIES: u32 = 5;
    const INITIAL_BACKOFF: u64 = 10;
    const BATCH_SIZE: usize = 1;

    let mut all_results = Vec::with_capacity(blogs.len());

    for chunk in blogs.chunks(BATCH_SIZE) {
        println!("need_pro{}", need_processing);
        let mut chunk_retry_count = 0;
        let mut chunk_processed = false;
        let mut last_error = None;

        if need_processing {
            while chunk_retry_count <= MAX_RETRIES && !chunk_processed {
                // Rotate API key if we've hit max retries with current key
                if chunk_retry_count >= MAX_RETRIES - 1 && chunk_retry_count % MAX_RETRIES == 0 {
                    current_key_index = (current_key_index + 1) % api_keys.len();
                    println!("Rotating to API key {}", current_key_index + 1);
                    chunk_retry_count = 0; // Reset retry counter for new key
                }

                let mut headers = HeaderMap::new();
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                headers.insert(
                    "x-goog-api-key",
                    HeaderValue::from_str(&api_keys[current_key_index]).expect("Invalid API key"),
                );

                let requests: Vec<_> = chunk
                    .iter()
                    .map(|blog| {
                        json!({
                            "model": "models/text-embedding-005",
                            "task_type": "SEMANTIC_SIMILARITY",
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
                match client
                    .post(embeding_url)
                    .headers(headers)
                    .json(&request_body)
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            let response_body = response.json::<Value>().await?;
                            if let Some(embeddings) = response_body["embeddings"].as_array() {
                                let mut results = Vec::with_capacity(embeddings.len());
                                for embedding in embeddings {
                                    if let Some(values) = embedding["values"].as_array() {
                                        let mut vec = Vec::with_capacity(values.len());
                                        for value in values {
                                            if let Some(val) = value.as_f64() {
                                                vec.push(val as f32);
                                            }
                                        }
                                        results.push(vec);
                                    }
                                }
                                all_results.extend(results);
                                chunk_processed = true;
                                continue;
                            }
                        } else {
                            let status = response.status();
                            let error_body = response.text().await.unwrap_or_default();
                            println!(
                                "Error from API key {}: HTTP {}: {}",
                                current_key_index + 1,
                                status,
                                error_body
                            );
                            last_error = Some(anyhow::anyhow!("HTTP {}: {}", status, error_body));

                            if status == StatusCode::TOO_MANY_REQUESTS
                                || status == StatusCode::INTERNAL_SERVER_ERROR
                            {
                                let backoff =
                                    INITIAL_BACKOFF * 2u64.pow(chunk_retry_count % MAX_RETRIES);
                                println!(
                                    "Rate limited on key {}. Waiting {} seconds before retry (attempt {}/{})...",
                                    current_key_index + 1,
                                    backoff,
                                    (chunk_retry_count % MAX_RETRIES) + 1,
                                    MAX_RETRIES
                                );
                                tokio::time::sleep(Duration::from_secs(backoff)).await;
                            }
                        }
                    }
                    Err(e) => {
                        last_error = Some(anyhow::anyhow!("Request failed: {}", e));
                    }
                }
                chunk_retry_count += 1;
            }
        }

        if !chunk_processed {
            return Err(last_error.unwrap_or_else(|| {
                anyhow::anyhow!("Failed to process chunk after exhausting all API keys")
            }));
        }

        // Small delay between chunks
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    Ok(all_results)
}

pub async fn bactch_embeding(all_blogs: &Vec<QdrantdbObject>, client: &Qdrant) -> Result<()> {
    const BATCH_SIZE: usize = 1;
    let mut counter = 0;

    for blogs in all_blogs.chunks(BATCH_SIZE) {
        counter += 1;
        print!("Batch:{}", counter);
        // create a vec of emb that will be feeded to the emebding

        let blog_contents: Vec<&str> = blogs.iter().map(|e| e.content.as_str()).collect();
        let embeding = embed_blog(blog_contents).await?;

        // creating the vec of points here

        let points = blogs
            .iter()
            .zip(embeding)
            .map(|(blog, embedding)| create_point_struct(blog, &embedding))
            .collect::<Vec<_>>();

        //    Insert it to qdrnt

        insert_to_qdrant(&client, points).await?;

        tokio::time::sleep(Duration::from_millis(200)).await
    }

    Ok(())
    //
}

fn create_point_struct(blog: &QdrantdbObject, embedding: &[f32]) -> PointStruct {
    PointStruct::new(
        blog.id.clone(),
        embedding.to_vec(),
        [
            ("title", blog.metadata.title.as_str().into()),
            ("author", blog.metadata.author.as_str().into()),
            ("date", blog.metadata.date.as_str().into()),
            ("publisher", blog.metadata.publisher.as_str().into()),
        ],
    )
}
async fn insert_to_qdrant(client: &Qdrant, points: Vec<PointStruct>) -> anyhow::Result<()> {
    let result = client
        .upsert_points(UpsertPointsBuilder::new("lblogs", points).wait(true))
        .await?;
    println!("QD_UPSERTING_RESULT:{:?}", result);

    Ok(())
}

pub fn generate_content_id(blog_content: &str) -> String {
    // Generate SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(blog_content);
    let hash = hasher.finalize();

    // Convert hash to a fixed-size array and create UUID v4
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&hash[..16]);
    Uuid::from_bytes(bytes).to_string()
}

pub fn already_embeded(connection: &Connection, hash: &str) -> Result<bool> {
    let query = "SELECT 1 FROM urls WHERE hash = ?";
    let mut statement = connection.prepare(query)?;
    statement.bind((1, hash))?;

    let exists = statement.next()? == sqlite::State::Row;

    Ok(exists)
}

pub fn store_content_hash(connection: &Connection, url_hash: &str) -> Result<()> {
    let query = "INSERT INTO urls (hash) VALUES (?)";
    let mut statement = connection.prepare(query)?;
    statement.bind((1, url_hash))?;

    statement.next();
    Ok(())
}
