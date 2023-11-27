use actix_web::{get, post, web, Responder};
use crate::handlers::hello_world::get_hello as gh;
#[get("/hello")]
pub async fn get_hello() -> impl Responder {
    gh().await
}