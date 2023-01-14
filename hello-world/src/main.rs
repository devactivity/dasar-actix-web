use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[get("/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greet)
            .route("/hey", web::post().to(manual_hello)) // Guard trait
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
