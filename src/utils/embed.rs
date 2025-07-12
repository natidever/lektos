// src/embedding.rs

use reqwest::Client;
use serde::{Serialize, Deserialize};
use anyhow::Result;


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
    pub content: EmbedContent,
}

#[derive(Debug, Deserialize)]
pub struct EmbeddingValues {
    pub values: Vec<f32>,
}

#[derive(Debug, Deserialize)]
pub struct EmbedResponse {
    pub embedding: Option<EmbeddingValues>,
}


pub async fn generate_embedding(
    text: String,
    api_key: &str,
    model_name: &str,
) -> Result<Vec<f32>> {
    println!("DEBUG: [embedding.rs] Starting generate_embedding for text: '{}'", text);

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
            parts: vec![EmbedContentPart { text: text.clone() }],
        },
    };
    println!("DEBUG: [embedding.rs] Request payload prepared: {:?}", request_body);

    let response = client.post(&api_url)
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
        println!("  Embedding Values (first 5): {:?}", &embedding.values[0..5]);
        println!("  Dimensionality: {}", embedding.values.len());
        Ok(embedding.values)
    } else {
        eprintln!("DEBUG: [embedding.rs] 'embedding' field was missing or null in the successful response.");
        anyhow::bail!("API response successful, but no embedding found. Full response: {}", response_text);
    }
} else {
    let error_text = response.text().await?;
    eprintln!("Error: [embedding.rs] API request failed with status {}. Response: {}", status, error_text);
    anyhow::bail!("API request failed: Status {} - {}", status, error_text);
}
}