// Serve individual file
// ==============================================================================
/* use actix_web::{
    web, App,
    HttpServer, HttpRequest,
    Result
};
use actix_files::NamedFile;
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} */


// Serve individual file mod
// ==============================================================================
/* use actix_web::{
    web, App,
    HttpServer, HttpRequest,
    Result, error
};
use regex::Regex;
use std::io;
use actix_files::NamedFile;
use std::path::{Path, PathBuf};

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let filename: PathBuf = req.match_info().query("filename").parse().unwrap();

    let filename_str = filename.to_string_lossy();
    let filename_regex = Regex::new(r"^[\w\./_]+$").unwrap();

    if !filename_regex.is_match(&filename_str) {
        return Err(error::ErrorNotFound("invalid filename"));
    }

    let sanitized_filename = sanitize_filename::sanitize(&filename_str);

    let base_dir = "./static";
    let path = Path::new(&base_dir).join(&sanitized_filename);

    if !path.starts_with(&base_dir) {
        return Err(error::ErrorNotFound("invalid file path"));
    }

    match NamedFile::open(path) {
        Ok(file) => Ok(file),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                Err(error::ErrorNotFound("file not found"))
            } else {
                Err(error::ErrorInternalServerError("internal server error"))
            }
        }
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} */



// Serve directory listing
// ==============================================================================
use actix_web::{
    App,
   HttpServer,
   middleware
};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
       App::new()
           .wrap(middleware::DefaultHeaders::new().add(("X-Content-Type-Options", "nosniff")))
           .service(
               fs::Files::new("/static", ".")
                   .show_files_listing()
                   .use_last_modified(true)
                   .disable_content_disposition()
           )
           .default_service(
               fs::Files::new("/static", ".")
                   .index_file("index.html")
                   .use_last_modified(true)
                   .disable_content_disposition()
           )
   })
   .bind(("127.0.0.1", 8080))?
   .run()
   .await
}