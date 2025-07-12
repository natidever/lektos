
use warc::WarcHeader;
use warc::WarcReader;
use std::fs;
use std::io;

use crate::extractors::pipeline::MetadataPipeline;
use crate::models::blog::Blog;
use crate::models::metadata;
use crate::utils::html_utils::BlogProcessor;

mod utils;
mod extractors;
mod models;




// fn main() -> io::Result<()> {
//     println!("Extracting Medium blogs from WARC file...");
//     let warc_name = "src/resource.warc.gz";
    
//     let mut reader = WarcReader::from_path_gzip(warc_name)?;
//     let mut stream_iter = reader.stream_records();
    
//     let mut blog_count = 0;
//     const MAX_BLOGS: usize = 2;

//     while let Some(record_result) = stream_iter.next_item() {
//         let record = match record_result {
//             Ok(r) => r,
//             Err(e) => {
//                 eprintln!("Error reading record: {}", e);
//                 continue;
//             }
//         };
        
//         // Extract URL
//         let url = record.header(WarcHeader::TargetURI)
//             .map(|s| s.to_string())
//             .unwrap_or_default();
        
//         // Skip non-Medium URLs
//         if !url.contains("medium.com/") || url.contains("/tag/") || url.contains("/search") {
//             continue;
//         }
        
//         // Check WARC type is response (contains actual content)
//         if record.header(WarcHeader::WarcType).map(|s| s.to_string()) != Some("response".to_string()) {
//             continue;
//         }
        
//         // Process content
//         match record.into_buffered() {
//             Ok(buffered) => {
//                 let body = buffered.body();
                
//                 // Extract the actual HTML from the HTTP response
//                 if let Some(html_start) = find_html_start(body) {
//                     let html = &body[html_start..];
//                     let preview = String::from_utf8_lossy(&html);
//                     let file_name = format!("blog_{}.html", blog_count + 1);
//                     fs::write(file_name, &html).expect("Failed to write HTML preview");
                    
//                     println!("=== Blog #{} ===", blog_count + 1);
//                     println!("URL: {}", url);
//                     println!("Content preview:");
//                     println!("{}", preview);
//                     println!("-----");
                    
//                     blog_count += 1;
//                     if blog_count >= MAX_BLOGS {
//                         break;
//                     }
//                 } else {
//                     eprintln!("No HTML found in: {}", url);
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
// fn find_html_start(body: &[u8]) -> Option<usize> {
//     // Look for end of HTTP headers (blank line)
//     let header_end = body.windows(4).position(|w| w == b"\r\n\r\n")
//         .map(|pos| pos + 4)
//         .or_else(|| body.windows(2).position(|w| w == b"\n\n").map(|pos| pos + 2))?;
    
//     // Look for HTML tags after headers
//     let html_tag_start = body[header_end..]
//         .windows(5)
//         .position(|w| w.eq_ignore_ascii_case(b"<html") || w.eq_ignore_ascii_case(b"<!doc"))?;
    
//     Some(header_end + html_tag_start)
// }













// Amonia experiments



// use maplit::hashset;

// use ammonia::Builder
// ;
// fn main (){

//   let file_html= fs::read_to_string("blog_1.html").expect("Failed to read file");
//   let sanitized =  Builder::new()
//   .tags(hashset!("meta","p")).clean_content_tags().
//   generic_attributes(hashset!()).clean(&file_html).to_string();
    
//  println!("Sanitized HTML: {}", sanitized);

//   // You can now use `sanitized` as needed, e.g., save to a file or process further
// //   fs::write("sanitized_blog_1.html", sanitized).expect("Failed to write sanitized HTML");




// }


fn main (){
    let file_html= fs::read_to_string("blog_1.html").expect("Failed to read file");

      let html = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta property="og:title" content="Rust Memory Safety">
        <meta property="og:article:author" content="Alice Rust">
        <meta property="og:article:published_time" content="2023-09-20">
    </head>
    <body></body>
    </html>
    "#;

    let pipeline = MetadataPipeline::new();
    let metadata = pipeline.run(file_html.as_str());
    let blog_content = BlogProcessor::extract_and_sanitize(file_html.as_str());

    let mut blog = Blog{
    title: metadata.title.map(|f| f.value).unwrap_or("Untitled".into()),
    author: metadata.author.map(|f| f.value).unwrap_or("Unknown".into()),
    description: metadata.description.map(|f| f.value).unwrap_or_default(),
    date: metadata.date.map(|f| f.value).unwrap_or("".into()),
    publisher: metadata.publisher.map(|f| f.value).unwrap_or_default(),
    content: blog_content,

    };
   
        


       
    
    //  println!("Extracted Metadata:");
    // if let Some(title) = &metadata.title {
    // }
    // if let Some(author) = &metadata.author {
    //     println!("- Author: {} (source: {})", author.value, author.source);
    // }
    // if let Some(date) = &metadata.date {
    //     println!("- Date: {} (source: {})", date.value, date.source);
    // }
    // if let Some(publisher) = &metadata.publisher {
    //     println!("- Publisher: {} (source: {})", publisher.value, publisher.source);
    // }
    
  
}


