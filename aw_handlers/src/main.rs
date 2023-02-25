// - `FromRequest` Trait
// - `HttpResponse` Type
// - `Responder` Trait

// Handler return types
// ===========================================
async fn index(_req: HttpRequest) -> String {
    format!("Hello world!")
}

async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    format!("{} {} {} {}", path.0, path.1, json.id, json.username)
}


// Custom handler return type
// ===========================================
use actix_web::{
    App, HttpServer, get,
    body::BoxBody, http::header::ContentType,
    HttpRequest, HttpResponse, Responder
};
use serde::Serialize;

#[derive(Serialize)]
struct MyCustomReturn {
    name: &'static str,
}

impl Responder for MyCustomReturn {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
    }
}

#[get("/")]
async fn index() -> impl Responder {
    MyCustomReturn { name: "rustlang" }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


// Streaming body response
// ===========================================
use actix_web::{get, web, App, Error, HttpResponse, HttpServer};
use futures::{future::ok, stream::once};

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(
        ok::<_, Error>(web::Bytes::from_static(b"rust"))
    ); // Future

    HttpResponse::Ok()
    .content_type("application/json")
    .streaming(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(stream))
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


// 2 return types
// ===========================================
use actix_web::Either;

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

fn is_valid() -> bool {
    // do something here
    true
}

async fn index() -> RegisterResult {
    if is_valid() {
        Either::Left(HttpResponse::BadRequest().body("bad data"))
    } else {
        Either::Right(Ok("success!"))
    }
}