pub enum OrderType {
    Buy,
    Sell
}

pub struct Order {
    pub order_type: OrderType,
    pub price: f64,
    pub quantity: u32,
}

impl Order {
    pub fn new() -> OrderBuilder {
        OrderBuilder::new()
    }
}

pub struct OrderBuilder {
    order_type: Option<OrderType>,
    price: Option<f64>,
    quantity: Option<u32>,
}

impl OrderBuilder {
    pub fn new() -> Self {
        OrderBuilder {
            order_type: None,
            price: None,
            quantity: None
        }
    }

    pub fn order_type(&mut self, order_type: OrderType) -> &Self {
        self.order_type = Some(order_type);
        self
    }

    pub fn price(&mut self, price: f64) -> &Self {
        self.price = Some(price);
        self
    }

    pub fn quantity(&mut self, quantity: u32) -> &Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn build(self) -> Order {
        let order_type: OrderType = self.order_type.expect("Order type cannot be empty");
        let price: f64 = self.price.expect("Price cannot be empty");
        let quantity:u32 = self.quantity.expect("Quantity cannot be empty");
        
        Order {
            order_type,
            price,
            quantity,
        }
    }
}