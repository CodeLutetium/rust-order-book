pub struct Transaction {
    pub price: f64,
    pub quantity: u32,
}

impl Transaction {
    pub fn new(price: f64, quantity: u32) -> Self {
        Transaction {
            price,
            quantity,
        }
    }
}