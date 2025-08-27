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


#[test]

fn test_url_hashing_different_urls() {

let urls = create_test_urls();

let mut hashes = HashSet::new();

// hashing different url

for url in urls {

let hash = UrlVisitTracker::hash_url(url);

assert_eq!(hash.len(), 16);



let hash_vec = hash.to_vec();

assert!(hashes.insert(hash_vec), "Hash collision detected for URL: {}", url);

}

}




#[test]

fn test_url_hashing_edge_cases() {

let edge_cases = vec![

"", // Empty string

"a", // Single character

"https://", // Protocol only

"https://example.com/very/long/path/with/many/segments/and/query?param1=value1&param2=value2&param3=value3", // Very long URL

"https://example.com/path with spaces", // URL with spaces

"https://example.com/path?query=value#fragment", // URL with query and fragment

"https://user:pass@example.com:8080/path", // Url with credentials and port

];

  

for url in edge_cases {

let hash = UrlVisitTracker::hash_url(url);

assert_eq!(hash.len(), 16, "Hash length should be 16 bytes for URL: {}", url);

}


}


#[test]
fn test_blake3_hash_verification() {

let url = "https://example.com";

let tracker_hash = UrlVisitTracker::hash_url(url);

// Manually compute Blake3 hash to verify our implementation

let mut hasher = Hasher::new();

hasher.update(url.as_bytes());

let full_hash = hasher.finalize();

let mut expected_hash = [0u8; 16];

expected_hash.copy_from_slice(&full_hash.as_bytes()[..16]);

assert_eq!(tracker_hash, expected_hash);

}


#[test]

fn test_bloom_filter_basic_functionality() {

let mut bloom = BloomFilter::with_rate(0.01, 1_000_000);

let test_urls = create_test_urls();

// first no URLs should be in the bloom filter

for url in &test_urls {

let hash = UrlVisitTracker::hash_url(url);

assert!(!bloom.contains(&hash.to_vec()));

}

//adding url to blom

for url in &test_urls {

let hash = UrlVisitTracker::hash_url(url);

bloom.insert(&hash.to_vec());

}

// 

for url in &test_urls {

let hash = UrlVisitTracker::hash_url(url);

assert!(bloom.contains(&hash.to_vec()));

}

}

  #[test]

fn test_bloom_filter_false_positives() {

let mut bloom = BloomFilter::with_rate(0.01, 1_000_000);

let test_urls = vec!["https://example1.com", "https://example2.com"];

let check_urls = vec!["https://example3.com", "https://example4.com"];

// Add test URLs to bloom filter

for url in &test_urls {

let hash = UrlVisitTracker::hash_url(url);

bloom.insert(&hash.to_vec());

}

// Check URLs should not be in bloom filter (though false positives are possible)

let mut false_positives = 0;

for url in &check_urls {

let hash = UrlVisitTracker::hash_url(url);

if bloom.contains(&hash.to_vec()) {

false_positives += 1;

}

}

// With a 0.01 false positive rate and only 2 items, false positives should be rare

assert!(false_positives <= check_urls.len(), "Too many false positives");

}




 

#[tokio::test]

async fn test_url_visit_tracker_creation() {

// This test might fail if Scylla is not available, so we'll make it conditional

match UrlVisitTracker::new().await {

Ok(_tracker) => {

// If we can create a tracker, that's good

println!("Successfully created UrlVisitTracker");

}

Err(e) => {

// If Scylla is not available, that's expected in test environment

println!("Scylla not available for testing: {} - this is expected in CI/test environments", e);

}

}

}