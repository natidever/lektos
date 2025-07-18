

// use warc::Record;
use warc::WarcHeader;
use warc::WarcReader;
use std::fs;
use std::io;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use rayon::prelude::*;

use crate::extractors::pipeline::MetadataPipeline;
use crate::models::blog::Blog;
use crate::models::metadata;
use crate::utils::embed::generate_embedding;
use crate::utils::html_utils::BlogProcessor;

mod utils;
mod extractors;
mod models;



// Import Embedding and Embeddings from the 'embed' module
use anyhow::Result;


fn is_blog_url(url: &str) -> bool {
    let url_lower = url.to_lowercase();
    
    // Domain-based matching (more reliable than path-based)
    let is_target_domain = 
        url_lower.contains(".medium.com") ||
        url_lower.contains(".substack.com") ;
        // url_lower.contains(".tumblr.com") ||
        url_lower.contains(".blogspot.com") ;
        // // url_lower.contains(".wordpress.com") ||
        // url_lower.contains(".ghost.io") ||
        // // url_lower.contains("blogger.com") ||
        // // url_lower.contains(".hashnode.dev") ||
        // // url_lower.contains(".dev.to") ||
        // // url_lower.contains(".wixsite.com") ||
        // // url_lower.contains(".weebly.com");
        // url_lower.contains("/blog/");


    // Path-based exclusions
    let is_excluded_path = url_lower.contains("/search") ||
        url_lower.contains("/tag/") ||
        url_lower.contains("/category/") ||
        url_lower.contains("/author/") ||
        url_lower.contains("/page/") ||
        url_lower.contains("/wp-json") ||
        url_lower.contains("/feed")||
        url_lower.contains("/comments")||
        url_lower.contains("/comment");



    is_target_domain && !is_excluded_path
    //    is_target_domain
}



// fn main() -> io::Result<()> {

// // PSEUDO-CODE - CONCEPTUAL IMPLEMENTATION

// use rayon::prelude::*;
// use std::sync::atomic::{AtomicUsize, Ordering};
























    


//     println!("Extracting Medium blogs from WARC file...");
//     // let warc_name = "src/resources/2025-26.warc";
//     let warc_name = "src/resources/CC-MAIN-20250612112840-20250612142840-00000.warc.gz";


    
    
//     let mut reader = WarcReader::from_path_gzip(warc_name)?; 


//     let mut stream_iter = reader.stream_records();
// // let mut records = Vec::new();

// // Use a loop with explicit scoping




//     // let mut strea_iter=reader::stream_recored();

  

    
//     let mut blog_count = 0;
//     const MAX_BLOGS: usize = 500;


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
        
  
//        if !is_blog_url(&url){
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
//                     let html_content = String::from_utf8_lossy(&html);
//                     let file_name = format!("sub_sblog_{}.html", blog_count + 1);
//                     println!("URL {}",url);

//                     // if url.contains(".substack"){
//                     // fs::write(file_name, &html).expect("Failed to write HTML preview");


//                     // }

//                     // fs::write(file_name, &html).expect("Failed to write HTML preview");
//                         let pipeline = MetadataPipeline::new();
//     let file_html= fs::read_to_string("ss.html").expect("Failed to read file");
//     // println!("Extracting from HTML  {}",file_html);

//     let metadata = pipeline.run(file_html.as_ref());
//     let blog_content = BlogProcessor::extract_and_sanitize(file_html.as_ref());
//     println!("MetaDataEXTRACTED: {:?}", metadata);
                    
//                     // println!("=== Blog #{} ===", blog_count + 1);
//                     // println!("Content preview {}",preview);
//                     // println!("{}", preview);
//                     // println!("-----");
                    
//                     blog_count += 1;
//                     if blog_count >= MAX_BLOGS {
//                         break;
//                     }
//                 } else {
//                     // here we can check if it is RSS feed or atom 
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
fn find_html_start(body: &[u8]) -> Option<usize> {
    // Look for end of HTTP headers (blank line)
    let header_end = body.windows(4).position(|w| w == b"\r\n\r\n")
        .map(|pos| pos + 4)
        .or_else(|| body.windows(2).position(|w| w == b"\n\n").map(|pos| pos + 2))?;
    
    // Look for HTML tags after headers
    let html_tag_start = body[header_end..]
        .windows(5)
        .position(|w| w.eq_ignore_ascii_case(b"<html") || w.eq_ignore_ascii_case(b"<!doc"))?;
    
    Some(header_end + html_tag_start)
}















#[tokio::main]
async fn main (){

  

    let file_html= fs::read_to_string("ss.html").expect("Failed to read file");

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
    content: "blog_content".to_string(),


    };

    println!("Extracted Metadata: {:?}", blog);
    

    // let embedding_text = blog.to_embedding_text();
   

    // let api_key = &std::env::var("GEMINI_API_KEY").expect("API KEY NOT FOUND");
    
   
    // let embedding_model = "models/embedding-001"; 

 
    // let text1 = "The quick brown fox jumps over the lazy dog.".to_string();

    // match generate_embedding(text1, api_key, embedding_model).await {
    //     Ok(embedding) => {
    //         println!("Main: Successfully obtained embedding with dimensionality: {}", embedding.len());
    //         println!("Main: First 5 values: {:?}", &embedding[0..5]);
    //     }
    //     Err(e) => {
    //         eprintln!("Main: Failed to get embedding for example 1: {}", e);
    //     }
    // }
    
      





    
  
}


// """" get html 

