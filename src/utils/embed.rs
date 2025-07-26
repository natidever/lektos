// src/embedding.rs

use std::env;

use anyhow::Result;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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

pub async fn generate_embedding(text: &str, model_name: &str) -> Result<Vec<f32>> {
    dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY")?;

    println!(
        "DEBUG: [embedding.rs] Starting generate_embedding for text: '{}'",
        text
    );

    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/{}:embedContent",
        model_name
    );
    println!("DEBUG: [embedding.rs] Target API URL: {}", api_url);

    let client = Client::new();
    println!("DEBUG: [embedding.rs] reqwest client built.");

    let request_body = EmbedRequest {
        model: model_name.to_string(),
        content: EmbedContent {
            parts: vec![EmbedContentPart {
                text: text.to_string(),
            }],
        },
    };
    println!(
        "DEBUG: [embedding.rs] Request payload prepared: {:?}",
        request_body
    );

    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .header("x-goog-api-key", api_key)
        .json(&request_body)
        .send()
        .await?;

    println!("DEBUG: [embedding.rs] Received HTTP response from API.");

    let status = response.status();
    println!("DEBUG: [embedding.rs] HTTP Status Code: {}", status);

    if status.is_success() {
        let response_text = response.text().await?;
        let response_body: EmbedResponse = serde_json::from_str(&response_text)?;
        println!("DEBUG: [embedding.rs] Response body parsed successfully.");

        if let Some(embedding) = response_body.embedding {
            println!("DEBUG: [embedding.rs] Embedding found in response.");
            println!(
                "  Embedding Values (first 5): {:?}",
                &embedding.values[0..5]
            );
            println!("  Dimensionality: {}", embedding.values.len());
            Ok(embedding.values)
        } else {
            eprintln!(
                "DEBUG: [embedding.rs] 'embedding' field was missing or null in the successful response."
            );
            anyhow::bail!(
                "API response successful, but no embedding found. Full response: {}",
                response_text
            );
        }
    } else {
        let error_text = response.text().await?;
        eprintln!(
            "Error: [embedding.rs] API request failed with status {}. Response: {}",
            status, error_text
        );
        anyhow::bail!("API request failed: Status {} - {}", status, error_text);
    }
}

pub async fn handle_embedding(text: &str, model: &str) -> Result<Vec<f32>> {
    match generate_embedding(text, model).await {
        Ok(embedding) => {
            println!(
                " Successfully obtained embedding with dimensionality: {}",
                embedding.len()
            );

            println!(
                "🔍 First 5 values: {:?}",
                &embedding[0..5.min(embedding.len())]
            );
            Ok(embedding)
        }
        Err(e) => Err(e.into()),
    }
}

use anyhow::Context;

use crate::{models::blog::Blog, utils::analysis::BlogResult};

pub async fn generate_embeddings_batch(
    texts: Vec<String>,
    model_name: &str,
) -> Result<Vec<Vec<f32>>> {
    dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").context("Missing GEMINI_API_KEY")?;
    let client = Client::new();

    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:batchEmbedContents",
        model_name
    );
    let batch_size = 50;
    let mut all_embeddings = Vec::with_capacity(texts.len());

    for chunk in texts.chunks(batch_size) {
        let mut retries = 0;
        let mut success = false;

        while retries < 3 && !success {
            let response =
                execute_batch_request(&client, &api_url, &api_key, chunk, model_name).await;

            match response {
                Ok(embeddings) => {
                    all_embeddings.extend(embeddings);
                    success = true;
                    println!("Processed batch of {} texts", chunk.len());
                }
                Err(e) => {
                    eprintln!("Batch error (retry {}): {}", retries + 1, e);
                    retries += 1;
                    tokio::time::sleep(std::time::Duration::from_secs(2 * retries as u64)).await;
                }
            }
        }

        if !success {
            anyhow::bail!("Failed to process batch after 3 retries");
        }
    }

    Ok(all_embeddings)
}

async fn execute_batch_request(
    client: &Client,
    api_url: &str,
    api_key: &str,
    texts: &[String],
    model_name: &str,
) -> Result<Vec<Vec<f32>>> {
    let requests = texts
        .iter()
        .map(|text| EmbedRequest {
            model: format!("models/{}", model_name),
            content: EmbedContent {
                parts: vec![EmbedContentPart {
                    text: text.to_string(),
                }],
            },
        })
        .collect::<Vec<_>>();

    let request_body = serde_json::json!({ "requests": requests });

    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("x-goog-api-key", api_key)
        .json(&request_body)
        .send()
        .await
        .context("API request failed")?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        anyhow::bail!("API error {}: {}", status, error_text);
    }

    let response_body: serde_json::Value = response.json().await.context("JSON parse failed")?;

    response_body["embeddings"]
        .as_array()
        .context("Invalid response format")?
        .iter()
        .map(|item| {
            item["values"]
                .as_array()
                .context("Missing values array")?
                .iter()
                .map(|v| {
                    v.as_f64()
                        .map(|f| f as f32)
                        .context("Invalid embedding value")
                })
                .collect::<Result<Vec<f32>>>()
        })
        .collect()
}

pub async fn process_blogs(blogs: Vec<Blog>, model_name: &str) -> Vec<EmbedingResult> {
    let texts = blogs.iter().map(|blog| blog.to_embedding_text()).collect();

    match generate_embeddings_batch(texts, model_name).await {
        Ok(embeddings) => blogs
            .into_iter()
            .zip(embeddings)
            .map(|(blog, embedding)| EmbedingResult {
                title: blog.title,
                embedding,
                status: "success".to_string(),
            })
            .collect(),
        Err(e) => {
            eprintln!("Embedding failed: {}", e);
            blogs
                .into_iter()
                .map(|blog| EmbedingResult {
                    title: blog.title,
                    embedding: Vec::new(),
                    status: format!("failed: {}", e),
                })
                .collect()
        }
    }
}

struct EmbedingResult {
    title: String,
    embedding: Vec<f32>,
    status: String,
}
