use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct Business {
    org: u64,
    notes: String,
}

#[post("/add")]
pub async fn add_business(state: Data<AppState>, info: Json<Business>) -> impl Responder {
    // Check if business is already in DB
    if sqlx::query!("SELECT id FROM businesses WHERE org=?", info.org)
        .fetch_one(&state.db)
        .await
        .is_ok()
    {
        return HttpResponse::BadRequest().body("This business is already saved");
    }

    // Add business info to DB
    match sqlx::query!(
        "INSERT INTO businesses VALUES (NULL, ?, ?)",
        info.org,
        info.notes
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => {
            return HttpResponse::Ok().body("Business successfully added");
        }
        Err(error) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", error));
        }
    }
}
