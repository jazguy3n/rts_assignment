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

    // Spawn the thread to receive and store orders
    thread::spawn(move || {
        database_order(database_tx);
    });

    loop {
        match database_rx.recv() {
            Ok(received_order) => {
                println!(
                    "Order ID: {}, Item: {}, Quantity: {}, Shipping Address: {}, Final Status: {}",
                    received_order.id, received_order.item, received_order.quantity, received_order.shipping_address, received_order.final_status
                );
            }
            Err(e) => {
                println!("Error receiving order: {}", e);
                break;
            }
        }
    }
}

fn database_order(sender: Sender<Order>) {
    loop {
        let order = recv_msg("database_queue");
        if !order.is_empty() {
            let deserialized_order: Order = serde_json::from_str(&order).unwrap();
            sender.send(deserialized_order).unwrap();
        }
    }
}
