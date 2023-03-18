// Path info
// ===========================================================

/* use actix_web::{get, web, App, HttpServer, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// extract path info using serde
#[get("/{username}/index.html")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[get("/{username}/{id}/index.html")] // <- define path parameters
async fn index2(info: web::Path<(String, u32)>) -> Result<String> {
    let info = info.into_inner();
    Ok(format!("Welcome {}! id: {}", info.0, info.1))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(index2))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
} */



// Generate URL
// ===========================================================

/* use actix_web::{get, http::header, HttpRequest, HttpResponse, Result, guard};

#[get("/test/")]
async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("baz", ["1", "2", "3"])?;

    Ok(HttpResponse::Found().insert_header((header::LOCATION, url.as_str())).finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/test/{a}/{b}/{c}")
                .name("foo")
                .guard(guard::Get())
                .to(HttpResponse::Ok)
            )
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} */



/* use actix_web::{get, web, HttpRequest, Responder,  HttpResponse};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let url = req.url_for("youtube", ["lkawefoi"]).unwrap();

    url.to_string()
}

#[get("/show", name = "show_users")]
async fn show_users(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

async fn generate_url(req: HttpRequest) -> impl Responder {
    let url = req.url_for("show_users", &[""]);

    match url {
        Ok(url) => HttpResponse::Ok().body(format!("generate URL: {}", url)),
        Err(_) => HttpResponse::InternalServerError().body("Failed to generate URL")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(index)
            .external_resource("youtube", "https://youtube.com/watch/{video_id}")
            .service(web::scope("/users").service(show_users))
            .route("/generate", web::get().to(generate_url))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
 */


// Path normalization
// ===========================================================

/* use actix_web::{ middleware, get, http::Method, HttpResponse};

#[get("/resource")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .service(index)
            .default_service(web::route().method(Method::GET))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} */


// Custom guard
// ===========================================================

/* use actix_web::{ http, guard::{self, Guard, GuardContext},  HttpResponse};

struct ContetTypeHeader;

impl Guard for ContetTypeHeader {
    fn check(&self, req: &GuardContext<'_>) -> bool {
        req.head()
            .headers()
            .contains_key(http::header::CONTENT_TYPE)
    }
}

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::route()
                    .guard(ContetTypeHeader)
                    .to(index)
            )
            .route(
                "/notallowed",
                web::route().guard(guard::Not(guard::Get()))
                .to(HttpResponse::MethodNotAllowed)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} */



// Override default response
// ===========================================================

use actix_web::{ guard, HttpResponse};

async fn index() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(HttpResponse::MethodNotAllowed)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}