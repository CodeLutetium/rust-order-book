use actix_web::{web, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::{authenticate_user, postgres_functions::postgres_user::get_user_details, user_api::jwt::validate_jwt};

use super::create_jwt;

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    jwt: String,
    owned: i32,
    cash: f64,
}

/// Handles POST requests to /api/users/login
///
/// This function authenticates a user traditionally (with username and password) and returns a JWT token if the login is successful.
///
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
    .await
    {
        Ok(user_details) => {
            // Create jwt_token
            let jwt: String = create_jwt(login_input.username.clone()).unwrap();

            let response_body = serde_json::json!(LoginResponse {
                jwt,
                owned: user_details.owned,
                cash: user_details.cash,
            });
            println!("User {} authenticated", login_input.username);
            HttpResponse::Ok().json(response_body)
        }
        Err(_) => {
            println!("Failed to authenticate user {}", login_input.username);
            HttpResponse::Unauthorized().body("Invalid username or password")
        }
    }
}

pub async fn jwt_login(pool: web::Data<Pool<Postgres>>, req: HttpRequest) -> HttpResponse {
    // Extract JWT from request
    let jwt: String = match extract_jwt(req) {
        Ok(token) => token,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid JWT"),
    };
    
    // Get username from JWT
    let username: String = validate_jwt(&jwt).unwrap();
    println!("JWT Login request received for {}", username);

    match get_user_details(pool.get_ref(), username.clone()).await {
        Ok(user_details) => {
            // Refresh JWT
            let jwt: String = create_jwt(username.clone()).unwrap();

            let response_body = serde_json::json!(LoginResponse {
                jwt: jwt,
                owned: user_details.owned,
                cash: user_details.cash,
            });
            println!("User {} authenticated", username);
            HttpResponse::Ok().json(response_body)
        }
        Err(_) => {
            println!("Failed to authenticate user {}", jwt);
            HttpResponse::Unauthorized().body("Invalid JWT")
        }
    }
}

// Helper function to extract the token from the request
fn extract_jwt(req: HttpRequest) -> Result<String, Error>{
    if let Some(auth) = req.headers().get("Authorization") {
        let auth = auth.to_str().unwrap();
        let token = auth.replace("Bearer ", "");
        return Ok(token);
    }
    Err(actix_web::error::ErrorUnauthorized("Invalid token"))
}
