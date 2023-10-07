use serde_json::{Map as JsonMap, Value as JsonValue};
use actix_web::HttpResponse;
use validator::ValidationErrors;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2
};

use crate::errors::Error as AppError;

pub fn validation_errors_response(validation_errors: &ValidationErrors) -> HttpResponse {
    let mut cleaned_errors = JsonMap::new();

    for (field, field_errors) in validation_errors.field_errors().iter() {
        let mut cleaned_field_errors = Vec::new();

        for error in field_errors.iter() {
            let cleaned_error = serde_json::json!({
                "code": error.code,
                "message": error.message
            });

            cleaned_field_errors.push(cleaned_error);
        }

        cleaned_errors.insert(field.to_string(), JsonValue::Array(cleaned_field_errors));
    }

    let error_response = serde_json::json!({
        "error": "Validation failed",
        "details": cleaned_errors
    });

    HttpResponse::BadRequest().json(error_response)
}

pub fn hash_password(password: &[u8]) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password, &salt)
        .map_err(| _ | AppError::InternalServerError)?;

    Ok(password_hash.to_string())
}