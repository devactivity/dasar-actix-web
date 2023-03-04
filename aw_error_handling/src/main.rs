// Pengantar ResponseError
// ==========================================================

/* use actix_web::{
    error, web, App, HttpServer, Result
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
#[display(fmt = "Invalid name: {}", name)]
struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

async fn index() -> Result<&'static str, MyError> {
    Err(MyError { name: "rustlang" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} */


// Custom Error Response
// ==========================================================

/* use actix_web::{
    error, web, App, HttpServer, Result,
    http::{header::ContentType, StatusCode},
    HttpResponse
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "Internal error")]
    InternalError,

    #[display(fmt = "Bad request")]
    BadClientData,

    #[display(fmt = "Timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

async fn index() -> Result<&'static str, MyError> {
    Err(MyError::Timeout)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
 */


// Error Helper functions
// ==========================================================

/*  use actix_web::{
    error, web, App, HttpServer, Result
};

struct MyError {
    name: &'static str,
}

async fn index() -> Result<&'static str> {
    let result: Result<&'static str, MyError> = Err(MyError { name: "rustlang" });

    Ok(result.map_err(| e | error::ErrorBadRequest(e.name))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} */


// Generic Error
// ==========================================================

use actix_web::{
    error,  App, HttpServer, Result,
    http::{header::ContentType, StatusCode},
    HttpResponse, post
};
use derive_more::{Display, Error};
use log::error;
use env_logger;

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,

    #[display(fmt = "Invalid input provided.")]
    InvalidInput,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_message = self.to_string();
        error!("{} - Status code: {}", error_message, status_code);


        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            UserError::InvalidInput => StatusCode::BAD_REQUEST,
        }
    }
}


#[derive(Debug, serde::Deserialize)]
struct MyData {
    value: String,
}

fn do_thing_that_fails() -> Result<(), UserError> {
    Err(UserError::InternalError)
}


#[post("/process")]
async fn process(data: actix_web::web::Json<MyData>) -> Result<&'static str, UserError> {
    if data.value.is_empty() {
        return Err(UserError::InvalidInput)
    }

    do_thing_that_fails().map_err(| _e | UserError::InternalError)?;
    Ok("success!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(process)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}