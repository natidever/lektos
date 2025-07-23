use anyhow::Error;
use lektos::utils::find_feeds::extract_url;
use lektos::utils::find_feeds::parse_feed;
// use warc::Record;
use rayon::prelude::*;
use reqwest::Response;
use warc::BufferedBody;
use warc::Record;
use warc::StreamingBody;
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
use crate::utils::valid_url_from_feeds::is_url_from_feed;
use crate::utils::valid_url_from_feeds::store_valid_url_from_feed;
// use crate::utils::valid_url_from_feeds::test_rock_db;

mod extractors;
mod models;
mod utils;
use std::io::Write;

// Import Embedding and Embeddings from the 'embed' module
use anyhow::Result;

    use rayon::prelude::*;



// Finds the start of HTML content in HTTP response


// pub fn main_m (){

//     let files = fs::read_to_string("tests/fixtures/medium.xml").expect("Unable to read file");

//     let result = extract_url(&files).expect("Failed to extract feed");

//     println!("Extracted url,{}",result);
    


// }




pub fn main ()-> Result<()> {
    // main_m();
    // is url feed
     
    let warc_name = "src/common_crawl_2025-26_warcfiles/CC-MAIN-20250612112840-20250612142840-00003.warc.gz";

    let mut reader = WarcReader::from_path_gzip(warc_name).unwrap();

    let mut stream_iter = reader.stream_records();

    let mut blog_count = 0;
    const MAX_BLOGS: usize = 500;

    let mut confirmed_blogs = 0;


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

       

    

        // Check WARC type is response (contains actual content) which is the html
        if record.header(WarcHeader::WarcType).map(|s| s.to_string())
            != Some("response".to_string())
        {
            continue;
        }
         if is_url_visited(){ continue;}

        if is_url_from_feed(&url)?{

            //processs it 
        }else{
            if is_blog_url(&url){


                match record.into_buffered() {
            Ok(buffered) => {
                let body = buffered.body();

                // Extract the actual HTML from the HTTP response
                if let Some(html_start) = find_html_start(body) {
                    let html = &body[html_start..];
                    let html_content = String::from_utf8_lossy(&html);
                    let file_name = format!("sub_sblog_{}.html", blog_count + 1);
                    

                    

                    let pipeline = MetadataPipeline::new();
                    let file_html = html_content.to_string();

                  

                    blog_count += 1;
                    if blog_count >= MAX_BLOGS {
                        break;
                    }
                } 
                else {
                    // the content is not htm check if it is feed (which is xml format )

                    let string_content = String::from_utf8_lossy(body);
                   
                    // here we can check if it is Rss/atoms
                    if is_feed(string_content.as_ref()) {
                        // extract the url from here 
                      let urls=  extract_url(string_content.as_ref()).unwrap();
                      

                        // store it in db as valid url(no need classification later)
                        store_valid_url_from_feed(&urls).unwrap();

                    
                        
                    }
                }
            }
            Err(e) => {
                eprintln!("Error buffering record: {}", e);
            }
        }
                
               
            } else{
                // do nothin
                // the blog is not url
            
            }
        }
        
       
        

        
    }
   Ok(())
}




pub fn is_url_visited()->bool{
    todo!()
}







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