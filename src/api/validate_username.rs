use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::is_username_available;

#[derive(Serialize)]
struct UsernameValidationResponse {
    is_valid: bool,
}

pub async fn check_username(
    pool: web::Data<Pool<Postgres>>,
    username: web::Path<String>,
) -> impl Responder {
    println!("Checking username: {}", username);
    
    let is_valid: bool = is_username_available(pool.get_ref(), username.to_string()).await;

    let response: UsernameValidationResponse = UsernameValidationResponse { is_valid };

    HttpResponse::Ok().json(response)
}
