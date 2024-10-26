use std::sync::{Arc, Mutex};

use actix_web::{rt, web, Error, HttpRequest, HttpResponse};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;

use crate::OrderBook;


// Send updates only
pub async fn order_book_websocket_handler(
    order_book: web::Data<Arc<Mutex<OrderBook>>>,
    req: HttpRequest,
    body: web::Payload,
) -> Result<HttpResponse, Error> {
    let (response, mut session, stream) = actix_ws::handle(&req, body)?;

    println!("Websocket connection established");

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            match msg {
                // Must be implemented so that the connection does not terminate
                AggregatedMessage::Ping(msg) => {
                    // respond to PING frame with PONG frame
                    // println!("Received message: {:?}", msg);
                    session.pong(&msg).await.unwrap();
                }
                _ => {
                    println!("Received non-text message");
                }
            }
        }
    });
    Ok(response)
}
