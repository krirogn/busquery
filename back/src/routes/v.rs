use actix_web::{get, HttpResponse, Responder};

/// A route that returns the version of the API
#[get("/v")]
async fn get_version() -> impl Responder {
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}
