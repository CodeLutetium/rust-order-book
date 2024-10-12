use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::authenticate_user;

use super::create_jwt;

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    jwt_token: String,
    owned: i32,
    cash: f64,
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
            // Create jwt_token
            let jwt_token: String = create_jwt(login_input.username.clone()).unwrap();

            let response_body = serde_json::json!(LoginResponse{
                jwt_token,
                owned: user_details.owned,
                cash: user_details.cash,
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
