// Response Builder
// =======================================================
// use actix_web::{get, http::header::ContentType, HttpResponse};

// #[get("/")]
// async fn index() -> HttpResponse {
//     HttpResponse::Ok()
//         .content_type(ContentType::plaintext())
//         .insert_header(("X-Hdr", "sample"))
//         .body("data")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};

//     HttpServer::new(|| App::new().service(index))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }


// Response JSON
// =======================================================
// use actix_web::{get, Responder, Result, web};
// use serde::Serialize;

// #[derive(Serialize)]
// struct Myobj {
//     name: String
// }

// #[get("/{name}")]
// async fn index(name: web::Path<String>) -> Result<impl Responder> {
//     let obj = Myobj {
//         name: name.to_string()
//     };

//     Ok(web::Json(obj))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};

//     HttpServer::new(|| App::new().service(index))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }




// Content Encoding
// =======================================================
// use actix_web::{get, middleware, HttpResponse};

// #[get("/")]
// async fn index() -> HttpResponse {
//     HttpResponse::Ok().body("data")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};

//     HttpServer::new(|| App::new()
//         .wrap(middleware::Compress::default())
//         .service(index)
//     )
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }


// use actix_web::{get, middleware, http::header::ContentEncoding, HttpResponse};

// #[get("/")]
// async fn index() -> HttpResponse {
//     HttpResponse::Ok()
//         .insert_header(ContentEncoding::Identity)
//         .body("data")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};

//     HttpServer::new(|| App::new()
//         .wrap(middleware::Compress::default())
//         .service(index)
//     )
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }



// Compressed Payload
// =======================================================
use actix_web::{get, middleware, http::header::ContentEncoding, HttpResponse};

static HELLO_WORLD: &[u8] = &[
    0x1f, 0x8b, 0x08, 0x00, 0xa2, 0x30, 0x10, 0x5c, 0x00, 0x03, 0xcb, 0x48, 0xcd, 0xc9, 0xc9,
    0x57, 0x28, 0xcf, 0x2f, 0xca, 0x49, 0xe1, 0x02, 0x00, 0x2d, 0x3b, 0x08, 0xaf, 0x0c, 0x00,
    0x00, 0x00,
];

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(ContentEncoding::Gzip)
        .body(HELLO_WORLD)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new()
        .wrap(middleware::Compress::default())
        .service(index)
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
