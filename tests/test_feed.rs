
        use feed_rs::parser;

    use lektos::utils::find_feeds::is_feed;

#[cfg(test)]
mod tests {
    use lektos::utils::find_feeds::parse_feed;

    use super::*;      


    
    
    

     const VALID_RSS: &str = r#"
    <?xml version="1.0"?>
    <rss version="2.0">
        <channel>
            <title>RSS Example Blog</title>
            <description>Test RSS feed</description>
            <link>https://example.com</link>
            <item>
                <title>Item 1</title>
                <description>Test item</description>
                <link>https://example.com/item1</link>
            </item>
        </channel>
    </rss>
    "#;

    // Valid Atom 1.0 sample
    const VALID_ATOM: &str = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <feed xmlns="http://www.w3.org/2005/Atom">
        <title>Atom Example Blog</title>
        <link href="https://example.com/atom"/>
        <updated>2025-07-21T00:00:00Z</updated>
        <entry>
            <title>Atom Entry Blog</title>
            <link href="https://example.com/entry1"/>
            <id>urn:uuid:1225c695-cfb8-4ebb-aaaa-80da344efa6a</id>
            <updated>2025-07-21T00:00:00Z</updated>
        </entry>
    </feed>
    "#;

    #[test]
    fn parse_valid_rss() {
        let result = parse_feed(VALID_RSS);
        assert!(result.is_ok());
        
        let feed = result.unwrap();
        assert_eq!(feed.title.unwrap().content, "RSS Example Blog");
        assert_eq!(feed.entries.len(), 1);
    }

    #[test]
    fn parse_valid_atom() {
        let result = parse_feed(VALID_ATOM);
        assert!(result.is_ok());
        
        let feed = result.unwrap();
        assert_eq!(feed.title.unwrap().content, "Atom Example Blog");
        assert_eq!(feed.entries.len(), 1);
    }

    #[test]
    fn parse_empty_string() {
        let result = parse_feed("");
        assert!(result.is_err());
    
    }

    #[test]
    fn parse_invalid_xml() {
        let result = parse_feed("This is not XML");
        assert!(result.is_err());
    }

    #[test]
    fn parse_malformed_feed() {
        let malformed = r#"
        <rss version="2.0">
            <channel>
                <title>Missing closing tag
            </channel>
        </rss>
        "#;
        
        let result = parse_feed(malformed);
        assert!(result.is_err());
    }

    #[test]
    fn parse_unsupported_format() {
        let json_feed = r#"
        {
            "version": "https://jsonfeed.org/version/1",
            "title": "JSON Feed"
        }
        "#;
        
        let result = parse_feed(json_feed);
        assert!(result.is_err());
    }

    #[test]
    fn parse_feeds_with_special_characters() {
        let feed = format!(
            r#"<rss version="2.0"><channel><title>Test &amp; {}</title></channel></rss>"#,
            "Entities"
        );
        
        let result = parse_feed(&feed);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().title.unwrap().content,
            "Test & Entities"
        );
    }

    #[test]
    fn parse_large_feed() {
        let mut large_feed = String::with_capacity(10_000);
        large_feed.push_str(r#"<rss version="2.0"><channel><title>Large Feed</title>"#);
        
        // Generate 1000 items
        for i in 0..1000 {
            large_feed.push_str(&format!(
                r#"<item><title>Item {}</title><description>Test</description></item>"#,
                i
            ));
        }
        
        large_feed.push_str("</channel></rss>");
        
        let result = parse_feed(&large_feed);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().entries.len(), 1000);
    }


    




}