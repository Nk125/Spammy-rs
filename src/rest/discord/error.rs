use ntex::{http, web};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorType {
    VerificationError,
    JsonError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    error: ErrorType,
}

impl Error {
    pub fn new(error: ErrorType) -> Self {
        Self { error }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error {
            ErrorType::VerificationError => write!(f, "Failed to verify cryptographic signature"),
            ErrorType::JsonError => write!(f, "Failed at parsing json body"),
        }
    }
}

impl StdError for Error {}

impl web::error::WebResponseError for Error {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        web::HttpResponse::build(self.status_code()).json(&self)
    }

    fn status_code(&self) -> http::StatusCode {
        http::StatusCode::BAD_REQUEST
    }
}
