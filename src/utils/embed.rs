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
use reqwest::header::CONTENT_TYPE;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;
use std::collections::HashMap;
use sha2::{ Sha256,Digest};

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

pub async fn embed_blog(blogs: Vec<&str>) -> Result<Vec<Vec<f32>>> {
    dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY").expect("failed to get env ");
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
                "model": "models/text-embedding-005",
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

    if response.status().is_success() {
        let response_body = response.json::<Value>().await?;

        let embeddings = response_body["embeddings"].as_array().unwrap();

        // Process each embedding in the batch
        let mut results = Vec::with_capacity(embeddings.len());
        for embedding in embeddings {
            if let Some(values) = embedding["values"].as_array() {
                let mut vec = Vec::with_capacity(values.len());
                for value in values {
                    let val = value
                        .as_f64()
                        .ok_or_else(|| anyhow::anyhow!("Invalid embedding value: {:?}", value))?;
                    vec.push(val as f32);
                }
                results.push(vec);
            }

            for (i, embedding) in results.iter().take(3).enumerate() {
                println!(
                    "Embedding {} ({} dims): {:?}",
                    i,
                    embedding.len(),
                    &embedding[..embedding.len().min(5)]
                );
            }
        }

        return Ok(results);
    } else {
        let error_body = response.text().await?;
        return Err(Error::msg(error_body));
    }
}

pub async fn bactch_embeding(all_blogs: &Vec<QdrantdbObject>) -> Result<()> {
    const BATCH_SIZE: usize = 5;
    let mut counter = 0;
    dotenv().ok();

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
        let quadrant_api = env::var("QUADRANT_API_KEY").expect("failed to load quadrant api key");
        let quadrant_url = env::var("QUADRANT_URL").expect("failed to load qdt url");

        let client = Qdrant::from_url(&quadrant_url)
            .api_key(quadrant_api)
            .build()
            .unwrap();

        insert_to_qdrant(&client, points).await?;

        tokio::time::sleep(Duration::from_millis(20000)).await
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
        .upsert_points(UpsertPointsBuilder::new("lektos_blogs", points).wait(true))
        .await?;
    println!("QD_UPSERTING_RESULT:{:?}", result);

    Ok(())
}

pub fn generate_content_id(blog_content: &String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(blog_content);
    format!("{:x}", hasher.finalize())
}