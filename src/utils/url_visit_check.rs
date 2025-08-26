use blake3::Hasher;
use bloom::BloomFilter;

use fjall::{Config, Keyspace, Partition, PartitionCreateOptions};
use std::path::Path;

// dedup implementationf for distributed 
struct UrlStore {
    partition: Partition, // "visited_urls" partition
}

impl UrlStore {
    pub fn new(path: &Path) -> Self {
        let keyspace = Config::new(path)
            .open()
            .expect("Failed to open Fjall keyspace");

        let partition = keyspace
            .open_partition("visited_urls", PartitionCreateOptions::default())
            .expect("Failed to create psartition");

        Self { partition }
    }
}










pub struct UrlVisitTracker {
    pub bloom: BloomFilter, // In-memory filter
    pub store: UrlStore,    // Disk-backed store
}

impl UrlVisitTracker {
    pub fn new() -> Self {
        Self {
            bloom: BloomFilter::with_rate(0.01, 1_000_000_000),
            // todo :change hardcoded urls 
            store: UrlStore::new(Path::new("home/natnael/projects/lektos/src/db")),
        }
    }

    /// Hash URL to 16-byte array
    pub fn hash_url(url: &str) -> [u8; 16] {
        let mut hasher = Hasher::new();
        hasher.update(url.as_bytes());
        let full_hash = hasher.finalize();
        let mut truncated = [0u8; 16];
        truncated.copy_from_slice(&full_hash.as_bytes()[..16]);
        truncated
    }

    /// Core deduplication logic
    pub fn is_url_visited(&self, url: &str) -> bool {
        let hash = Self::hash_url(url);
        let hash_bytes = &hash[..];

        // Step 1: Bloom filter check
        if !self.bloom.contains(&hash_bytes.to_vec()) {
            return false;
        }

        // Step 2: Fjall disk check
        match self.store.partition.get(hash_bytes) {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(_) => false,
        }
    }

    /// Mark URL as visited
    pub fn mark_visited(&mut self, url: &str) {
        let hash = Self::hash_url(url);
        let hash_bytes = &hash[..];

        // Add to Bloom filter
        self.bloom.insert(&hash_bytes.to_vec());

        self.store
            .partition
            .insert(hash_bytes, &[])
            .expect("Fjall write failed");
    }
}


