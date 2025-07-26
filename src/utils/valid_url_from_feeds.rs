use anyhow::Error;
use blake3::Hasher;
use bloom::BloomFilter;
use fjall::{Config, Partition, PartitionCreateOptions, PersistMode};
use std::path::Path;

// RSS URL verification system
pub struct FeedUrlValidator {
    bloom: BloomFilter,   // In-memory Bloom filter
    partition: Partition, // Fjall disk partition
}

impl FeedUrlValidator {
    /// Initialize the RSS verifier with its own partition
    pub fn new() -> Result<Self, Error> {
        let keyspace = Config::new(Path::new("src/db")).open()?;
        let partition = keyspace.open_partition("rss_urls", PartitionCreateOptions::default())?;

        Ok(Self {
            bloom: BloomFilter::with_rate(0.01, 1_000_000_000),
            partition,
        })
    }

    /// Hash URL to 16-byte array (same as URL visitor)
    fn hash_url(url: &str) -> [u8; 16] {
        let mut hasher = Hasher::new();
        hasher.update(url.as_bytes());
        let full_hash = hasher.finalize();
        let mut truncated = [0u8; 16];
        truncated.copy_from_slice(&full_hash.as_bytes()[..16]);
        truncated
    }

    /// Store multiple RSS URLs with batch optimization
    pub fn store_urls(&mut self, urls: &[String]) -> Result<(), Error> {
        for url in urls {
            let hash = Self::hash_url(url);
            self.bloom.insert(&hash.to_vec());
            self.partition.insert(&hash, b"")?;
        }
        Ok(())
    }

    /// Check if URL is from RSS feed (two-tier verification)
    pub fn is_from_feed(&self, url: &str) -> Result<bool, Error> {
        let hash = Self::hash_url(url);
        let hash_bytes = &hash[..];

        // Tier 1: Bloom filter check (memory)
        if !self.bloom.contains(&hash_bytes.to_vec()) {
            return Ok(false);
        }

        // Tier 2: Disk verification
        Ok(self.partition.get(hash_bytes)?.is_some())
    }
}

// Example usage
