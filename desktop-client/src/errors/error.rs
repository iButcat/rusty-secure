use core::fmt;

#[derive(Debug)]
pub enum Error {
    ApiError(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
           Error::ApiError(msg) => write!(f, "{}", msg)
        }
    }
}

impl std::error::Error for Error {}

