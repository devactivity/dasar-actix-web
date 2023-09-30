use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use sqlx::Error as PgError;
use serde_json::{Map as JsonMap, Value as JsonValue};
use validator::ValidationErrors;
use thiserror::Error as ThisError;

use std::convert::From;

#[derive(ThisError, Debug)]
pub enum Error {
    // 400
    #[error("BadRequest: {0}")]
    BadRequest(JsonValue),

    // 401
    #[error("Unauthorized: {0}")]
    Unauthorized(JsonValue),

    // 403
    #[error("Forbidden: {0}")]
    Forbidden(JsonValue),

    // 404
    #[error("Not Found: {0}")]
    NotFound(JsonValue),

    // 422
    #[error("Unprocessable Entity: {0}")]
    UnprocessableEntity(JsonValue),

    // 500
    #[error("Internal Server Error")]
    InternalServerError,
}


impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            Error::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            Error::Forbidden(ref message) => HttpResponse::Forbidden().json(message),
            Error::NotFound(ref message) => HttpResponse::NotFound().json(message),
            Error::UnprocessableEntity(ref message) => {
                HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message)
            }
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
        }
    }
}

impl From<PgError> for Error {
    fn from(err: PgError) -> Self {
        match err {
            PgError::Database(db_err) => {
                eprintln!("Database Error: {:?}", db_err);

                Error::InternalServerError
            }

            _ => Error::InternalServerError
        }
    }
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        let mut err_map = JsonMap::new();

        for (field, errors) in errors.field_errors().iter() {
            let errors: Vec<JsonValue> = errors
                .iter()
                .map(| error | {
                    serde_json::json!(error.message)
                })
                .collect();

            err_map.insert(field.to_string(), serde_json::json!(errors));
        }

        Error::UnprocessableEntity(serde_json::json!({
            "errors": err_map
        }))
    }
}