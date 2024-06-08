use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use rts_assignment::{
    structs::Order,
    functions::{
        process_payment,
        receive_orders,
        send_queue,
    },
};

fn main() {
    // Use match to determine the queue name based on the condition
    let queue_name = "payment_queue";

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
                    send_queue(&order, "inventory");
                    println!("Shutting down the payment system...");
                    break;
                }

                process_payment(&mut order);
            }
            Err(e) => {
                println!("Error receiving order: {:?}", e);
                break;
            }
        }
    }
}
