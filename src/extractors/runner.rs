// this will expose a funcion to that access a path to warc file and process it

use std::fs;

use anyhow::Result;
use warc::{WarcHeader, WarcReader};

use crate::{
    extractors::pipeline::MetadataPipeline,
    models::blog::Blog,
    utils::{
        analysis::{BlogLog, log_blog_to_csv},
        find_blog_url::is_blog_url,
        find_feeds::{extract_url, is_feed},
        html_utils::BlogProcessor,
        url_visit_check::UrlVisitTracker,
        valid_url_from_feeds::FeedUrlValidator,
    },
};

pub fn core_extractor_runner(warc_path: &str) -> Result<()> {
    let vist_url_tracker = UrlVisitTracker::new();

    let feed_url_validator = FeedUrlValidator::new()?;

    // let warc_path =
    //     "src/common_crawl_2025-26_warcfiles/CC-MAIN-20250612112840-20250612142840-00001.warc.gz";

    let mut reader = WarcReader::from_path_gzip(warc_path).unwrap();

    let mut stream_iter = reader.stream_records();

    let mut blog_count = 0;
    const MAX_BLOGS: usize = 500;

    while let Some(record_result) = stream_iter.next_item() {
        let record = match record_result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error reading record: {}", e);
                continue;
            }
        };
        // Extract URL
        let url = record
            .header(WarcHeader::TargetURI)
            .map(|s| s.to_string())
            .unwrap_or_default();
        println!("URL:{}", url);

        // Check WARC type is response (contains actual content) which is the html
        // if record.header(WarcHeader::WarcType).map(|s| s.to_string())
        //     != Some("response".to_string())
        // {
        //     continue;
        // }
        // if vist_url_tracker.is_url_visited(&url) {
        //     continue;
        // }

        // if feed_url_validator.is_from_feed(&url)? {
        //     match record.into_buffered() {
        //         Ok(buffered) => {
        //             let body = buffered.body();

        //             // Extract the actual HTML from the HTTP response
        //             if let Some(html_start) = BlogProcessor::find_html_start(body) {
        //                 blog_count += 1;
        //                 if blog_count >= MAX_BLOGS {
        //                     break;
        //                 }

        //                 let html = &body[html_start..];
        //                 let html_content = String::from_utf8_lossy(&html);
        //                 let file_html = html_content.to_string();
        //                 let pipeline = MetadataPipeline::new();
        //                 let metadata = pipeline.run(file_html.as_str());
        //                 let blog_content = BlogProcessor::extract_and_sanitize(file_html.as_str());

        //                 let blog = Blog {
        //                     title: metadata.title.map(|f| f.value).unwrap_or("Untitled".into()),
        //                     author: metadata.author.map(|f| f.value).unwrap_or("Unknown".into()),

        //                     date: metadata.date.map(|f| f.value).unwrap_or("".into()),
        //                     publisher: metadata.publisher.map(|f| f.value).unwrap_or_default(),
        //                     content: blog_content,
        //                 };

        //                 let log = BlogLog {
        //                     url: url,
        //                     title: blog.title.clone(),
        //                     author: blog.author.clone(),
        //                     date: blog.date.clone(),
        //                     publisher: blog.publisher.clone(),
        //                     word_count: blog.content.split_whitespace().count(),
        //                 };
        //                 println!("immediate above function");
        //                 log_blog_to_csv(&log, "src/blog_log.csv")?;
        //             } else {
        //                 eprintln!("No HTML content found in record for URL: {}", url);

        //                 continue;
        //             }
        //         }
        //         Err(e) => {
        //             eprintln!("Error buffering record: {}", e);
        //         }
        //     }
        // } else {
        //     if is_blog_url(&url) {
        //         match record.into_buffered() {
        //             Ok(buffered) => {
        //                 let body = buffered.body();

        //                 // Extract the actual HTML from the HTTP response
        //                 if let Some(html_start) = BlogProcessor::find_html_start(body) {
        //                     blog_count += 1;
        //                     if blog_count >= MAX_BLOGS {
        //                         break;
        //                     }

        //                     let html = &body[html_start..];
        //                     let html_content = String::from_utf8_lossy(&html);
        //                     let file_html = html_content.to_string();
        //                     let pipeline = MetadataPipeline::new();
        //                     let metadata = pipeline.run(file_html.as_str());
        //                     let blog_content =
        //                         BlogProcessor::extract_and_sanitize(file_html.as_str());

        //                     let mut blog = Blog {
        //                         title: metadata.title.map(|f| f.value).unwrap_or("Untitled".into()),
        //                         author: metadata
        //                             .author
        //                             .map(|f| f.value)
        //                             .unwrap_or("Unknown".into()),

        //                         date: metadata.date.map(|f| f.value).unwrap_or("".into()),
        //                         publisher: metadata.publisher.map(|f| f.value).unwrap_or_default(),
        //                         content: blog_content,
        //                     };
        //                     let log = BlogLog {
        //                         url: url,
        //                         title: blog.title.clone(),
        //                         author: blog.author.clone(),
        //                         date: blog.date.clone(),
        //                         publisher: blog.publisher.clone(),
        //                         word_count: blog.content.split_whitespace().count(),
        //                     };
        //                     log_blog_to_csv(&log, "src/blog_log.csv")?;
        //                 } else {
        //                     let body = buffered.body();

        //                     let str_body = String::from_utf8_lossy(body);

        //                     if is_feed(&str_body) {
        //                         println!("Yes it is feed{}", url);

        //                         let valid_urls = extract_url(&str_body)?;

        //                         let urls = valid_urls.join("\n");
        //                         fs::write("accepted_url.html", urls)?;
        //                     }
        //                     eprintln!("No HTML content found in record for URL: {}", url);
        //                     continue;
        //                 }
        //             }
        //             Err(e) => {
        //                 eprintln!("Error buffering record: {}", e);
        //             }
        //         }
        //     } else {
        //         // do nothin
        //         // the url is not blog
        //     }
        // }
    }

    Ok(())
}
