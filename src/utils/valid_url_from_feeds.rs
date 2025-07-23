use anyhow::Error;
use fjall::{Config, PersistMode, Keyspace, PartitionCreateOptions};


pub fn store_valid_url_from_feed(urls:&[String])-> Result<(), Error> {


//
let folder = "src/db";
let keyspace = Config::new(folder).open()?; 

let items = keyspace.open_partition("valid_urls", PartitionCreateOptions::default())?;

// Here we will hash the key for better comparison and similar key length 

// Write some data
for url in urls {
    items.insert(url, "")?;
}


// When the keyspace is dropped, it will try to persist with `PersistMode::SyncAll` as well
keyspace.persist(PersistMode::SyncAll)?;
Ok(())
}


pub fn is_url_from_feed(url:&str)->Result<bool, Error> {
let folder = "src/db";
let keyspace = Config::new(folder).open()?; 

let items = keyspace.open_partition("my_items", PartitionCreateOptions::default())?;

let bytes = items.get("a")?;


// todo:same again we'll compare the hash not hte real string value



 match bytes {
        Some(slice) =>{
            if  slice.as_ref() == url.as_bytes() {
                return Ok(true)
            }else{
                return Ok(false)
            }
        }
        None => Ok(false),
    }

}


