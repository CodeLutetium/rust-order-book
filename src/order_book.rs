use std::iter::zip;

use anyhow::Ok;

use crate::Transaction;
use crate::Order;
use crate::OrderType;
use crate::TransactionBook;

pub struct OrderBook {
    buy_orders: Vec<Order>,
    sell_orders: Vec<Order>,
    transaction_book: TransactionBook,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
            transaction_book: TransactionBook::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) -> Result<(), anyhow::Error>{
        // Add to array based on order type
        match order.order_type {
            OrderType::Buy => {
                self.buy_orders.push(order);
            }
            OrderType::Sell => {
                self.sell_orders.push(order);
            }
        }

        // Sort the orders
        self.sort_orders();

        // Check for execution
        if !self.buy_orders.is_empty() && !self.sell_orders.is_empty() && self.buy_orders[0].price >= self.sell_orders[0].price {
            self.execute_orders();
        }
        Ok(())
    }

    fn execute_orders(&mut self) {
        // Case 1: Buy order qty is same as sell order qty
        if self.buy_orders[0].quantity == self.sell_orders[0].quantity {
            let buy_order: Order = self.buy_orders.remove(0);
            let _sell_order: Order = self.sell_orders.remove(0);

            let transaction: Transaction = Transaction::new(buy_order.price, buy_order.quantity);
            self.transaction_book.post(transaction);
        }
    }

    fn sort_orders(&mut self) {
        self.buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
        self.sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    }

    pub fn print(&self) {
        println!("+=============+=============++=============+=============+");
        println!("+=============+         ORDER BOOK         +=============+");
        println!("+=============+=============++=============+=============+");
        println!("|  Quantity   |  Bid price  || Offer price |  Quantity   |");
        println!("|             |     ($)     ||     ($)     |             |");
        println!("+=============+=============++=============+=============+");

        let mut order_book_rows: Vec<OrderBookRow> = Vec::new();

        // Load rows
        let combined_orders = zip(self.buy_orders.iter(), self.sell_orders.iter());
        for (buy_order, sell_order) in combined_orders {
            order_book_rows.push(OrderBookRow::new(&Some(buy_order), &Some(sell_order)));
        }

        // Handle remaining buy orders
        for buy_order in self.buy_orders.iter().skip(order_book_rows.len()) {
            order_book_rows.push(OrderBookRow::new(&Some(buy_order), &None));
        }

        // Handle remaining sell orders
        for sell_order in self.sell_orders.iter().skip(order_book_rows.len()) {
            order_book_rows.push(OrderBookRow::new(&None, &Some(sell_order)));
        }


        // Print rows
        for row in order_book_rows {
            row.print_row();
        }
    }
}


// For print function. Alternatively, cli_table crate could be used to print this as well.
struct OrderBookRow {
    bid_qty: Option<u32>,
    bid_price: Option<f64>,
    offer_price: Option<f64>,
    offer_qty: Option<u32>,
}

impl OrderBookRow {
    fn new(buy_order: &Option<&Order>, sell_order: &Option<&Order>) -> Self {
        let mut order_row: OrderBookRow = OrderBookRow {
            bid_qty: None,
            bid_price: None,
            offer_price: None,
            offer_qty: None,
        };

        if let Some(order) = buy_order {
            order_row.bid_price = Some(order.price);
            order_row.bid_qty = Some(order.quantity);
        }

        if let Some(order) = sell_order {
            order_row.offer_price = Some(order.price);
            order_row.offer_qty = Some(order.quantity);
        }

        order_row
    }

    fn print_row(self) {
        let row: String = format!(
            "|  {:<11}|  {:<11}||{:>11}  |{:>11}  |",
            self.bid_qty.map_or(" ".to_string(), |qty| qty.to_string()), 
            self.bid_price.map_or(" ".to_string(), |price| format!("{:.2}", price)), 
            self.offer_price.map_or(" ".to_string(), |price| format!("{:.2}", price)), 
            self.offer_qty.map_or(" ".to_string(), |qty| qty.to_string()) 
        );
        println!("{}", row);
        println!("+-------------+-------------++-------------+-------------+");
    }
}
