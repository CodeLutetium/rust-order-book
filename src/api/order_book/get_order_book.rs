use std::sync::{Arc, Mutex};
use actix_web::{rt, web, HttpRequest, Error, HttpResponse, Responder};
use serde_json::json;
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;

use crate::OrderBook;

pub async fn get_order_book(order_book: web::Data<Arc<Mutex<OrderBook>>>) -> impl Responder {
    let order_book = order_book.get_ref().lock().unwrap();
    HttpResponse::Ok().json(json!({
        "buy_orders": order_book.buy_orders,
        "sell_orders": order_book.sell_orders,
        "transactions": order_book.transaction_book.transactions,
    }))
}

pub async fn order_book_websocket_handler(
    order_book: web::Data<Arc<Mutex<OrderBook>>>,
    req: HttpRequest,
    body: web::Payload,
) -> Result<HttpResponse, Error> {
    let (response, mut session, stream) = actix_ws::handle(&req, body)?;

    println!("Websocket connection established");

    let mut stream = stream.aggregate_continuations().max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            match msg {
                AggregatedMessage::Text(text) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }
                _ => {}
            }
        }
    });
    Ok(response)    
}
