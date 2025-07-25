use crate::extractors::pipeline::MetadataPipeline;
use crate::models::blog::Blog;
use crate::models::metadata::ExtractionResult;
use crate::models::metadata::FieldResult;
use crate::utils::analysis::BlogLog;
use crate::utils::analysis::log_blog_to_csv;
use crate::utils::embed::handle_embedding;
use crate::utils::find_blog_url::is_blog_url;
use crate::utils::find_feeds::is_feed;
use crate::utils::url_visit_check::UrlVisitTracker;
use lektos::utils::find_feeds::extract_url;
use lektos::utils::html_utils::BlogProcessor;
use lektos::utils::valid_url_from_feeds::FeedUrlValidator;
use scraper::Selector;
use warc::WarcHeader;
use warc::WarcReader;
mod extractors;
mod models;
mod utils;

use std::{env, fs};

use anyhow::Result;

// Finds the start of HTML content in HTTP response

// #[tokio::main]
// pub async fn main() -> Result<()> {
//     for file_number in 1..11 {
//         println!("Processing {:?} file", file_number);
//         let vist_url_tracker = UrlVisitTracker::new();

//         let feed_url_validator = FeedUrlValidator::new()?;

//         // let warc_name =
//         //     "src/common_crawl_2025-26_warcfiles/CC-MAIN-20250612112840-20250612142840-00001.warc.gz";

//         let warc_name = format!(
//             "src/common_crawl_2025-26_warcfiles/CC-MAIN-20250612112840-20250612142840-0000{:?}.warc.gz",
//             file_number
//         );

//         let mut reader = WarcReader::from_path_gzip(warc_name).unwrap();

//         let mut stream_iter = reader.stream_records();

//         let mut blog_count = 0;
//         const MAX_BLOGS: usize = 500;

//         while let Some(record_result) = stream_iter.next_item() {
//             let record = match record_result {
//                 Ok(r) => r,
//                 Err(e) => {
//                     eprintln!("Error reading record: {}", e);
//                     continue;
//                 }
//             };
//             // Extract URL
//             let url = record
//                 .header(WarcHeader::TargetURI)
//                 .map(|s| s.to_string())
//                 .unwrap_or_default();

//             // Check WARC type is response (contains actual content) which is the html
//             if record.header(WarcHeader::WarcType).map(|s| s.to_string())
//                 != Some("response".to_string())
//             {
//                 continue;
//             }
//             if vist_url_tracker.is_url_visited(&url) {
//                 continue;
//             }

//             if feed_url_validator.is_from_feed(&url)? {
//                 match record.into_buffered() {
//                     Ok(buffered) => {
//                         let body = buffered.body();

//                         // Extract the actual HTML from the HTTP response
//                         if let Some(html_start) = BlogProcessor::find_html_start(body) {
//                             blog_count += 1;
//                             if blog_count >= MAX_BLOGS {
//                                 break;
//                             }

//                             let html = &body[html_start..];
//                             let html_content = String::from_utf8_lossy(&html);
//                             let file_html = html_content.to_string();
//                             let pipeline = MetadataPipeline::new();
//                             let metadata = pipeline.run(file_html.as_str());
//                             let blog_content =
//                                 BlogProcessor::extract_and_sanitize(file_html.as_str());

//                             let blog = Blog {
//                                 title: metadata.title.map(|f| f.value).unwrap_or("Untitled".into()),
//                                 author: metadata
//                                     .author
//                                     .map(|f| f.value)
//                                     .unwrap_or("Unknown".into()),

//                                 date: metadata.date.map(|f| f.value).unwrap_or("".into()),
//                                 publisher: metadata.publisher.map(|f| f.value).unwrap_or_default(),
//                                 content: blog_content,
//                             };

//                             let log = BlogLog {
//                                 url: url,
//                                 title: blog.title.clone(),
//                                 author: blog.author.clone(),
//                                 date: blog.date.clone(),
//                                 publisher: blog.publisher.clone(),
//                                 word_count: blog.content.split_whitespace().count(),
//                             };
//                             println!("immediate above function");
//                             log_blog_to_csv(&log, "src/blog_log.csv")?;

//                             // let embedding_text = blog.to_embedding_text();

//                             // let embedding_model = "models/embedding-001";
//                             // let embeding_data=handle_embedding(&embedding_text, embedding_model).await?;
//                         } else {
//                             eprintln!("No HTML content found in record for URL: {}", url);

//                             continue;
//                         }
//                     }
//                     Err(e) => {
//                         eprintln!("Error buffering record: {}", e);
//                     }
//                 }
//             } else {
//                 if is_blog_url(&url) {
//                     match record.into_buffered() {
//                         Ok(buffered) => {
//                             let body = buffered.body();

