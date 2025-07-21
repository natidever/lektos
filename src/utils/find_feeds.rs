use anyhow::Error;
use feed_rs::parser;

// pretty simple check if it it is a feed
pub fn is_feed(content: &str) -> bool {
    // Check if the content starts with an RSS or Atom tag
    content.starts_with("<?xml") || content.contains("<rss") || content.contains("<feed")
}

pub fn parse_feed(content: &str) -> Result<feed_rs::model::Feed, Error> {
    // Use `?` to propagate errors instead of unwrapping
    Ok(parser::parse(content.as_bytes())?)
}
