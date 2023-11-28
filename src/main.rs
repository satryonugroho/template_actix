mod config;
mod handlers;
mod models;
mod routes;
mod services;
mod state;

use std::env;
use env_logger;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use crate::routes::init_routes;
use state::AppState;
use config::db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let db_connection = establish_connection().await;
    HttpServer::new(move|| {
        let cors = Cors::permissive(); // Konfigurasi CORS

        App::new()
            .app_data(web::Data::new(AppState { db: db_connection.clone() }))
            .wrap(cors)
            .service(web::scope("/api").configure(init_routes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
