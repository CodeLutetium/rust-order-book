use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::authenticate_user;

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

pub async fn login(
    pool: web::Data<Pool<Postgres>>,
    login_input: web::Form<LoginInput>,
) -> HttpResponse {
    println!("Login request received for {}", login_input.username);

    match authenticate_user(
        pool.get_ref(),
        login_input.username.clone(),
        login_input.password.clone(),
    )
    .await {
        Ok(user_details) => {
            let response_body = serde_json::json!({
                "owned": user_details.owned,
                "cash": user_details.cash,
            });
            println!("User {} authenticated", login_input.username);
            HttpResponse::Ok().json(response_body)
        },
        Err(_) => {
            println!("Failed to authenticate user {}", login_input.username);
            HttpResponse::Unauthorized().body("Invalid username or password")
        }
    }
}
