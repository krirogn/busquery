use actix_web::{HttpResponse, Responder, get};

#[get("/list")]
pub async fn get_business() -> impl Responder {
    HttpResponse::NotImplemented().body("This endpoint is not yet implemented")
}
