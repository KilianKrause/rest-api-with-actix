use actix_web::{error, HttpResponse};
use failure::Fail;
use serde_json::Value; // json representation of error message

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "{}", _0)]
    NotFound(Value),
    #[fail(display = "{}", _0)]
    Conflict(Value),
}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::NotFound(msg) => HttpResponse::NotFound().json(msg),
            Error::Conflict(msg) => HttpResponse::Conflict().json(msg),
        }
    }
}
