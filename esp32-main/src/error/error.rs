use core::error::Error as CoreError;
use core::fmt;
use heapless::String;

#[derive(Debug)]
pub enum Error {
    Timeout,
    Error(String<128>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Timeout => write!(f, "Timeout"),
            Error::Error(s) => write!(f, "Error: {}", s),
        }
    }
}

impl CoreError for Error {}