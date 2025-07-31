use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
  
    EmbeddingService(String),
    #[from]
        InvalidEmbeddingFormat,
            #[from]
    Reqwest(reqwest::Error),
    //   #[from]
    // Json(JsonError),

    
    


    
    #[from]
    Io(std::io::Error), // as example
}





