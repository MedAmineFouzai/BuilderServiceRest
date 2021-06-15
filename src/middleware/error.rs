use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    status_code: String,
    msg: String,
}

#[derive(Debug, Display, Error)]
pub enum ContentBuilderCustomResponseError {
    #[display(fmt = "internal error !")]
    InternalError,

    #[display(fmt = "Bad Header Data Forbidden !")]
    BadHeaderData,

    #[display(fmt = "Bad Client Data !")]
    BadClientData,

    #[display(fmt = "Category not Found!")]
    NotFound,

    #[display(fmt = "User not Allowed!")]
    NotAllowed,

}

impl error::ResponseError for ContentBuilderCustomResponseError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            status_code: self.status_code().to_string(),
            msg: self.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ContentBuilderCustomResponseError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ContentBuilderCustomResponseError::BadClientData => StatusCode::BAD_REQUEST,
            ContentBuilderCustomResponseError::NotFound => StatusCode::NOT_FOUND,
            ContentBuilderCustomResponseError::BadHeaderData => StatusCode::FORBIDDEN,
            // UserCustomResponseError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            ContentBuilderCustomResponseError::NotAllowed => StatusCode::FORBIDDEN,
        }
    }
}
