use bigdecimal::BigDecimal;
use sqlx::{Pool, Postgres};
use sqlx::{postgres::types::PgMoney, prelude::FromRow};
use sqlx::types::Uuid;
use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};

#[derive(Debug, FromRow)]
pub struct User {
    user_id: Uuid,
    username: String,
    owned: i32,
    cash: PgMoney,
    password: String,
}

impl User {
    pub fn new() -> UserBuilder {
        UserBuilder::new()
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn owned(&self) -> i32 {
        self.owned
    }

    pub fn cash(&self) -> BigDecimal {
        self.cash.to_bigdecimal(2)
    }

    /// Returns user details from DB if username and password is valid
    pub async fn login(pool: &Pool<Postgres>, username: String, password: String) -> Result<User, anyhow::Error> {
        let maybe_user:Option<User> = sqlx::query_as("SELECT * FROM users WHERE username = $1").bind(username).fetch_optional(pool).await?;

        if maybe_user.is_none() {
            return Err(anyhow::anyhow!("Invalid username or password"));
        }

        let user = maybe_user.unwrap();

        // https://docs.rs/pbkdf2/latest/pbkdf2/
        let password_hash = PasswordHash::new(&user.password).unwrap();
        let is_valid = Pbkdf2.verify_password(password.as_bytes(), &password_hash).is_ok();

        if is_valid {
            Ok(user)
        } else {
            Err(anyhow::anyhow!("Invalid username or password"))
        }
    }

    /// Inserts user into the database
    pub async fn insert_user(pool: Pool<Postgres>, user: User) {
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

    /// Updates user's cash in the database
    pub async fn update_cash(pool: Pool<Postgres>, user_id: Uuid, cash: PgMoney) {
        sqlx::query("UPDATE users SET cash = $1 WHERE user_id = $2")
            .bind(cash)
            .bind(user_id)
            .execute(&pool)
            .await
            .unwrap();
    }

    /// Updates user's owned in the database
    pub async fn update_owned(pool: Pool<Postgres>, user_id: Uuid, owned: i32) {
        sqlx::query("UPDATE users SET owned = $1 WHERE user_id = $2")
            .bind(owned)
            .bind(user_id)
            .execute(&pool)
            .await
            .unwrap();
    }
}

pub struct UserBuilder {
    user_id: Uuid,
    username: Option<String>,
    owned: i32,
    cash: PgMoney,
    password: Option<String>,
}

impl UserBuilder {
    fn new() -> Self {
        UserBuilder {
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

    pub fn set_cash(mut self, cash: PgMoney) -> Self {
        self.cash = cash;
        self
    }

    /// This function hashes the password. Pass the unhashed password into this function.
    pub fn set_password(mut self, password: String) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Pbkdf2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
        
        self.password = Some(password_hash);
        self
    }

    pub fn build(self) -> User {
        User {
            user_id: self.user_id,
            username: self.username.unwrap(),
            owned: self.owned,
            cash: self.cash,
            password: self.password.unwrap(),
        }
    }
}