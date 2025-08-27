use blake3::Hasher;
use bloom::BloomFilter;
use anyhow::{ Result};

use fjall::{Config, Keyspace, Partition, PartitionCreateOptions};
use std::path::Path;
use scylla::client::session::Session;
use crate::scylla::quries::{create_or_get_syclla_session, create_scylla_keyspace, UrlStoreProcedures};

// dedup implementationf for distributed
#[derive(Debug)]
struct UrlStore {
    scylla_session:Session,
    url_store_procedures: UrlStoreProcedures, // "visited_urls" partition ket his be sessin
}

impl UrlStore {
    pub async fn new() -> Result<Self> {
        
        let scylla_session = create_or_get_syclla_session().await?;
        let url_store_procedures=UrlStoreProcedures::default();
        Ok(Self { scylla_session ,url_store_procedures})
    }



}




pub struct UrlVisitTracker {
    pub bloom: BloomFilter, // In-memory filter
    pub store: UrlStore,    // Disk-backed store(scyla)
}

impl UrlVisitTracker {
    pub async  fn new() -> Result<Self> {
        Ok(
            Self {
            bloom: BloomFilter::with_rate(0.01, 1_000_000_000),
            // todo :change hardcoded urls
            store: UrlStore::new().await?,
        }
        )
        
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
    pub async fn is_url_visited(&self, url: &str) -> bool {
        let hash = Self::hash_url(url);
        let hash_bytes = &hash[..];

        // Step 1: Bloom filter check
        if !self.bloom.contains(&hash_bytes.to_vec()) {
            return false;
        }

        // Step 2:  disk check -Scylla 
       

        match self.store.url_store_procedures.get_url_hash(&self.store.scylla_session, url).await{
            Ok(Some(_))=>true,
            Ok(None)=>false,
            Err(_)=>false
        }
    }

    /// Mark URL as visited
    pub async  fn mark_visited(&mut self, url: &str) {
        let hash = Self::hash_url(url);
        let hash_bytes = &hash[..];

        // Add to Bloom filter
        self.bloom.insert(&hash_bytes.to_vec());

        self.store.url_store_procedures.store_url_hash(&self.store.scylla_session, url).await;
        
    }
}
