use serde::{Serialize, Deserialize};

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