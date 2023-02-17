use actix_web::{web, App, HttpServer, HttpResponse};
use std::sync::{Arc, Mutex};

struct AppState {
    counter: Arc<Mutex<i32>>
}

async fn increment_counter(state: web::Data<AppState>) -> HttpResponse {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;

    HttpResponse::Ok().body(format!("Counter: {}", *counter))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Arc::new(Mutex::new(0))
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(increment_counter))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
