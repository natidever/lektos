#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use lektos::utils::valid_url_from_feeds::RssUrlVerifier;
    use tempfile::tempdir;

    // Helper to create a test verifier
    fn setup_verifier() -> (RssUrlVerifier, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let verifier = RssUrlVerifier::new(dir.path()).unwrap();
        (verifier, dir) // Return both to keep tempdir alive
    }

    #[test]
    fn test_new_creates_partition() {
        let dir = tempdir().unwrap();
        let mut verifier = RssUrlVerifier::new(dir.path()).unwrap();
        
        // Verify partition exists by attempting to insert
        assert!(verifier.store_urls(&["https://test.com/rss".to_string()]).is_ok());
    }

    #[test]
    fn test_store_and_verify_url() {
        let (mut verifier, _dir) = setup_verifier();
        let test_url = "https://example.com/feed.xml";
        
        // Store URL
        verifier.store_urls(&[test_url.to_string()]).unwrap();
        
        // Verify positive case
        assert!(verifier.is_from_feed(test_url).unwrap());
        
        // Verify negative case
        assert!(!verifier.is_from_feed("https://unknown.com/feed").unwrap());
    }

    #[test]
    fn test_batch_url_operations() {
        let (mut verifier, _dir) = setup_verifier();
        let urls = vec![
            "https://blog.com/atom".to_string(),
            "https://news.com/rss".to_string(),
        ];
        
        // Store batch
        verifier.store_urls(&urls).unwrap();
        
        // Verify all
        for url in urls {
            assert!(verifier.is_from_feed(&url).unwrap());
        }
    }

    #[test]
    fn test_bloom_filter_false_positives() {
        let (mut verifier, _dir) = setup_verifier();
        
        // Insert known URL
        verifier.store_urls(&["https://real.com/feed".to_string()]).unwrap();
        
        // Test with URL that may trigger false positive
        let mut false_positives = 0;
        for i in 0..1000 {
            let test_url = format!("https://fake{}.com/rss", i);
            if verifier.is_from_feed(&test_url).unwrap() {
                false_positives += 1;
            }
        }
        
        // Should have <2% false positives (we configured 1%)
        assert!(false_positives < 20);
    }

    #[test]
    fn test_empty_input_handling() {
        let (verifier, _dir) = setup_verifier();
        
        // Verify empty check doesn't panic
        assert!(!verifier.is_from_feed("").unwrap());
    }

    #[test]
    fn test_url_normalization() {
        let (mut verifier, _dir) = setup_verifier();
        let base_url = "https://example.com/feed";
        let variant_url = "https://example.com/feed?utm_source=test";
        
        // Store base URL
        verifier.store_urls(&[base_url.to_string()]).unwrap();
        
        // Should NOT match variant (unless normalized)
        assert!(!verifier.is_from_feed(variant_url).unwrap());
    }

    
}