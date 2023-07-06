use actix_web::web;

mod v;
mod add_business;
mod get_business;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(v::get_version)
        .service(add_business::add_business)
        .service(get_business::get_business);
}
