use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct Business {
    notes: String,
}

/// A route that adds a new business to the database
#[post("/{org}/add")]
pub async fn add_business(
    state: Data<AppState>,
    req: HttpRequest,
    info: Json<Business>,
) -> impl Responder {
    // Request error handling
    let business_query = req.match_info().query("org").parse::<u32>();
    let org = match business_query {
        Ok(path_info) => path_info,
        Err(_) => return HttpResponse::BadRequest().body("The query was malformed"),
    };

    // Check if business is already in DB
    if sqlx::query!("SELECT id FROM businesses WHERE org=?", org)
        .fetch_one(&state.db)
        .await
        .is_ok()
    {
        return HttpResponse::BadRequest().body("This business is already saved");
    }

    // Add business info to DB
    match sqlx::query!(
        "INSERT INTO businesses VALUES (NULL, ?, ?)",
        org,
        info.notes
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().body("Business successfully added"),
        Err(error) => HttpResponse::InternalServerError().body(format!("{:?}", error)),
    }
}
