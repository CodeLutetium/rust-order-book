use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::{insert_user, is_username_available, create_jwt, PostgresUser};

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
        .set_username(user.username.clone())
        .set_owned(user.owned)
        .set_cash(user.cash.to_owned())
        .set_password(user.password.to_owned())
        .build();

    // Insert into DB
    match insert_user(pool.get_ref(), postgres_user).await {
        Ok(_) => {
            let jwt: String = create_jwt(user.username.clone()).unwrap();

            let response_body = serde_json::json!({
                "jwt": jwt,
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
