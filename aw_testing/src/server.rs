use std::net::TcpListener;
use actix_web::{
    web, dev::Server, App, HttpServer
};
use crate::routes::api;

pub fn start(
    listener: TcpListener
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(api))
            .route("/", web::post().to(api))
    })
    .listen(listener)?
    .run();

    Ok(server)
}