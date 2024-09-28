use crate::Transaction;

pub struct TransactionBook {
    transactions: Vec<Transaction>,
}

impl TransactionBook {
    pub fn new() -> TransactionBook {
        TransactionBook {
            transactions: Vec::new(),
        }
    }

    pub fn post(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn print(&self) {
        println!("+=============+=============+");
        println!("+     TRANSACTIONS BOOK     +");
        println!("+=============+=============+");
        println!("|  Quantity   |    Price    |");
        println!("|             |     ($)     |");
        println!("+=============+=============+");
        
        for transaction in self.transactions.iter() {
            let transaction_row: String = format!("|  {:<11}|  {:<11.2}|", transaction.quantity, transaction.price);
            println!("{}", transaction_row);
            println!("+=============+=============+");
        }
    }
}