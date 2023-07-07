use actix_web::{
    put,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder,
};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
pub struct BusinessesQuery {
    notes: String,
}

/// A route that updates the notes of a business in the database
#[put("/{org}/update")]
pub async fn update_business(
    state: Data<AppState>,
    req: HttpRequest,
    info: Json<BusinessesQuery>,
) -> impl Responder {
    // Request error handling
    let business_query = req.match_info().query("org").parse::<u32>();
    let org = match business_query {
        Ok(path_info) => path_info,
        Err(_) => return HttpResponse::BadRequest().body("The query was malformed"),
    };

    // Check if the business exists
    if sqlx::query!("SELECT id FROM businesses WHERE org=?", org)
        .fetch_one(&state.db)
        .await
        .is_err()
    {
        return HttpResponse::BadRequest().body("Couldn't find the business");
    }

    // Update the business
    match sqlx::query!("UPDATE businesses SET notes=? WHERE org=?", info.notes, org)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Business updated"),
        Err(error) => HttpResponse::InternalServerError().body(format!("{:?}", error)),
    }
}
