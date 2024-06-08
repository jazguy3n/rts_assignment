use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use rts_assignment::{
    structs::Order,
    functions::{
        process_payment,
        receive_orders,
        receive_exit_signal
    },
};

fn main() {
    // Define the queue name for payment processing
    let queue_name = "payment_queue";

    // Create channels for order processing and exit signals
    let (order_tx, order_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();
    let (exit_tx, exit_rx): (Sender<()>, Receiver<()>) = mpsc::channel();

    // Spawn a thread to receive orders
    let order_handle = thread::spawn(move || receive_orders(queue_name, order_tx));

    // Spawn a thread to receive the exit signal
    let exit_handle = thread::spawn(move || receive_exit_signal(exit_tx));

    // Main thread loop for processing orders
    loop {
        // Check for exit signal
        if exit_rx.try_recv().is_ok() {
            println!("Received exit signal. Shutting down payment system.");
            break;
        }

        // Process orders sequentially
        if let Ok(mut order) = order_rx.try_recv() {
            process_payment(&mut order);
        }
    }

    // Ensure both threads are joined before shutting down
    order_handle.join().unwrap();
    exit_handle.join().unwrap();

    println!("Payment system is shutting down...");
}


