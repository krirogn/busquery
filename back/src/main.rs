use actix_web::{web::Data, App, HttpServer};
use colored::Colorize;
use sqlx::{MySql, Pool};

use crate::helpers::{db, env_check::env_check};

mod helpers;
mod routes;

pub struct AppState {
    db: Pool<MySql>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init
    println!("{}\n", "Starting server".bold());

    // .env loading
    dotenvy::dotenv().ok(); // For DATABASE_URL

    #[cfg(debug_assertions)]
    dotenvy::from_filename(".env.dev").ok();

    // Check for env variables
    env_check(&["SERVER_HOST", "SERVER_PORT", "DATABASE_URL"]).unwrap();

    // Get the database pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::get_pool(database_url).await.unwrap();

    // Check the DB
    db::check_db(&pool, "busquery", vec!["businesses"])
        .await
        .unwrap();

    // Start the actix server
    println!("========================================\n");
    HttpServer::new(move || {
        App::new()
            .configure(routes::config)
            .app_data(Data::new(AppState { db: pool.clone() }))
    })
    .bind((
        std::env::var("SERVER_HOST").expect("SERVER_HOST must be set"),
        std::env::var("SERVER_PORT")
            .expect("SERVER_PORT must be set")
            .parse::<u16>()
            .expect("SERVER_PORT couldn't be parsed as u16"),
    ))?
    .run()
    .await
}