//                             // Extract the actual HTML from the HTTP response
//                             if let Some(html_start) = BlogProcessor::find_html_start(body) {
//                                 blog_count += 1;
//                                 if blog_count >= MAX_BLOGS {
//                                     break;
//                                 }

//                                 // let html = &body[html_start..];
//                                 //     let html_content = String::from_utf8_lossy(&html);
//                                 //     let file_html = html_content.to_string();
//                                 //     let pipeline = MetadataPipeline::new();
//                                 //     let metadata = pipeline.run(file_html.as_str());
//                                 //     let blog_content =
//                                 //         BlogProcessor::extract_and_sanitize(file_html.as_str());

//                                 //     let mut blog = Blog {
//                                 //         title: metadata.title.map(|f| f.value).unwrap_or("Untitled".into()),
//                                 //         author: metadata
//                                 //             .author
//                                 //             .map(|f| f.value)
//                                 //             .unwrap_or("Unknown".into()),

//                                 //         date: metadata.date.map(|f| f.value).unwrap_or("".into()),
//                                 //         publisher: metadata.publisher.map(|f| f.value).unwrap_or_default(),
//                                 //         content: blog_content,
//                                 //     };
//                                 //           let log = BlogLog {
//                                 //     url: url,
//                                 //     title: blog.title.clone(),
//                                 //     author: blog.author.clone(),
//                                 //     date: blog.date.clone(),
//                                 //     publisher: blog.publisher.clone(),
//                                 //     word_count: blog.content.split_whitespace().count(),
//                                 // };
//                                 // println!("immediate above function");
//                                 // log_blog_to_csv(&log, "src/blog_log.csv")?;

//                                 // EMBED ONLY IF WE HAVE ALL THE DATA

//                                 // let embedding_text = blog.to_embedding_text();

//                                 // let embedding_model = "models/embedding-001";
//                                 // let embeding_data =
//                                 //     handle_embedding(&embedding_text, embedding_model).await?;
//                             } else {
//                                 let body = buffered.body();

//                                 let str_body = String::from_utf8_lossy(body);

//                                 if is_feed(&str_body) {
//                                     println!("Yes it is feed{}", url);

//                                     let valid_urls = extract_url(&str_body)?;

//                                     let urls = valid_urls.join("\n");
//                                     fs::write("accepted_url.html", urls)?;
//                                 }
//                                 eprintln!("No HTML content found in record for URL: {}", url);
//                                 continue;
//                             }
//                         }
//                         Err(e) => {
//                             eprintln!("Error buffering record: {}", e);
//                         }
//                     }
//                 } else {
//                     // do nothin
//                     // the url is not blog
//                 }
//             }
//         }
//     }

//     Ok(())
// }

const COMMON_SELECTOR_MAPPING: [(&str, &str); 5] = [
     ("author", "author"),
    ("title", "title"),
    ("description", "description"),
   
    ("published_time", "date"),
    ("publisher", "publisher"),
];

// const COMMON_SELECTOR_MAPPING: [&str; 5] = [
//      "author",
//     "title",
//     "description",
   
//     "published_time",
//     "publisher"
// ];


fn extract(html: &str) -> ExtractionResult {
    println!("Called");
        let mut extraction_result = ExtractionResult::default();

        let document = scraper::Html::parse_document(html);

        let selector = match Selector::parse("meta[name]") {
            Ok(selector) => selector,
            Err(_) => return extraction_result, // Return empty result on error
        };

        let mut found_count = 0;

        for element in document.select(&selector) {
            let raw_name: &str = element.value().attr("name").unwrap_or("");
            let content = element.value().attr("content").unwrap_or("");
          


            for  (name,field) in COMMON_SELECTOR_MAPPING {
                if name == raw_name {
                    let field_result = create_field_result(content);
                    match field {
                        "title" => extraction_result.title = Some(field_result),
                        "description" => extraction_result.description = Some(field_result),
                        "author" => extraction_result.author = Some(field_result),
                        "date" => extraction_result.date = Some(field_result),
                        "publisher" => extraction_result.publisher = Some(field_result),
                        _ => {}
                    }

                    found_count += 1;
                    break;
                }
            }
        }

        extraction_result.confidence = 0.2 + (0.2 * found_count as f32);
        extraction_result
    }


      

fn create_field_result(value: &str) -> FieldResult {
        FieldResult {
            value: value.to_string(),
            source: "common".to_string(),
        }
    }

fn main (){

    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Example Page Title</title>
    <meta name="author" content="John Doe">
    <meta name="description" content="This is an example webpage description">
    <meta name="publisher" content="Example Publishing">
</head>
<body>
    <h1>Hello, World!</h1>
    <p>This is an example HTML page.</p>
</body>
</html>
"#;
let result  = extract(&html);
println!{"result{:?}",result}
}