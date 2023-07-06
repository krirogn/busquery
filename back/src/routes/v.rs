use actix_web::{get, HttpResponse, Responder};

#[get("/v")]
async fn get_version() -> impl Responder {
    HttpResponse::Ok().body("0.1.0")
}
