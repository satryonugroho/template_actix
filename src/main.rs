mod config;
mod handlers;
mod models;
mod routes;
mod services;
mod state;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use crate::routes::init_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); // Konfigurasi CORS

        App::new()
            .wrap(cors)
            .service(web::scope("/api").configure(init_routes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
