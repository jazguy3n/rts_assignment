use std::{
    thread,
    sync::mpsc::{self, Sender, Receiver},
};

use rts_assignment::{
    rabbitmq::recv_msg,
    structs::Order,
};

use serde_json;
use rand::Rng;
use rts_assignment::rabbitmq::send_msg;

fn main() {
    let (inventory_tx, inventory_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();

    inventory_order(inventory_tx);

    loop {
        match inventory_rx.recv() {
            Ok(received_order) => {
                println!("Inventory system received order:");
                println!("Order ID: {}", received_order.id);
                println!("Item: {}", received_order.item);
                println!("Quantity: {}", received_order.quantity);
                println!("Shipping Address: {}", received_order.shipping_address);
                println!("Payment Status: {}", received_order.payment_status);
                println!("--------------------------");
            }
            Err(e) => {
                println!("Error receiving order: {}", e);
                break;
            }
        }
    }
}

fn inventory_order(sender: Sender<Order>) {
    thread::spawn(move || {
        loop {
            let order = recv_msg("inventory_queue");
            if !order.is_empty() {
                let deserialized_order: Order = serde_json::from_str(&order).unwrap();
                sender.send(deserialized_order).unwrap();
            }
        }
    });
}