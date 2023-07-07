use colored::Colorize;
use spinners::{Spinner, Spinners};
use sqlx::{mysql::MySqlPoolOptions, FromRow, MySql, Pool};

/// Gets a MySQL connection pool from a MySQL URL
///
/// # Arguments
///
/// * `database_url` - A string that holds a connection URL to a
/// MySQL server in this format `mysql://user:pass@server/database`
pub async fn get_pool(database_url: String) -> Result<Pool<MySql>, ()> {
    let mut sp = Spinner::new(Spinners::Line, "Connecting to the database".into());

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    sp.stop();
    print!("\r");

    let pool: Result<Pool<MySql>, ()> = match pool {
        Ok(db_pool) => {
            println!("Successfully connected to the database");
            println!(
                "  - {}",
                &database_url
                    .split('/')
                    .last()
                    .expect("SQL URI was malformed")
                    .green()
            );

            Ok(db_pool)
        }
        Err(error_message) => {
            std::panic::set_hook(Box::new(move |_| {
                println!("Database connection failed  ");
                print!("Error building a connection pool: ");
                println!("{}", error_message);
                println!("{}", "Stopping server".bold());
            }));

            Err(())
        }
    };

    pool
}

#[derive(FromRow)]
struct CountTablesQuery {
    tables: i64,
}

/// Checks if a MySQL database has a set of tables
///
/// # Arguments
///
/// * `pool` - A reference to a MySQL pool
/// * `database` - A string slice that holds the name of the database
/// * `tables` - A vector of string slices witch holds table names to be checked
///
/// # Examples
///
/// ```
/// db::check_db(&pool, "databaseName", vec!["table1", "table2"])
///     .await
///     .expect("The database is corrupt or not set up correctly");
/// ```
pub async fn check_db(pool: &Pool<MySql>, database: &str, tables: Vec<&str>) -> Result<(), ()> {
    let mut sp = Spinner::new(Spinners::Line, "Checking the DB".into());

    // Check the tables
    match sqlx::query_as::<_, CountTablesQuery>(&format!(
        "SELECT COUNT(*) AS tables
        FROM information_schema.tables
        WHERE TABLE_SCHEMA = '{}'
        AND TABLE_NAME IN ({})",
        database,
        tables
            .iter()
            .map(|table| format!("'{table}'"))
            .collect::<Vec<String>>()
            .join(", ")
    ))
    .fetch_one(pool)
    .await
    {
        Ok(info) => {
            if info.tables != tables.len() as i64 {
                return check_db_panic(&mut sp, tables);
            }
        }
        Err(_) => {
            return check_db_panic(&mut sp, tables);
        }
    }

    sp.stop();
    print!("\r");

    println!("Successfully checked the database tables");
    println!(
        "{}",
        tables
            .iter()
            .map(|table| format!("  - {}", table.green()))
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(())
}

/// A private helper function to create a panic that prints out
/// an error message to stdio
///
/// # Arguments
///
/// * `sp` - A mutable reference to a Spinner instance
/// * `tables` - A vector of string slices that hold table names to be printed
fn check_db_panic(sp: &mut Spinner, tables: Vec<&str>) -> Result<(), ()> {
    sp.stop();
    print!("\r");

    let tables = tables
        .iter()
        .map(|table| table.to_string())
        .collect::<Vec<String>>();

    std::panic::set_hook(Box::new(move |_| {
        println!("                   "); // clear out spinner
        println!("{}", "The database isn't correctly set up".red());
        println!("Check that all these tables exist");
        println!(
            "{}",
            tables
                .iter()
                .map(|table| format!("  - {}", table.green()))
                .collect::<Vec<String>>()
                .join("\n")
        );
        println!(
            "Please run the {} query to correctly set up the database",
            "setup.sql".underline()
        );
        println!("{}", "Stopping server".bold());
    }));

    Err(())
}
