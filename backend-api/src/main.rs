// backend-api/src/main.rs

use actix_web::{web, App, HttpServer, Responder};
use std::sync::Arc;

mod routes;
mod models;
mod auth;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = Arc::new(db::init_db().await?);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}