// use actix_web::{web, App, HttpResponse, HttpServer};

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(||
//         App::new().route("/", web::get().to(HttpResponse::Ok))
//     )
//     .workers(2) // <= set worker manually
//     // .bind(("127.0.0.1", 8080))? // alternative
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }


// rustls, openssl
// Perintah membuat file `cert.pem` & `key.pem`
//
// $ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
//   -days 365 -sha256 -subj "/C=ID/ST=DKI/L=Jakarta/O=RustLang/OU=Org/CN=localhost"
//
use actix_web::{get, App, Responder, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index() -> impl Responder {
    "Welcome HTTPS!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();

    builder
        .set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().service(index))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}