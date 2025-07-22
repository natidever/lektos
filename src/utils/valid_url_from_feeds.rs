use anyhow::Error;

pub fn store_valid_url_from_feed(url: &str){
    use fjall::{Config, PersistMode, Keyspace, PartitionCreateOptions};

// A keyspace is a database, which may contain multiple collections ("partitions")
// You should probably only use a single keyspace for your application
//
let folder = "src/db"
let keyspace = Config::new(folder).open()?; // or open_transactional for transactional semantics

// Each partition is its own physical LSM-tree
let items = keyspace.open_partition("my_items", PartitionCreateOptions::default())?;

// Write some data
items.insert("a", "hello")?;

// And retrieve it
let bytes = items.get("a")?;

// Or remove it again
items.remove("a")?;

// Search by prefix
for kv in items.prefix("prefix") {
  // ...
}

// Search by range
for kv in items.range("a"..="z") {
  // ...
}

// Iterators implement DoubleEndedIterator, so you can search backwards, too!
for kv in items.prefix("prefix").rev() {
  // ...
}

// Sync the journal to disk to make sure data is definitely durable
// When the keyspace is dropped, it will try to persist with `PersistMode::SyncAll` as well
keyspace.persist(PersistMode::SyncAll)?;
}