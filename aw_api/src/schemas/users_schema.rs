use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;
use chrono::NaiveDateTime;
use uuid::Uuid;

use utoipa::ToSchema;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[_0-9a-zA-Z]{3,}$").unwrap();
}

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub bio: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct In<U> {
    pub user: U,
}

// Client Messages
#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct UserRegister {
    #[validate(
        length(
            min = 3,
            max = 20,
            message = "fails validation - must be 3-20 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: String,

    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email: String,

    #[validate(length(
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct UserLogin {
    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email: String,

    #[validate(length(
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct UserUpdate {
    #[validate(
        length(
            min = 1,
            max = 20,
            message = "fails validation - must be 1-20 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: Option<String>,

    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(
        min = 8,
        max = 72,
        message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: Option<String>,

    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
    pub bio: Option<String>,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct UserDelete {
    #[validate(
        length(
            min = 1,
            max = 20,
            message = "fails validation - must be 1-20 characters long"
        ),
        regex(
            path = "RE_USERNAME",
            message = "fails validation - is not only alphanumeric/underscore characters"
        )
    )]
    pub username: String
}