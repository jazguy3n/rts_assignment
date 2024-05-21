use std::{
    thread,
    sync::mpsc::{self, Sender, Receiver},
};

use rts_assignment::{
    rabbitmq::recv_msg,
    structs::Order,
};

use serde_json;

fn main() {
    let (database_tx, database_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();

    database_order(database_tx, orders);

    loop {
        match database_rx.recv() {
            Ok(received_order) => {
                println!(
                    "Order ID: {}, Item: {}, Quantity: {}, Shipping Address: {}, Payment Status: {}, Delivery Status: {}, Final Status: {}",
                    order.id, order.item, order.quantity, order.shipping_address, order.payment_status, order.delivery_status, order.final_status
                );
            }
            Err(e) => {
                println!("Error receiving order: {}", e);
                break;
            }
        }
    }
}

fn database_order(sender: Sender<Order>, orders: Arc<Mutex<Vec<Order>>>) {
    loop {
        let order = recv_msg("cancel_queue");
        if !order.is_empty() {
            let deserialized_order: Order = serde_json::from_str(&order).unwrap();
            sender.send(deserialized_order).unwrap();
        }
    }
}

