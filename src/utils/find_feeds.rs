use anyhow::Error;
use feed_rs::parser;

use rss::Channel;

// pretty simple check if it it is a feed
pub fn is_feed(content: &str) -> bool {
    // Check if the content starts with an RSS or Atom tag
    content.starts_with("<?xml") || content.contains("<rss") || content.contains("<feed")
}

pub fn parse_feed(content: &str) -> Result<feed_rs::model::Feed, Error> {
    // Use `?` to propagate errors instead of unwrapping
    Ok(parser::parse(content.as_bytes())?)
}





pub fn dubline_extractor(){

let xml = std::fs::read_to_string("src/resources/medium.xml").unwrap();
let channel = xml.parse::<Channel>().unwrap();
// Feed-level Dublin Core metadata
if let Some(dc_feed) = channel.dublin_core_ext() {

    
    for creator in &dc_feed.creators {
        println!("Feed author: {}", creator);
    }
}

// Item-level Dublin Core metadata
for item in channel.items() {
    if let Some(dc_item) = item.dublin_core_ext() {
        for creator in &dc_item.creators {
            println!("Item author: {}", creator);
        }
    }
    println!("Title: {:?}", item.title());
}

}