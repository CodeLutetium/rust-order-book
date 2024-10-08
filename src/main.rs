use actix_web::{web, App, HttpServer};
use order_book::check_username;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/api/usernames/{username}/valid",
            web::get().to(check_username),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
