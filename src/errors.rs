use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    EmbeddingService(String),
    #[from]
    Reqwest(reqwest::Error),


    #[from]
    Io(std::io::Error), // as example
}
