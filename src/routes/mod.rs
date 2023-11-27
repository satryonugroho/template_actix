use actix_web::web;

pub mod hello_world;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello_world::get_hello);
}
