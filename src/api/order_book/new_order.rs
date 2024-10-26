use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::{validate_jwt, Order, OrderBook, OrderType};

// Raw order received from user
#[derive(Debug, Deserialize)]
pub struct NewOrder {
    username: String,
    jwt: String,
    price: f64,
    quantity: u32,
    order_type: OrderType,
}

pub async fn new_order_request(order_book: web::Data<Arc<Mutex<OrderBook>>>, new_order_request: web::Form<NewOrder>) -> HttpResponse {
    println!("New order request: {:?}", new_order_request);

    // Validate JWT
    match validate_jwt(&new_order_request.jwt) {
        Ok(username) => {
            if username != new_order_request.username {
                println!("JWT username does not match request username");
                
                return HttpResponse::Unauthorized().body("Invalid user credentials");
            }
        }
        Err(_) => {
            println!("JWT username does not match request username");
            return HttpResponse::Unauthorized().body("Invalid user credentials");
        }
    }

    // Add order to order book
    let new_order: Order = Order {
        price: new_order_request.price,
        quantity: new_order_request.quantity,
        order_type: new_order_request.order_type.clone(),
    };

    let mut order_book = order_book.lock().unwrap();
    order_book.add_order(new_order).unwrap();
    println!("Order added to order book");
    
    HttpResponse::Ok().body("success")
}