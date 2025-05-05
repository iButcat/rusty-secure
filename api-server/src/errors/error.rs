use core::fmt;
use std::fmt::Display;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde_json::json;

#[derive(Debug)]
pub enum Error {
    WithText(String),
    DatabaseError(String),
    StorageError(String),
    ServiceError(String),
    NotFound(String),
    EmptyError(String),
    UuidFormatError(String),
    InternalError(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::WithText(msg) => write!(f, "{}", msg),
            Error::DatabaseError(msg) => write!(f, "{}", msg),
            Error::StorageError(msg) => write!(f, "{}", msg),
            Error::ServiceError(msg) => write!(f, "{}", msg),
            Error::NotFound(msg) => write!(f, "{}", msg),
            Error::EmptyError(msg) => write!(f, "{}", msg),
            Error::UuidFormatError(msg) => write!(f, "{}", msg),
            Error::InternalError(msg) => write!(f, "{}", msg)
        }
    }
}

impl std::error::Error for Error {}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::UuidFormatError(_) => StatusCode::BAD_REQUEST,
            Error::EmptyError(_) => StatusCode::BAD_REQUEST,
            Error::WithText(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse{
        let status_code = self.status_code();
        HttpResponse::build(status_code)
        .json(json!({
            "error": self.to_string()
        }))
    }
}