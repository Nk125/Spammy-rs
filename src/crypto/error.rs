use core::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error<'a> {
    ConversionError(&'a str),
    InvalidLength,
    SignatureError,
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConversionError(what) => write!(f, "Conversion error ({})", what),
            Self::InvalidLength => write!(f, "Invalid string length"),
            Self::SignatureError => write!(f, "Signature error"),
        }
    }
}

impl StdError for Error<'_> {}
