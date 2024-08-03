use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

//Using enumeration
#[derive(Error, Debug)]
pub enum Error {
    #[error("File not found")]
    FileNotFound,
    #[error("Internal Server Error")]
    InternalServerError,
}

//Convert errors -> HttpResponse
impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::FileNotFound => HttpResponse::NotFound().finish(),
            Error::InternalServerError => HttpResponse::InternalServerError().finish(),
        }
    }
}
