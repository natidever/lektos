use anyhow::Error;
use lektos::utils::find_feeds::parse_feed;
// use warc::Record;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use warc::WarcHeader;
use warc::WarcReader;

use crate::extractors::pipeline::MetadataPipeline;
use crate::models::blog::Blog;
use crate::models::metadata;
use crate::utils::embed::generate_embedding;
use crate::utils::find_blog_url::is_blog_url;
use crate::utils::find_feeds::extract_feed;
use crate::utils::find_feeds::is_feed;
use crate::utils::html_utils::BlogProcessor;

mod extractors;
mod models;
mod utils;
use std::io::Write;

// Import Embedding and Embeddings from the 'embed' module
use anyhow::Result;

fn main() -> Result<(), Error> {
    // 1. Read the XML file
    let content = std::fs::read_to_string("src/resources/medium.xml")?;
    
    // 2. Process the feed
    let results = extract_feed(&content).unwrap();
    
    // 3. Print the results
    println!("Found {} items:", results.len());
    for (i, item) in results.iter().enumerate() {
        println!("\nItem {}:", i + 1);
        println!("Title: {}", item.title);
        println!("Author: {}", item.author);
        // println!("Description: {}", item.description);
        println!("Date: {}", item.date);
        // println!("Publisher: {}", item.publisher);
        println!("URL: {}", item.url);
    }
    
    Ok(())
}

// fn main() -> io::Result<()> {
//     // PSEUDO-CODE - CONCEPTUAL IMPLEMENTATION

//     use rayon::prelude::*;
//     use std::sync::atomic::{AtomicUsize, Ordering};

//     println!("Extracting  blogs from WARC file...");
//     // let warc_name = "src/resources/2025-26.warc";
//     let warc_name =
//         "src/common_crawl_2025-26_warcfiles/CC-MAIN-20250612112840-20250612142840-00003.warc.gz";

//     let mut reader = WarcReader::from_path_gzip(warc_name)?;

//     let mut stream_iter = reader.stream_records();
//     // let mut records = Vec::new();

//     // Use a loop with explicit scoping

//     // let mut strea_iter=reader::stream_recored();

//     let mut blog_count = 0;
//     const MAX_BLOGS: usize = 500;

//     let mut confirmed_blogs = 0;

//     while let Some(record_result) = stream_iter.next_item() {
//         let record = match record_result {
//             Ok(r) => r,
//             Err(e) => {
//                 eprintln!("Error reading record: {}", e);
//                 continue;
//             }
//         };

//         // Extract URL
//         let url = record
//             .header(WarcHeader::TargetURI)
//             .map(|s| s.to_string())
//             .unwrap_or_default();

//         if !is_blog_url(&url) {
//             // let mut file = OpenOptions::new()
//             // .append(true)  // Open in append mode
//             // .create(true)  // Create the file if it doesn't exist
//             // .open("rejected_url.html")?;
//             // writeln!(file, "{}", url)?;

//             continue;
//         }
//         //   let mut file = OpenOptions::new()
//         // .append(true)  // Open in append mode
//         // .create(true)  // Create the file if it doesn't exist
//         // .open("accepted_url.html")?;
//         // writeln!(file, "{}", url)?;

//         // Check WARC type is response (contains actual content)
//         if record.header(WarcHeader::WarcType).map(|s| s.to_string())
//             != Some("response".to_string())
//         {
//             continue;
//         }

//         // Process content
//         match record.into_buffered() {
//             Ok(buffered) => {
//                 let body = buffered.body();

//                 // Extract the actual HTML from the HTTP response
//                 if let Some(html_start) = find_html_start(body) {
//                     let html = &body[html_start..];
//                     let html_content = String::from_utf8_lossy(&html);
//                     let file_name = format!("sub_sblog_{}.html", blog_count + 1);
//                     // println!("URL {}", url);

//                     // if url.contains(".substack"){
//                     // fs::write(file_name, &html).expect("Failed to write HTML preview");

//                     // }

//                     // fs::write(file_name, &html).expect("Failed to write HTML preview");
//                     let pipeline = MetadataPipeline::new();
//                     // reading html from file (test)
//                     // let file_html= fs::read_to_string(html_content.as_ref()).expect("Failed to read file");
//                     let file_html = html_content.to_string();

//                     // println!("Extracting from HTML  {}",file_html);

//                     // let metadata = pipeline.run(file_html.as_ref());
//                     // let blog_content = BlogProcessor::extract_and_sanitize(file_html.as_ref());
//                     // println!("MetaDataEXTRACTED: {:?}", metadata);

//                     // println!("=== Blog #{} ===", blog_count + 1);
//                     // println!("Content preview {}",preview);
//                     // println!("{}", preview);
//                     // println!("-----");

//                     blog_count += 1;
//                     if blog_count >= MAX_BLOGS {
//                         break;
//                     }
//                 } else {
//                     println!("Found nonhtml in: {}", url);

//                     let string_content = String::from_utf8_lossy(body);
//                     fs::write("string_contnet.html", string_content.as_ref())
//                         .expect("Failed to write string content to file");

//                     // here we can check if it is Rss/atoms
//                     if is_feed(string_content.as_ref()) {
//                         println!("Found feed in: {}", url);
//                         let mut file = OpenOptions::new()
//                             .append(true) // Open in append mode
//                             .create(true) // Create the file if it doesn't exist
//                             .open("feed_urls.html")?;
//                         let feed = parse_feed(string_content.as_ref());

//                         if let Some(feed) = feed.ok() {
//                             writeln!(file, "{}", url)?;
//                             // println!("Feed title: {}", feed.title.unwrap_or_default().content);
//                         } else {
//                             println!("Failed to parse feed from: {}", url);
//                         }

//                         let mut file = OpenOptions::new()
//                             .append(true) // Open in append mode
//                             .create(true) // Create the file if it doesn't exist
//                             .open("non_html_files.html")?;
//                         writeln!(file, "{}", url)?;
//                     }
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Error buffering record: {}", e);
//             }
//         }
//     }

//     println!("Extracted {} Medium blogs", blog_count);
//     Ok(())
// }

// Finds the start of HTML content in HTTP response
fn find_html_start(body: &[u8]) -> Option<usize> {
    // Look for end of HTTP headers (blank line)
    let header_end = body
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|pos| pos + 4)
        .or_else(|| {
            body.windows(2)
                .position(|w| w == b"\n\n")
                .map(|pos| pos + 2)
        })?;

    // Look for HTML tags after headers
    let html_tag_start = body[header_end..]
        .windows(5)
        .position(|w| w.eq_ignore_ascii_case(b"<html") || w.eq_ignore_ascii_case(b"<!doc"))?;

    Some(header_end + html_tag_start)
}
