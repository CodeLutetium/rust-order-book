use anyhow::{Context, Ok};
use bigdecimal::BigDecimal;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use pbkdf2::Pbkdf2;
use rand_core::OsRng;
use sqlx::Result;
use sqlx::{
    postgres::{types::PgMoney, PgQueryResult},
    FromRow, PgPool,
};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct PostgresUser {
    user_id: Uuid,
    username: String,
    owned: i32,
    cash: PgMoney,
    password: String,
}

impl PostgresUser {
    pub fn new() -> PostgresUserBuilder {
        PostgresUserBuilder::new()
    }
}

pub struct PostgresUserBuilder {
    user_id: Uuid,
    username: Option<String>,
    owned: i32,
    cash: PgMoney,
    password: Option<String>,
}

impl PostgresUserBuilder {
    fn new() -> Self {
        PostgresUserBuilder {
            user_id: Uuid::new_v4(),
            username: None,
            owned: 0,
            cash: PgMoney(0),
            password: None,
        }
    }

    pub fn set_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn set_owned(mut self, owned: i32) -> Self {
        self.owned = owned;
        self
    }

    /// Pass the cash in as String, and set the cash value as PgMoney.
    pub fn set_cash(mut self, cash: String) -> Self {
        let pg_cash: BigDecimal = BigDecimal::from_str(cash.trim())
            .unwrap_or_else(|_| {
                println!("Invalid starting cash. Default value of 0 is set.");
                BigDecimal::from(0)
            })
            .max(BigDecimal::from(0));
        self.cash = PgMoney::from_bigdecimal(pg_cash, 2).expect("invalid cash amt");
        self
    }

    /// This function hashes the password. Pass the unhashed password into this function.
    pub fn set_password(mut self, password: String) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        self.password = Some(password_hash);
        self
    }

    pub fn build(self) -> PostgresUser {
        PostgresUser {
            user_id: self.user_id,
            username: self.username.unwrap(),
            owned: self.owned,
            cash: self.cash,
            password: self.password.unwrap(),
        }
    }
}

#[derive(Debug)]
/// User details to be returned via API call, will not return sensitive info
pub struct UserDetails {
    pub owned: i32,
    pub cash: f64,
}

// Returns true if username DOES NOT exists in database (valid username).
pub async fn is_username_available(pool: &PgPool, username: String) -> bool {
    let exists: (bool,) = sqlx::query_as("SELECT EXISTS (SELECT 1 FROM users WHERE username = $1)")
        .bind(username.as_str().trim())
        .fetch_one(pool)
        .await
        .unwrap();

    !exists.0
}

/// Inserts user into the database
pub async fn insert_user(pool: &PgPool, user: PostgresUser) -> Result<PgQueryResult> {
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
pub async fn update_cash(pool: &PgPool, user_id: Uuid, cash: PgMoney) -> Result<PgQueryResult> {
    sqlx::query("UPDATE users SET cash = $1 WHERE user_id = $2")
        .bind(cash)
        .bind(user_id)
        .execute(pool)
        .await
}

/// Updates user's owned in the database
pub async fn update_owned(pool: &PgPool, user_id: Uuid, owned: i32) -> Result<PgQueryResult> {
    sqlx::query("UPDATE users SET owned = $1 WHERE user_id = $2")
        .bind(owned)
        .bind(user_id)
        .execute(pool)
        .await
}

/// Returns user details from DB if username and password is valid. See: https://docs.rs/pbkdf2/latest/pbkdf2/
pub async fn authenticate_user(
    pool: &PgPool,
    username: String,
    password: String,
) -> anyhow::Result<UserDetails> {
    let maybe_user: Option<PostgresUser> =
        sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await?;

    // User does not exist
    if maybe_user.is_none() {
        return Err(anyhow::Error::msg("User not found"));
    }

    // User exists. Check whether password input matches db password hash.
    let user: PostgresUser = maybe_user.unwrap();
    let password_hash: PasswordHash<'_> = PasswordHash::new(&user.password).unwrap();
    let is_valid: bool = Pbkdf2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    if is_valid {
        let converted_cash: f64 = user
            .cash
            .to_bigdecimal(2)
            .to_string()
            .parse::<f64>()
            .with_context(|| "Failed to convert cash to f64")?;

        let return_user: UserDetails = UserDetails {
            owned: user.owned,
            cash: converted_cash,
        };
        Ok(return_user)
    } else {
        Err(anyhow::Error::msg("User not found"))
    }
}

/// Returns user details from DB given username only. This function is used for JWT authentication.
pub async fn get_user_details(pool: &PgPool, username: String) -> anyhow::Result<UserDetails> {
    let maybe_user: Option<PostgresUser> =
        sqlx::query_as("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await?;

    // User does not exist
    if maybe_user.is_none() {
        return Err(anyhow::Error::msg("User not found"));
    }

    let user: PostgresUser = maybe_user.unwrap();
    let converted_cash: f64 = user
        .cash
        .to_bigdecimal(2)
        .to_string()
        .parse::<f64>()
        .with_context(|| "Failed to convert cash to f64")?;

    let return_user: UserDetails = UserDetails {
        owned: user.owned,
        cash: converted_cash,
    };
    Ok(return_user)
}
