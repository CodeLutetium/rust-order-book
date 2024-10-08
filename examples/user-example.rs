// Example code on creating a user and logging in a user.

use std::{env, io};
use dotenv::dotenv;
use sqlx::{migrate, postgres::{types::PgMoney, PgPoolOptions}};
use order_book::User;
use bigdecimal::BigDecimal;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    // Create pg connection pool
    dotenv().ok();
    let pg_connection_str: String = env::var("POSTGRES_URL").unwrap();
    let pool = PgPoolOptions::new().max_connections(5).connect(&pg_connection_str).await?;

    // Run migrations
    migrate!("./migrations").run(&pool).await?;

    loop {
        println!("====================");
        println!("|--  MAIN MENU   --|");
        println!("====================");
        println!("1) Create new account");
        println!("2) Login");
        println!("Press any other key to exit");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                // Create new account
                println!("Enter username:");
                let mut username: String = String::new();
                io::stdin().read_line(&mut username).unwrap();

                // Check if username already exists
                let exists: (bool, ) = sqlx::query_as("SELECT EXISTS (SELECT 1 FROM users WHERE username = $1)").bind(username.trim()).fetch_one(&pool).await?;
                if exists == (true, ) {
                    println!("Username already exists. Please try again.");
                    continue;
                }

                println!("Enter password:");
                let mut password: String = String::new();
                io::stdin().read_line(&mut password).unwrap();

                println!("Enter starting cash ($0 will be given as default): ");
                let mut starting_cash: String = String::new();
                io::stdin().read_line(&mut starting_cash).unwrap();

                // Check starting cash is valid
                let cash: BigDecimal = BigDecimal::from_str(starting_cash.trim()).unwrap_or_else(|_| {
                    println!("Invalid starting cash. Default value of 0 is set.");
                    BigDecimal::from(0)
                }).max(BigDecimal::from(0));
                // let cash: i64 = starting_cash.trim().parse::<f64>().unwrap_or_else(|_| {
                //     println!("Invalid starting cash. Default value of 0 is set.");
                //     0.0
                // }).max(0.0) as i64;

                let user = User::new()
                    .set_username(username.trim().to_string())
                    .set_password(password.trim().to_string())
                    .set_cash(PgMoney::from_bigdecimal(cash, 2).expect("Invalid cash"))
                    .build();

                User::insert_user(pool.clone(), user).await;

                username = username.trim().to_string();
                println!("{username} successfully created");
            }

            "2" => {
                // Login
                println!("Enter username:");
                let mut username: String = String::new();
                io::stdin().read_line(&mut username).unwrap();

                println!("Enter password:");
                let mut password: String = String::new();
                io::stdin().read_line(&mut password).unwrap();

                match User::login(&pool, username.trim().to_string(), password.trim().to_string()).await {
                    Ok(user) => {
                        println!("Welcome back, {}!", user.username());
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                    
                }
            }

            _ => {
                println!("Exiting now");
                break;
            }
        }
    }

    Ok(())
}