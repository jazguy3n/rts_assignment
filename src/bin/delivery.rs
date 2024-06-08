use std::{
    thread,
    sync::mpsc::{self, Sender, Receiver},
};

use rts_assignment::{
    structs::Order,
    functions::{
        receive_orders,
        process_delivery,
        send_queue,
    }
};

fn main() {
    // Define the queue name for payment processing
    let queue_name = "delivery_queue";

    // Create channels for order processing
    let (order_tx, order_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();

    // Spawn a thread to receive orders
    thread::spawn(move || receive_orders(queue_name, order_tx));

    // Main thread loop for processing orders
    loop {
        // Process orders sequentially
        match order_rx.recv() {
            Ok(mut order) => {
                if order.id == -1 {
                    send_queue(&order, "monitoring");
                    println!("Shutting down the delivery system...");
                    break;
                }
                process_delivery(&mut order);
            }
            Err(e) => {
                println!("Error receiving order: {:?}", e);
                break;
            }
        }
    }
}

