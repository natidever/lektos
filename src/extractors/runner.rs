// this will expose a funcion to that access a path to warc file and process it

use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, BufWriter},
    sync::Arc, u8,
};

use anyhow::{ Result};
use arrow::{
    array::{ArrayRef, RecordBatch, StringArray},
    datatypes::{DataType, Field, Schema}, ipc::writer::StreamWriter,
};
use dotenv::dotenv;
use pyo3::{ffi::PyByteArrayObject, prelude::*, types::PyBytes};
use qdrant_client::{Qdrant, qdrant::PointsOperationResponse};
use warc::{Record, StreamingBody, WarcHeader, WarcReader};

use std::ops::ControlFlow;

use crate::{
    extractors::pipeline::MetadataPipeline,
    models::blog::Blog,
    utils::{
        analysis::{BlogLog, log_blog_to_csv},
        embed::{DbMetadata, QdrantdbObject, Summary, bactch_embeding, generate_content_id},
        find_blog_url::is_blog_url,
        find_feeds::{extract_url, is_feed},
        html_utils::BlogProcessor,
        url_visit_check::UrlVisitTracker,
        valid_url_from_feeds::FeedUrlValidator,
    },
};


use std::net::TcpListener;


pub  fn extractor_runner(py:Python<'_>,warc_path: &str) -> Result<Vec<u8>>{

        
        
     
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

        // if is_blog_url(&url) {
        //         proccess_and_push(record, &mut blog_to_embed, &url);
        //     } else {
        //         // do nothin
        //         // the url is not blog
        //     }
            
        if vist_url_tracker.is_url_visited(&url) {
            println!("Visited URL:{}",&url);

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

    // alredy got the schema
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, true),
        Field::new("content", DataType::Utf8, true),
        Field::new("url", DataType::Utf8, true),
        Field::new("image_url", DataType::Utf8, true),
        Field::new("title", DataType::Utf8, true),
        Field::new("author", DataType::Utf8, true),
        Field::new("date", DataType::Utf8, true),
        Field::new("publisher", DataType::Utf8, true),
    ]));

    use arrow::array::{ArrayRef, StringArray};
    use std::sync::Arc;

    let id: ArrayRef = Arc::new(StringArray::from_iter_values(
        blog_to_embed.iter().map(|b| b.id.as_str()),
    ));

    let content: ArrayRef = Arc::new(StringArray::from_iter_values(
        blog_to_embed.iter().map(|b| b.content.as_str()),
    ));
    let url: ArrayRef = Arc::new(StringArray::from_iter_values(
        blog_to_embed.iter().map(|b| b.metadata.url.as_str()),
    ));
    let image_url: ArrayRef = Arc::new(StringArray::from_iter_values(
        blog_to_embed.iter().map(|b| b.metadata.image_url.as_str()),
    ));
    let title: ArrayRef = Arc::new(StringArray::from_iter_values(
        blog_to_embed.iter().map(|b| b.metadata.title.as_str()),
    ));
    let author: ArrayRef = Arc::new(StringArray::from_iter_values(
        blog_to_embed.iter().map(|b| b.metadata.author.as_str()),
    ));
    let date: ArrayRef = Arc::new(StringArray::from_iter_values(
        blog_to_embed.iter().map(|b| b.metadata.date.as_str()),
    ));
    let publisher: ArrayRef = Arc::new(StringArray::from_iter_values(
        blog_to_embed.iter().map(|b| b.metadata.publisher.as_str()),
    ));

    let arrays: Vec<ArrayRef> = vec![id, content, url, image_url, title, author, date, publisher];

    let record_batch =
        RecordBatch::try_new(schema.clone(), arrays).unwrap();
    // println!("Real batched:{:?}", record_batch);

    // trying to send for python 
    let mut buffer = Vec::new();
    let mut object_reciver = StreamWriter::try_new(&mut buffer, &schema).unwrap();
    {
    object_reciver.write(&record_batch);
    object_reciver.finish();

    }


    Ok(buffer)




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

        let blog_content = BlogProcessor::extract_and_sanitize(file_html.as_str());
        // Optimistic validation even if the url is identified as blog there is a big chance the url is not blog 
        // from the analysis it is rear the url is not blog if the title and the author is found so the url is pushed 
        // if the title and the author is found 
        if metadata.author.is_some() && metadata.title.is_some(){
            let qdrant_object: QdrantdbObject = QdrantdbObject {
            id: generate_content_id(&blog_content),
            content: blog_content,
            metadata: DbMetadata {
                url: url.to_lowercase(),
                image_url: metadata
                    .image_url
                    .map(|f| f.value)
                    .unwrap_or("Undefined".into()),
                title: metadata.title.map(|f| f.value).unwrap_or("Untitled".into()),
                author: metadata.author.map(|f| f.value).unwrap_or("Unknown".into()),
                date: metadata.date.map(|f| f.value).unwrap_or("".into()),
                publisher: metadata.publisher.map(|f| f.value).unwrap_or_default(),
            },
        };
        blog_to_embed.push(qdrant_object);

        } 
        
        

        return Ok(ControlFlow::Continue(()));
    } else {
        return Ok(ControlFlow::Continue(()));
    }
}
