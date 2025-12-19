use core::error::Error as CoreError;
use core::fmt::{self, Write};
use esp_hal::i2c::master::Error as I2cError;
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

impl From<I2cError> for Error {
    fn from(e: I2cError) -> Self {
        let mut error_str = String::<128>::new();
        write!(&mut error_str, "I2C Error: {:?}", e).unwrap_or(());
        Error::Error(error_str)
    }
}
