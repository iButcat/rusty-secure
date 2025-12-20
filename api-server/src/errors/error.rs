use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use core::fmt;
use serde_json::json;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    WithText(String),
    Database(String),
    Storage(String),
    Service(String),
    NotFound(String),
    Empty(String),
    UuidFormat(String),
    Internal(String),
    Parse(String),
    JSONUnmarshall(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::WithText(msg) => write!(f, "{}", msg),
            Error::Database(msg) => write!(f, "database error: {}", msg),
            Error::Storage(msg) => write!(f, "storage error: {}", msg),
            Error::Service(msg) => write!(f, "service error: {}", msg),
            Error::NotFound(msg) => write!(f, "not found: {}", msg),
            Error::Empty(msg) => write!(f, "{}", msg),
            Error::UuidFormat(msg) => write!(f, "error trying to format uuid: {}", msg),
            Error::Internal(msg) => write!(f, "internal server error: {}", msg),
            Error::Parse(msg) => write!(f, "error trying to parse: {}", msg),
            Error::JSONUnmarshall(msg) => {
                write!(f, "error trying to unmarshall json: {}", msg)
            }
        }
    }
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::UuidFormat(_) => StatusCode::BAD_REQUEST,
            Error::Empty(_) => StatusCode::BAD_REQUEST,
            Error::WithText(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(json!({
            "error": self.to_string()
        }))
    }
}
