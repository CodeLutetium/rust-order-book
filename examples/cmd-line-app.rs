// // Previous implementation using command line

// use order_book::{Order, OrderBook, OrderBuilder, OrderType};
// use sqlx::{migrate, postgres::PgPoolOptions};
// use std::io;
// use dotenv::dotenv;
// use std::env;


// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error>{
//     // Create pg connection pool
//     dotenv().ok();
//     let pg_connection_str: String = env::var("POSTGRES_URL").unwrap();
//     let pool = PgPoolOptions::new().max_connections(5).connect(&pg_connection_str).await?;

//     // Run migrations
//     migrate!("./migrations").run(&pool).await?;

//     // Initialize order book
//     let mut order_book: OrderBook = OrderBook::new();

//     println!("Welcome to the order book project! This simple program aims to simulate a simple order book.");
//     loop {
//         println!("====================");
//         println!("|--  MAIN MENU   --|");
//         println!("====================");
//         println!("1) Submit new order");
//         println!("2) View Order Book");
//         println!("3) View Transactions");
//         println!("\nPress any other key to exit");

//         let mut input: String = String::new();
//         io::stdin().read_line(&mut input).unwrap();

//         match input.trim() {
//             "1" => {
//                 handle_create_order(&mut order_book);
//             }
//             "2" => {
//                 order_book.print();
//             }
//             "3" => {
//                 order_book.print_transactions();
//             }
//             _ => {
//                 println!("Exiting now");
//                 break;
//             }
//         }
//     }

//     Ok(())
// }

// fn handle_create_order(order_book: &mut OrderBook) {
//     let mut order_builder: OrderBuilder = Order::new();
//     let mut input: String = String::new();

//     println!("Welcome to the create order function. We will walk you through the steps to submit your order.");
//     println!("If an invalid value is given, we will return to the main menu automatically.");

//     // Get order type
//     println!("STEP 1: Do you wish to BUY or SELL? ");
//     println!("Buy: Press 'B'");
//     println!("Sell: Press 'S'");
//     io::stdin().read_line(&mut input).unwrap();
//     match input.to_lowercase().trim() {
//         "b" => {
//             order_builder.order_type(OrderType::Buy);
//         }
//         "s" => {
//             order_builder.order_type(OrderType::Sell);
//         }
//         _ => return,
//     }

//     // Get price
//     println!("STEP 2: Enter price (any number more than 0):");
//     input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let price: f64 = match input.trim().parse() {
//         Ok(p) => p,
//         Err(_) => {
//             println!("Invalid price");
//             return;
//         }
//     };
//     // Return if price is negative
//     if price < 0.0 {
//         println!("Price cannot be negative");
//         return;
//     }
//     order_builder.price(price);

//     // Get quantity
//     println!("STEP 3: Enter quantity (any number more than 0):");
//     input = String::new();
//     io::stdin().read_line(&mut input).unwrap();
//     let quantity: u32 = match input.trim().parse() {
//         Ok(q) => q,
//         Err(_) => return,
//     };
//     order_builder.quantity(quantity);

//     let order: Order = order_builder.build();
//     match order_book.add_order(order) {
//         Ok(_) => println!("Order submitted successfully!"),
//         Err(_) => println!("Error submitting order, please try again!"),
//     }
// }
fn main() {
    println!("Above code no longer works due to refactoring, commented for future reference");
}