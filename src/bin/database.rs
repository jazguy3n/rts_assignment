use std::{
    thread,
    sync::mpsc::{self, Sender, Receiver},
};

use rts_assignment::{
    structs::Order,
    functions::receive_orders,
};

fn main() {
    let queue_name = "database_queue";

    // Create channels for order processing
    let (order_tx, order_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();

    // Spawn a thread to receive orders
    thread::spawn(move || receive_orders(queue_name, order_tx));

    loop {
        match order_rx.recv() {
            Ok(order) => {
                if order.id == -1 {
                    println!("Shutting down the database system...");
                    break;
                }
                println!(
                    "Order ID: {}, Item: {}, Quantity: {}, Shipping Address: {}, Final Status: {}",
                    order.id, order.item, order.quantity, order.shipping_address, order.final_status
                );
                println!("---------------------------------------------------------------------------------------------")
            }
            Err(e) => {
                println!("Error receiving order: {}", e);
                break;
            }
        }
    }
}
