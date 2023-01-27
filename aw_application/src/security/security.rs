use actix_web::{web, HttpResponse};

pub fn scope_security(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/security")
            .route(web::get().to(|| async { HttpResponse::Ok().body("you are accessing /security") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
    );
}


// Contoh data state khusus pada module
// ================================
// pub struct MyData {
//     count: std::cell::Cell<usize>,
// }

// async fn handler(counter: web::Data<MyData>) -> String {
//     // show counter value from app data
//     counter.count.set(counter.count.get() + 1);
//     format!("{}",counter.count.get().to_string())
// }

// pub fn scope_security(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/security")
//             .app_data(web::Data::new(MyData { count: Default::default() }))
//             .route(web::get().to(handler))
//             .route(web::head().to(HttpResponse::MethodNotAllowed)),
//     );
// }