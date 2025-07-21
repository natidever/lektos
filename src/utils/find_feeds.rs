use std::collections::HashMap;

use anyhow::Error;
use feed_rs::parser;

use reqwest::header::CONTENT_LENGTH;
use rss::Channel;

use crate::models::metadata::{ExtractionResult, FeedExtractionResult, FieldResult};


pub fn is_feed(content: &str) -> bool {
    // Check if the content starts with an RSS or Atom tag
    content.starts_with("<?xml") || content.contains("<rss") || content.contains("<feed")
}

pub fn parse_feed(content:
    
    
     &str) -> Result<feed_rs::model::Feed, Error> {
    // Use `?` to propagate errors instead of unwrapping
    Ok(parser::parse(content.as_bytes())?)
}




pub fn extract_feed(content: &str) -> Result<Vec<FeedExtractionResult>, Box<dyn std::error::Error>> {
    // Parse with feed_rs first
    let feed = parse_feed(content)?;
    // Parse with rss for Dublin Core fallback
    let channel = Channel::read_from(content.as_bytes())?;
    
    // Build Dublin Core authors lookup map
    let dc_authors: HashMap<String, String> = channel.items().iter()
        .filter_map(|item| {
            let link = item.link.clone()?;
            let author = item.dublin_core_ext()
                .and_then(|dc| {
                    if !dc.creators.is_empty() {
                        Some(dc.creators[0].clone())
                    } else {
                        None
                    }
                });
            Some((link, author?))
        })
        .collect();

    // Process all feed entries
    Ok(feed.entries.iter().map(|entry| {
        let url = entry.links.first()
            .map(|link| link.href.clone())
            .unwrap_or_default();

        FeedExtractionResult {
            title: entry.title.as_ref()
                .map(|t| t.content.clone())
                .unwrap_or_default(),
                
            author: entry.authors.first()
                .map(|a| a.name.clone())
                .or_else(|| dc_authors.get(&url).cloned())
                .unwrap_or_default(),
                
            description: entry.summary.as_ref()
                .map(|s| s.content.clone())
                .or_else(|| entry.content.as_ref().and_then(|c| c.body.clone()))
                .unwrap_or_default(),
                
            date: entry.published
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_default(),
                
            // publisher: entry.source.as_ref()
            //     .and_then(|s| s.title.as_ref())
            //     .map(|t| t.content.clone())
            //     .unwrap_or_default(),
                
            url,
        }
    }).collect())
}
















fn test_it() -> Result<(), Error> {
    // 1. Read the XML file
    let content = std::fs::read_to_string("tests/fixtures/medium.xml")?;
    
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








