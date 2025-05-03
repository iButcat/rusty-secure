use core::fmt;

#[derive(Debug)]
pub enum Error {
    WithText(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::WithText(msg) => write!(f, "{}", msg),
        }
    }
}