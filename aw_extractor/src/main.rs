use actix_web::{get, post, web,error, App, HttpServer, HttpRequest, Result, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    username: String,
}

/// extract path info using serde
#[get("/users/v1/{user_id}/{username}")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.username, info.user_id
    ))
}

#[get("/users/v2/{user_id}/{username}")] // <- define path parameters
async fn index2(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, username) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", username, user_id))
}

#[get("/users/v3/{user_id}/{username}")] // <- define path parameters
async fn index3(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("username").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("user_id").parse().unwrap();

    Ok(format!("Welcome {}, user_id {}!", name, userid))
}


#[derive(Deserialize)]
struct InfoQuery {
    username: String,
}

#[get("/query")]
async fn index_query(info: web::Query<InfoQuery>) -> String {
    format!("Welcome {}!", info.username)
}

#[derive(Deserialize)]
struct InfoJSON {
    username: String,
}

// #[post("/submit")]
async fn index_submit(info: web::Json<InfoJSON>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}


#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[post("/form")]
async fn index_form(info: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(1024)
            .error_handler(| err, _req | {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                .into()
            });

        App::new()
            .service(index)
            .service(index2)
            .service(index3)
            .service(index_form)
            .service(index_query)
            .service(
                web::resource("/submit")
                    .app_data(json_config)
                    .route(web::post().to(index_submit))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
