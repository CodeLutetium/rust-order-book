use sqlx::Result;
use sqlx::{
    postgres::{types::PgMoney, PgQueryResult},
    FromRow, PgPool, Pool, Postgres,
};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct User {
    user_id: Uuid,
    username: String,
    owned: i32,
    cash: PgMoney,
    password: String,
}

#[derive(Debug)]
/// User details to be returned via API call, will not return sensitive info
pub struct UserDetails {
    owned: i32,
    cash: f64,
}

// Returns true if username exists in database.
pub async fn check_existing_user(pool: &PgPool, username: String) -> bool {
    let exists: (bool,) = sqlx::query_as("SELECT EXISTS (SELECT 1 FROM users WHERE username = $1)")
        .bind(username.as_str().trim())
        .fetch_one(pool)
        .await.unwrap();

    !exists.0
}

/// Inserts user into the database
pub async fn insert_user(pool: &PgPool, user: User) -> Result<PgQueryResult> {
    sqlx::query(
        "INSERT INTO users (user_id, username, owned, cash, password) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(user.user_id)
    .bind(user.username)
    .bind(user.owned)
    .bind(user.cash)
    .bind(user.password)
    .execute(pool)
    .await
}

/// Updates user's cash in the database
pub async fn update_cash(
    pool: Pool<Postgres>,
    user_id: Uuid,
    cash: PgMoney,
) -> Result<PgQueryResult> {
    sqlx::query("UPDATE users SET cash = $1 WHERE user_id = $2")
        .bind(cash)
        .bind(user_id)
        .execute(&pool)
        .await
}

/// Updates user's owned in the database
pub async fn update_owned(
    pool: Pool<Postgres>,
    user_id: Uuid,
    owned: i32,
) -> Result<PgQueryResult> {
    sqlx::query("UPDATE users SET owned = $1 WHERE user_id = $2")
        .bind(owned)
        .bind(user_id)
        .execute(&pool)
        .await
}

/// Returns user details from DB if username and password is valid
pub async fn login() -> Result<UserDetails> {
    // TODO
    Ok(UserDetails {
        owned: 0,
        cash: 0.0
    })
}