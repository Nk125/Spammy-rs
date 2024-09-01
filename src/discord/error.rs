use std::error::Error as StdError;

#[derive(Debug)]
pub struct Error {
    pub category: ErrorCategory,
    cause: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ErrorCategory {
    Auth,
    Conversion,
    Transport,
    ServerSide,
    Lib,
}

impl Error {
    pub fn new(category: ErrorCategory, cause: Option<String>) -> Self {
        Self { category, cause }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first_write = match self.category {
            ErrorCategory::Auth => write!(f, "Invalid token"),
            ErrorCategory::Conversion => write!(f, "Conversion error"),
            ErrorCategory::Transport => write!(f, "Error while transacting API"),
            ErrorCategory::ServerSide => write!(f, "Error from the server"),
            ErrorCategory::Lib => write!(f, "Underlying lib error"),
        };

        if let Some(cause) = &self.cause {
            write!(f, " ({})", cause)
        } else {
            first_write
        }
    }
}

impl StdError for Error {}
