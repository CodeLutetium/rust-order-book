use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use order_book::{check_username, create_user, get_order_book, jwt_login, login, OrderBook};
use sqlx::{migrate, postgres::PgPoolOptions};
use std::{ env, io, sync::{Arc, Mutex}};

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

    let order_book: Arc<Mutex<OrderBook>> = Arc::new(Mutex::new(OrderBook::new()));
    {
        let mut order_book_ref = order_book.lock().unwrap();
        let order = order_book::OrderBuilder::new()
            .price(100.0)
            .quantity(10)
            .order_type(order_book::OrderType::Sell)
            .build();
        order_book_ref.add_order(order).unwrap();
    }

    println!("Order book initialized");

    HttpServer::new(move || {
        // Set up CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(order_book.clone()))
            .route("/api/users/{username}/valid", web::get().to(check_username))
            .route("/api/users/create-user", web::post().to(create_user))
            .route("/api/users/login", web::post().to(login))
            .route("/api/users/get-user", web::get().to(jwt_login))
            .route("/api/orders/get", web::get().to(get_order_book))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
