use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: i32,
    pub item: String,
    pub quantity: i32,
    pub shipping_address: String,
    pub payment_status: bool,
    pub delivery_status: bool,
    pub final_status: String,
}

#[derive(Debug, Clone)]
pub struct ItemStock {
    pub name: &'static str,
    pub quantity: i32,
}

pub struct Inventory {
    pub stocks: Vec<ItemStock>,
}

pub const MAX_CAPACITY: i32 = 20;

impl Inventory {
    pub fn new() -> Self {
        let items = vec![
            ItemStock { name: "T-Shirt", quantity: MAX_CAPACITY },
            ItemStock { name: "Hoodie", quantity: MAX_CAPACITY },
            ItemStock { name: "Skirt", quantity: MAX_CAPACITY },
            ItemStock { name: "Dress", quantity: MAX_CAPACITY },
            ItemStock { name: "Wallet", quantity: MAX_CAPACITY },
            ItemStock { name: "Shoes", quantity: MAX_CAPACITY },
            ItemStock { name: "Socks", quantity: MAX_CAPACITY },
            ItemStock { name: "Pants", quantity: MAX_CAPACITY },
            ItemStock { name: "Shorts", quantity: MAX_CAPACITY },
        ];
        Inventory { stocks: items }
    }

    pub fn is_stock_available(&self, item: &str, quantity: i32) -> bool {
        self.stocks.iter().any(|stock| stock.name == item && stock.quantity >= quantity)
    }

    pub fn deduct_stock(&mut self, item: &str, quantity: i32) {
        if let Some(stock) = self.stocks.iter_mut().find(|stock| stock.name == item) {
            stock.quantity -= quantity;
        }
    }

    pub fn get_stock(&self, item: &str) -> i32 {
        self.stocks.iter().find(|stock| stock.name == item).map_or(0, |stock| stock.quantity)
    }

    pub fn restock(&mut self) {
        for stock in &mut self.stocks {
            stock.quantity = MAX_CAPACITY;
        }
        println!("Inventory restocked to maximum capacity.");
    }
}
