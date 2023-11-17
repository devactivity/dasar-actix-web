use actix_web::HttpResponse;

use crate::errors::Error as AppError;

/// Get ping API response
///
/// Healthy API should always response OK
#[utoipa::path(
    get,
    path = "/ping",
    tag = "ping",
    responses(
        (status = 200, description = "Success"),
        (status = NOT_FOUND, description = "Not Found")
    )
)]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn third_party_api() -> Result<HttpResponse, AppError> {
    // Make a request to a third-party API
    let client = reqwest::Client::new();
    let response = client.get("https://api.example.com/data").send().await;

    match response {
        Ok(res) => {
            // Check if the request to the third-party API was successful
            if res.status().is_success() {
                // Return the third-party API response to the client
                Ok(HttpResponse::Ok().body(res.text().await.unwrap()))
            } else {
                // Return an error response
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}