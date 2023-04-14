use actix_web::{web, http::header, App, HttpServer, HttpRequest, Responder, HttpResponse};

async fn index(req: HttpRequest) -> HttpResponse {
    if let Some(_) = req.headers().get(header::CONTENT_TYPE) {
        HttpResponse::Ok().into()
    } else {
        HttpResponse::BadRequest().into()
    }
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| App::new()
        .route("/", web::get().to(index))
        .route("/echo", web::post().to(echo))
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self, header::ContentType},
        test
    };

    #[actix_web::test]
    async fn test_index_ok() {
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_http_request();

        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();

        let resp = index(req).await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_echo_ok() {
        let app = App::new().route("/echo", web::post().to(echo));
        let mut app = test::init_service(app).await;

        let req_body = "Hello, world!";
        let req = test::TestRequest::post()
            .uri("/echo")
            .insert_header((http::header::CONTENT_TYPE, "text/plain"))
            .set_payload(req_body)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let resp_body = test::read_body(resp).await;
        assert_eq!(resp_body, req_body);
    }
}