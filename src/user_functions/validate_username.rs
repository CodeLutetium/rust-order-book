use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::{Pool, Postgres};

#[derive(Serialize)]
struct UsernameValidationResponse {
    is_valid: bool,
}

pub async fn check_username(
    pool: web::Data<Pool<Postgres>>,
    username: web::Path<String>,
) -> impl Responder {
    println!("Checking username: {}", username);
    
    let exists: (bool,) = sqlx::query_as("SELECT EXISTS (SELECT 1 FROM users WHERE username = $1)")
        .bind(username.as_str().trim())
        .fetch_one(pool.get_ref())
        .await.unwrap();

    let is_valid: bool = !exists.0;

    let response: UsernameValidationResponse = UsernameValidationResponse { is_valid };

    HttpResponse::Ok().json(response)
}
