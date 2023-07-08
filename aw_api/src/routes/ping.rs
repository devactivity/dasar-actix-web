use actix_web::HttpResponse;

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