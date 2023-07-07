use actix_web::{delete, web::Data, HttpRequest, HttpResponse, Responder};

use crate::AppState;

/// A route that removes a business from the database
#[delete("/{org}/remove")]
pub async fn remove_business(state: Data<AppState>, req: HttpRequest) -> impl Responder {
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

    // Remove the business
    match sqlx::query!("DELETE FROM businesses WHERE org=?", org)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().body("Business removed"),
        Err(error) => HttpResponse::InternalServerError().body(format!("{:?}", error)),
    }
}
