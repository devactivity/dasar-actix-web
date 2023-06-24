#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

mod schema;
mod models;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Could not get DB connection from pool");

    let results = diesel::sql_query("SELECT name FROM items")
        .load::<models::Item>(&mut conn)
        .expect("Error executing query")
        .iter()
        .map(| item | item.name.clone())
        .collect::<Vec<String>>();

    HttpResponse::Ok().body(format!("Result: {:?}", results))
}

async fn create_item(
    pool: web::Data<DbPool>,
    item_name: web::Json<models::CreateItemPayload>
) -> impl Responder {
    let mut conn = pool.get().expect("Could not get DB connection from pool");

    let new_item = models::NewItem {
        id: Some(Uuid::new_v4()),
        name: item_name.name.clone()
    };

    diesel::insert_into(schema::items::table)
        .values(&new_item)
        .execute(&mut conn)
        .expect("Error creating item");

        HttpResponse::Ok().body("Item created successfully")
}

// Define a handler for updating a record
async fn update_item(
    pool: web::Data<DbPool>,
    item_id: web::Path<Uuid>,
    item_info: web::Json<models::UpdateItemRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let rows_updated = diesel::update(schema::items::table)
        .filter(schema::items::id.eq(item_id.clone()))
        .set(schema::items::name.eq(&item_info.item_name))
        .execute(&mut conn)
        .expect("Error updating item");

    if rows_updated > 0 {
        HttpResponse::Ok().body("Item updated successfully")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

// Define a handler for getting a single record
async fn get_item(
    pool: web::Data<DbPool>,
    item_id: web::Path<Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let item = schema::items::table
        .filter(schema::items::id.eq(item_id.into_inner()))
        .select(schema::items::name.nullable())
        .first::<Option<String>>(&mut conn)
        .optional()
        .expect("Error retrieving item");

    if let Some(name) = item {
        match name {
            Some(name) => HttpResponse::Ok().body(format!("Item: {}", name)),
            None => HttpResponse::NotFound().body("Item not found"),
        }
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

// Define a handler for deleting a record
async fn delete_item(
    pool: web::Data<DbPool>,
    item_id: web::Path<Uuid>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    let rows_deleted = diesel::delete(schema::items::table)
        .filter(schema::items::id.eq(item_id.into_inner()))
        .execute(&mut conn)
        .expect("Error deleting item");

    if rows_deleted > 0 {
        HttpResponse::Ok().body("Item deleted successfully")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_url must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/", web::post().to(create_item))
            .route("/{id}", web::get().to(get_item))
            .route("/{id}", web::put().to(update_item))
            .route("/{id}", web::delete().to(delete_item))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}