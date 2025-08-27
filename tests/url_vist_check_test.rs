use blake3::Hasher;
use bloom::BloomFilter;
use std::collections::HashSet;

// Import the modules we're testing
use lektos::utils::url_visit_check::UrlVisitTracker;

// Test helper functions
fn create_test_urls() -> Vec<&'static str> {
    vec![
        "https://example.com",
        "https://google.com",
        "https://github.com/rust-lang/rust",
        "https://docs.rs/tokio/latest/tokio/",
        "https://crates.io/crates/serde",
        "https://www.rust-lang.org/",
        "https://blog.rust-lang.org/",
        "https://forge.rust-lang.org/",
        "https://internals.rust-lang.org/",
        "https://users.rust-lang.org/",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_hashing_consistency() {
        let url = "https://example.com";
        
        // just hasing same url multiple time to test consistency
        let hash1 = UrlVisitTracker::hash_url(url);
        let hash2 = UrlVisitTracker::hash_url(url);
        let hash3 = UrlVisitTracker::hash_url(url);
        
        // here all must be identical
        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
        assert_eq!(hash1.len(), 16); // should be 16 bytes also
    }
}
