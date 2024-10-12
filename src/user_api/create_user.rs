use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::{is_username_available, insert_user, PostgresUser};

#[derive(Debug, Deserialize)]
pub struct UserInput {
    username: String,
    owned: i32,
    cash: String,
    password: String,
}

pub async fn create_user(
    pool: web::Data<Pool<Postgres>>,
    user: web::Form<UserInput>,
) -> HttpResponse {
    println!("request received");
    // println!("{}", format!("{:#?}", user));

    // Second layer of check to make sure user does not exist
    if is_username_available(pool.get_ref(), user.username.clone()).await == false {
        return HttpResponse::BadRequest().body(format!("Error: User exists"));
    }

    // Create user object
    let postgres_user: PostgresUser = PostgresUser::new()
        .set_username(user.username.to_owned())
        .set_owned(user.owned)
        .set_cash(user.cash.to_owned())
        .set_password(user.password.to_owned())
        .build();

    // Insert into DB
    match insert_user(pool.get_ref(), postgres_user).await {
        Ok(_) => {
            let response_body = serde_json::json!({
                "owned": user.owned,
                "cash": user.cash,
            });
            HttpResponse::Ok().json(response_body)
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("Error creating user. Error code: {}", e));
        }
    }
}
