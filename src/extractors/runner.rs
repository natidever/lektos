// this will expose a funcion to that access a path to warc file and process it

use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
};

use anyhow::{Ok, Result};
use dotenv::dotenv;
use pyo3::prelude::*;
use qdrant_client::Qdrant;
use warc::{Record, StreamingBody, WarcHeader, WarcReader};

use std::ops::ControlFlow;

use crate::{
    extractors::pipeline::MetadataPipeline,
    models::blog::Blog,
    utils::{
        analysis::{BlogLog, log_blog_to_csv},
        embed::{DbMetadata, QdrantdbObject, bactch_embeding, generate_content_id},
        find_blog_url::is_blog_url,
        find_feeds::{extract_url, is_feed},
        html_utils::BlogProcessor,
        url_visit_check::UrlVisitTracker,
        valid_url_from_feeds::FeedUrlValidator,
    },
};

pub async fn core_extractor_runner(warc_path: &str) -> Result<Vec<String>> {
    dotenv().ok();

    let mut blog_to_embed: Vec<QdrantdbObject> = Vec::new();

    let vist_url_tracker = UrlVisitTracker::new();

    let feed_url_validator = FeedUrlValidator::new()?;

    let mut reader = WarcReader::from_path_gzip(warc_path)?;

    let mut stream_iter = reader.stream_records();

    let mut blog_count = 0;
    const MAX_BLOGS: usize = 500;

    let mut urls: Vec<String> = Vec::new();

    while let Some(record_result) = stream_iter.next_item() {
        let record = record_result?;
        // Extract URL
        let url = record
            .header(WarcHeader::TargetURI)
            .map(|s| s.to_string())
            .unwrap_or_default();

        // Check WARC type is response (contains actual content) which is the html
        if record.header(WarcHeader::WarcType).map(|s| s.to_string())
            != Some("response".to_string())
        {
            continue;
        }
        if vist_url_tracker.is_url_visited(&url) {
            continue;
        }

        if feed_url_validator.is_from_feed(&url)? {
            // the url is blog since it came from feeds
            proccess_and_push(record, &mut blog_to_embed, &url);
        } else {
            if is_blog_url(&url) {
                proccess_and_push(record, &mut blog_to_embed, &url);
            } else {
                // do nothin
                // the url is not blog
            }
        }
    }

    let quadrant_api = env::var("QUADRANT_API_KEY").expect("failed to load quadrant api key");
    let quadrant_url = env::var("QUADRANT_URL").expect("failed to load qdt url");

    let client = Qdrant::from_url(&quadrant_url)
        .api_key(quadrant_api)
        .build()
        .unwrap();
    bactch_embeding(&blog_to_embed, &client).await?;

    Ok(urls)
}













fn proccess_and_push<B: BufRead>(
    record: Record<StreamingBody<'_, B>>,
    blog_to_embed: &mut Vec<QdrantdbObject>,
    url: &str,
) -> Result<ControlFlow<()>> {
    let buffered = record.into_buffered()?;
    let body = buffered.body();

    // Extract the actual HTML from the HTTP response
    

    if let Some(html_start) = BlogProcessor::find_html_start(body) {
        let html = &body[html_start..];
        let html_content = String::from_utf8_lossy(&html);
        let file_html = html_content.to_string();
        let pipeline = MetadataPipeline::new();
        let metadata = pipeline.run(file_html.as_str());

        println!("IMAGE_URL:{:?}",{&metadata.image_url});
        let blog_content = BlogProcessor::extract_and_sanitize(file_html.as_str());

        let qdrant_object = QdrantdbObject {
            id: generate_content_id(&blog_content),
            content: blog_content,
            metadata: DbMetadata {
                url: url.to_lowercase(),
                title: metadata.title.map(|f| f.value).unwrap_or("Untitled".into()),
                author: metadata.author.map(|f| f.value).unwrap_or("Unknown".into()),
                date: metadata.date.map(|f| f.value).unwrap_or("".into()),
                publisher: metadata.publisher.map(|f| f.value).unwrap_or_default(),
            },
        };

        blog_to_embed.push(qdrant_object);
        return Ok(ControlFlow::Continue(()));
    } else {
        // eprintln!("No HTML content found in record for URL: {}", url);

        return Ok(ControlFlow::Continue(()));
    }
}
