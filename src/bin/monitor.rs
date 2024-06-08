use std::{
    thread,
    sync::mpsc::{self, Sender, Receiver},
};

use rts_assignment::{
    structs::Order,
    functions::{
        receive_orders,
        repayment,
        redelivery,
        send_queue,
    },
};

fn main() {
    // Define the queue name for payment processing
    let queue_name = "monitor_queue";

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
                    send_queue(&order, "database");
                    println!("Shutting down the monitor system...");
                    break;
                }
                println!("[Order ID {}] Monitoring system received order", order.id);

                if !order.payment_status {
                    // Attempt to process the payment again
                    repayment(&mut order);
                } else if order.payment_status && !order.delivery_status {
                    // Attempt to process the delivery again
                    redelivery(&mut order);
                }
                println!("------------------------------------------------------------")
            }
            Err(e) => {
                println!("Error receiving order: {:?}", e);
                break;
            }
        }
    }
}
