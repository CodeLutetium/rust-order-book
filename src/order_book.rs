use std::iter::zip;

use anyhow::Ok;

use crate::Order;
use crate::OrderType;

pub struct OrderBook {
    buy_orders: Vec<Order>,
    sell_orders: Vec<Order>,
}

// For print function. Alternatively, cli_table crate could be used to print this as well.
struct OrderBookRow {
    bid_qty: u32,
    bid_price: f64,
    offer_price: f64,
    offer_qty: u32,
}

impl OrderBookRow {
    fn new(buy_order: &Option<&Order>, sell_order: &Option<&Order>) -> Self {
        let mut order_row: OrderBookRow = OrderBookRow {
            bid_qty: 0,
            bid_price: 0.0,
            offer_price: 0.0,
            offer_qty: 0,
        };

        if let Some(order) = buy_order {
            order_row.bid_price = order.price;
            order_row.bid_qty = order.quantity;
        }

        if let Some(order) = sell_order {
            order_row.offer_price = order.price;
            order_row.offer_qty = order.quantity;
        }

        order_row
    }

    fn print_row(self) {
        let row: String = format!(
            "|  {:<11}|  {:<11.2}||{:>11.2}  |{:>11}  |",
            self.bid_qty, self.bid_price, self.offer_price, self.offer_qty
        );
        println!("{}", row);
        println!("+-------------+-------------++-------------+-------------+");
    }
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) -> Result<(), anyhow::Error>{
        match order.order_type {
            OrderType::Buy => {
                self.buy_orders.push(order);
            }
            OrderType::Sell => {
                self.sell_orders.push(order);
            }
            // _ => {
            //     return Err(anyhow!("Invalid order type"))
            // }
        }
        Ok(())
    }

    pub fn print(&self) {
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


        for row in order_book_rows {
            row.print_row();
        }
    }
}