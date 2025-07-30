use derive_more::{Display, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
  
    Embeding,

    
    #[from]
    Io(std::io::Error), // as example
}



impl Error {
    pub fn custom_from_err(err: impl std::error::Error) -> Self {
        Self::Custom(err.to_string())
    }

    pub fn custom(val: impl Into<String>) -> Self {
        Self::Custom(val.into())
    }
}


