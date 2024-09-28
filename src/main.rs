use order_book::{Order, OrderBook, OrderType, TransactionBook};

fn main() {
    println!("Hello, world! This is the order book project");

    let mut order_book: OrderBook = OrderBook::new();
    let mut transaction_book: TransactionBook = TransactionBook::new();

    let order_1: Order = Order::new()
        .order_type(OrderType::Buy)
        .price(2.12)
        .quantity(200)
        .build();
    let order_2: Order = Order::new()
        .order_type(OrderType::Sell)
        .price(2.17)
        .quantity(100)
        .build();
    let order_3: Order = Order::new()
        .order_type(OrderType::Sell)
        .price(2.52)
        .quantity(100)
        .build();
    let order_4: Order = Order::new()
        .order_type(OrderType::Buy)
        .price(2.42)
        .quantity(200)
        .build();

    let _ = order_book.add_order(order_1);
    let _ = order_book.add_order(order_2);
    let _ = order_book.add_order(order_3);
    let _ = order_book.add_order(order_4);

    order_book.print();
}