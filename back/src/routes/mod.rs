use actix_web::web;

mod add_business;
mod get_business;
mod remove_business;
mod update_business;
mod v;
mod index;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(v::get_version).service(index::index).service(
        web::scope("/business")
            .service(add_business::add_business)
            .service(get_business::get_business)
            .service(update_business::update_business)
            .service(remove_business::remove_business),
    );
}
