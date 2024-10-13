use std::sync::{Arc, Mutex};

use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::OrderBook;

pub async fn get_order_book(order_book: web::Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    let order_book = order_book.get_ref().lock().unwrap();
    HttpResponse::Ok().json(json!({
        "buy_orders": order_book.buy_orders,
        "sell_orders": order_book.sell_orders,
        "transactions": order_book.transaction_book.transactions,
    }))
}