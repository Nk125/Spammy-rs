use crate::interaction::{request, response};
use crate::env::load_msg::load_message;
use ntex::web;
use serde_json as json;
use super::{error, verifier::Verifier};

pub async fn interactions(
    request: web::HttpRequest,
    body: String,
) -> Result<web::HttpResponse, error::Error> {
    Verifier::verify(request, &body).map_err(|e| {
        log::error!("Error verifying request authenticity: {}", e);
        error::Error::new(error::ErrorType::VerificationError)
    })?;

    let json_body: json::Value = json::from_str(&body).map_err(|e| {
        log::error!("Error deserializing json: {}", e);
        error::Error::new(error::ErrorType::JsonError)
    })?;

    let req_type = json_body["type"].as_i64().ok_or_else(|| {
        log::error!("Failed to get type from json");
        error::Error::new(error::ErrorType::JsonError)
    })? as u8;

    let req_type = request::RequestType::try_from(req_type).map_err(|_| {
        log::error!("Req type not found");
        error::Error::new(error::ErrorType::JsonError)
    })?;

    match req_type {
        request::RequestType::Ping => Ok(web::HttpResponse::Ok().json(&response::Response {
            category: response::ResponseType::Pong,
            data: None
        })),
        request::RequestType::ApplicationCommand => Ok(web::HttpResponse::Ok().json(&response::Response {
            category: response::ResponseType::ChannelMessageWithSource,
            data: Some(json::json!({
                "content": load_message()
            }))
        }))
    }
}
