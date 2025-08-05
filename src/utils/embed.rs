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
use qdrant_client::qdrant::Vectors;
use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};
use qdrant_client::Qdrant;
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

pub struct DbMetadata {
    pub title: String,
    pub author: String,
    pub date: String,
    pub publisher: String,
    pub description: String,
}


pub struct QdrantdbObject {
    id:u64,
    metadata:DbMetadata,
    content:String
}



pub async fn embed_blog(blogs:Vec<&str>) -> Result<Vec<Vec<f32>>> {
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

pub async fn bactch_embeding(all_blogs: &Vec<QdrantdbObject>) -> Result<()> {
    const BATCH_SIZE: usize = 1;
    let mut count = 1;
       dotenv().ok();

    let quadrant_api = env::var("QUADRANT_API_KEY").expect("failed to load quadrant api key");
    let quadrant_url = env::var("QUADRANT_URL").expect("failed to load qdt url");

    let client = Qdrant::from_url(&quadrant_url)
        .api_key(quadrant_api)
        .build()
        .unwrap();

    for blogs in all_blogs.chunks(BATCH_SIZE) {
        count += 1;


        // create a vec of content that will be feeded to the emebding

        let blog_contents:Vec<&str> =blogs.iter().map(|e| e.content.as_str()).collect();
        let embeding = embed_blog(blog_contents).await?;

    
        // creating the points 
         client
    .upsert_points(
        UpsertPointsBuilder::new(
            "{collection_name}",
            vec![
                PointStruct::new(1, vec![0.9, 0.1, 0.1], 
                    [
                        ("city", "red".into())
                        
                        ]
                ),
                PointStruct::new(2, vec![0.1, 0.9, 0.1], [("city", "green".into())]),
                PointStruct::new(3, vec![0.1, 0.1, 0.9], [("city", "blue".into())]),
            ],
        )
        .wait(true),
    )
    .await?;


        

         
        // get the result

        // store it to qdrant
    }

    Ok(())
    //
}

use qdrant_client::qdrant::{Vector,Value as QdrantValue};
use std::collections::HashMap;

fn create_point_struct(blog: &QdrantdbObject, embedding: &[f32]) -> PointStruct {
    PointStruct::new(
        blog.id, 
        embedding.to_vec(),
        [
            ("title", blog.metadata.title.as_str().into()),
            ("author", blog.metadata.author.as_str().into()),
            ("date", blog.metadata.date.as_str().into()),
            ("publisher", blog.metadata.publisher.as_str().into()),
            ("description", blog.metadata.description.as_str().into()),
        ],
    )
}