use actix_web::{get, web::Data, HttpResponse, Responder};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
struct BusinessesQuery {
    org: u32,
    notes: String,
}

/// A route that returns all the businesses in the database
#[get("/all")]
pub async fn get_business(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as!(
        BusinessesQuery,
        "SELECT org as `org: u32`, notes FROM businesses"
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
