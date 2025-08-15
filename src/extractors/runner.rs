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

// This core extractor is used to store embeding and  qdrant db
// or just the right way to communicate in the python side is to return analysis
// like {
// "blog_analyzed":320,
// "successsfully_stored_in_qdrant":410
// "failed_to_store_in_db":2
// "storage_area":"path/to/data/"
// }

pub  fn extractor_runner(py:Python<'_>,warc_path: &str) -> Result<Vec<u8>>{
    // PyResult<Py<PyBytes>>{
    let blog_to_embed: Vec<QdrantdbObject> = vec![
        QdrantdbObject {
            id: "abc123".to_string(),
            content: "This is the first blog content.".to_string(),
            metadata: DbMetadata {
                url: "https://example.com/blog1".to_string(),
                image_url: "https://example.com/img1.jpg".to_string(),
                title: "First Blog".to_string(),
                author: "Alice".to_string(),
                date: "2025-08-14".to_string(),
                publisher: "Publisher A".to_string(),
            },
        },
        QdrantdbObject {
            id: "def456".to_string(),
            content: "Second blog has different content.".to_string(),
            metadata: DbMetadata {
                url: "https://example.com/blog2".to_string(),
                image_url: "https://example.com/img2.jpg".to_string(),
                title: "Second Blog".to_string(),
                author: "Bob".to_string(),
                date: "2025-08-13".to_string(),
                publisher: "Publisher B".to_string(),
            },
        },
        QdrantdbObject {
            id: "ghi789".to_string(),
            content: "Third blog content is here.".to_string(),
            metadata: DbMetadata {
                url: "https://example.com/blog3".to_string(),
                image_url: "https://example.com/img3.jpg".to_string(),
                title: "Third Blog".to_string(),
                author: "Charlie".to_string(),
                date: "2025-08-12".to_string(),
                publisher: "Publisher C".to_string(),
            },
        },
        QdrantdbObject {
            id: "jkl012".to_string(),
            content: "Fourth blog with some unique content.".to_string(),
            metadata: DbMetadata {
                url: "https://example.com/blog4".to_string(),
                image_url: "https://example.com/img4.jpg".to_string(),
                title: "Fourth Blog".to_string(),
                author: "Dana".to_string(),
                date: "2025-08-11".to_string(),
                publisher: "Publisher D".to_string(),
            },
        },
    ];

    dotenv().ok();

    // let mut blog_to_embed: Vec<QdrantdbObject> = Vec::new();

    // let vist_url_tracker = UrlVisitTracker::new();

    // let feed_url_validator = FeedUrlValidator::new()?;

    // let mut reader = WarcReader::from_path_gzip(warc_path)?;

    // let mut stream_iter = reader.stream_records();

    // let mut blog_count = 0;
    // const MAX_BLOGS: usize = 500;

    // let mut urls: Vec<String> = Vec::new();

    // while let Some(record_result) = stream_iter.next_item() {
    //     let record = record_result?;
    //     // Extract URL
    //     let url = record
    //         .header(WarcHeader::TargetURI)
    //         .map(|s| s.to_string())
    //         .unwrap_or_default();

    //     // Check WARC type is response (contains actual content) which is the html
    //     if record.header(WarcHeader::WarcType).map(|s| s.to_string())
    //         != Some("response".to_string())
    //     {
    //         continue;
    //     }
    //     if vist_url_tracker.is_url_visited(&url) {
    //         continue;
    //     }

    //     if feed_url_validator.is_from_feed(&url)? {
    //         // the url is blog since it came from feeds
    //         proccess_and_push(record, &mut blog_to_embed, &url);
    //     } else {
    //         if is_blog_url(&url) {
    //             proccess_and_push(record, &mut blog_to_embed, &url);
    //         } else {
    //             // do nothin
    //             // the url is not blog
    //         }
    //     }
    // }

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
    println!("Real batched:{:?}", record_batch);

    // trying to send for python 
    let mut buffer = Vec::new();
    let mut object_reciver = StreamWriter::try_new(&mut buffer, &schema).unwrap();
    {
    object_reciver.write(&record_batch);
    object_reciver.finish();

    }


    Ok(buffer)



    // let client = Qdrant::from_url(&quadrant_url)
    //     .api_key(quadrant_api)
    //     .build()
    //     .unwrap();
    // // batch embeding should return what it has already done right ?
    // let result = bactch_embeding(&blog_to_embed, &client).await?;

    // Ok(result)
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

        let qdrant_object = QdrantdbObject {
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
        return Ok(ControlFlow::Continue(()));
    } else {
        return Ok(ControlFlow::Continue(()));
    }
}
