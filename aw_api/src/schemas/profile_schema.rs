use serde::{Serialize, Deserialize};
use validator::Validate;
use utoipa::ToSchema;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[_0-9a-zA-Z]{3,}$").unwrap();
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfileResponse {
    pub profile: ProfileResponseInner,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfileResponseInner {
    pub username: String,
    pub bio: Option<String>,
    pub following: bool,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct Profile {
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
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct ProfileFollow {
    #[validate(email(message = "fails validation - is not a valid email address"))]
    pub email: String
}