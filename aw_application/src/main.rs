// State antar Threads
// =================================================
// use actix_web::{web, get, App, HttpServer};
// use std::sync::Mutex;

// struct AppStateWithCounter {
//     counter: Mutex<i32>,
// }

// async fn index(data: web::Data<AppStateWithCounter>) -> String {
//     let mut counter = data.counter.lock().unwrap();
//     *counter += 1;

//     format!("Request number: {counter}!")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let counter = web::Data::new(AppStateWithCounter {
//         counter: Mutex::new(0)
//     });

//     HttpServer::new(move || {
//         App::new()
//             .app_data(counter.clone())
//             .route("/", web::get().to(index))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// Compose aplikasi menggunakan scope
// =================================================
// use actix_web::{web, get, App, HttpServer, Responder};

// #[get("/login")]
// async fn login() -> impl Responder {
//     "Login"
// }

// #[get("/logout")]
// async fn logout() -> impl Responder {
//     "Logout"
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         let scope_users = web::scope("/users")
//             .service(login)
//             .service(logout);

//         App::new()
//             .service(scope_users)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// Guard pada scope aplikasi
// =================================================
// use actix_web::{web, guard, App, HttpServer, Responder, HttpResponse};

// async fn manuall_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(
//                 web::scope("/guard")
//                     .guard(guard::Get())
//                     .route("/hey", web::to(|| manuall_hello()))
//             )
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// Aplikasi Modular dengan configuration
// =================================================
use actix_web::{web, App, HttpServer, HttpResponse};

use aw_application::{auth::scope_auth, security::scope_security};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(scope_security)
            .service(web::scope("/api").configure(scope_auth))
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("you are accessing /") }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// /
// /security
// /api/auth