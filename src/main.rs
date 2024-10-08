use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use order_book::{check_username, create_user};
use sqlx::{migrate, postgres::PgPoolOptions};
use std::{env, io};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create pg connection pool
    dotenv().ok();
    let pg_connection_str: String = env::var("POSTGRES_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_connection_str)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Run migrations
    migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    println!("Migrations complete, ready to accept requests");

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).route(
            "/api/usernames/{username}/valid",
            web::get().to(check_username),
        ).route("/api/create-user", web::post().to(create_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
