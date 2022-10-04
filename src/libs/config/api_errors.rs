extern crate derive_more;

use std::fmt::Debug;

use actix_web::{http::header, http::StatusCode, HttpResponse, HttpResponseBuilder};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ApiError {
    #[display(fmt = "Invalid position type")]
    InvalidPositionType,

    #[display(fmt = "No entries found")]
    NotFound,

    #[display(fmt = "Invalid option type")]
    InvalidOptionType,

    #[display(fmt = "Invalid strike price")]
    InvalidStrike,

    #[display(fmt = "Invalid number of options")]
    InvalidOptionCount,

    #[display(fmt = "Error parsing response body")]
    ParseError,

    #[display(fmt = "Internal error")]
    InternalError,

    #[display(fmt = "Bad request")]
    BadClientData,

    #[display(fmt = "Invalid transaction group")]
    InvalidTransactionGroup,

    #[display(fmt = "Invalid expirations on options")]
    InvalidExpiration,

    #[display(fmt = "Invalid user address")]
    InvalidUserAddress,

    #[display(fmt = "Data error")]
    DatabaseError,

    #[display(fmt = "Strategy no longer able to be updated")]
    InvalidStrategyStatus,
}

// impl Display for ApiError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadClientData => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
            .body(self.to_string())
    }
}