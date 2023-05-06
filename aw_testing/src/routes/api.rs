use actix_web::HttpResponse;

pub async fn api() -> HttpResponse {
    HttpResponse::Ok().finish()
}