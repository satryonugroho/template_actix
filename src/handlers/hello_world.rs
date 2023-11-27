use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn get_hello() -> impl Responder {
    HttpResponse::Ok().json(json!({"payload": "Hello form MKT"}))
}