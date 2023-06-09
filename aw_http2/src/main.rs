use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn index(req: HttpRequest) -> impl Responder {
    println!("{:#?}", req.head());

    let response = HttpResponse::Ok().body("Hello world\n");

    // Print the response ============================================
    // If you want to confirm the protocol being used for the response,
    // you can inspect it by examining the actual network traffic or
    // using a tool like Wireshark. It will show that the response is
    // sent over the wire using the HTTP/2 protocol.
    println!("{:#?}", response); // <= The `Debug` implementation prints the response in the HTTP/1.1 format.

    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();

    builder
        .set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
