use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use sqlx::{postgres::types::PgMoney, FromRow, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserInput {
    username: String,
    owned: i32,
    cash: f64,
    password: String,
}


#[derive(Debug, FromRow)]
pub struct User {
    user_id: Uuid,
    username: String,
    owned: i32,
    cash: PgMoney,
    password: String,
}


pub async fn create_user(
    pool: web::Data<Pool<Postgres>>,
    user: web::Form<UserInput>
) -> HttpResponse {
    println!("request received");
    println!("{}", format!("{:#?}", user));
    

    HttpResponse::Ok().body(format!("User successfully created"))
}

/// Inserts user into the database
async fn insert_user(pool: Pool<Postgres>, user: User) {
    sqlx::query("INSERT INTO users (user_id, username, owned, cash, password) VALUES ($1, $2, $3, $4, $5)")
        .bind(user.user_id)
        .bind(user.username)
        .bind(user.owned)
        .bind(user.cash)
        .bind(user.password)
        .execute(&pool)
        .await
        .unwrap();
}