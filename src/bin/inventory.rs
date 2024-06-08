use std::{
    sync::{mpsc::{self, Sender, Receiver}, Arc, Mutex},
    thread,
};

use rts_assignment::{
    structs::{Order, Inventory},
    functions::{
        receive_orders,
        handle_return,
        inventory_checking,
        send_queue,
    },
};


fn main() {
    // Define the queue names for order processing and return processing
    let queue_name = "inventory_queue";
    let queue_name_return = "return_inventory_queue"; // Ensure this matches the monitor system

    // Create channels for order and return processing
    let (order_tx, order_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();
    let (return_tx, return_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();

    // Initialize inventory
    let inventory = Arc::new(Mutex::new(Inventory::new()));

    // Spawn threads to receive orders and returns
    let inventory_clone = Arc::clone(&inventory);
    thread::spawn(move || receive_orders(queue_name, order_tx));

    let inventory_clone_return = Arc::clone(&inventory);
    thread::spawn(move || receive_orders(queue_name_return, return_tx));

    // Main thread loop for processing orders and returns
    loop {
        // Handle returns first
        match return_rx.try_recv() {
            Ok(return_order) => {
                println!("[Return ID {}] Received return order", return_order.id);
                handle_return(&inventory_clone_return, &return_order);
            }
            Err(_) => {
                // No returns to process, proceed to handle orders
                match order_rx.try_recv() {
                    Ok(mut order) => {
                        if order.id == -1 {
                            send_queue(&order, "delivery");
                            println!("Inventory system has been shut down.");
                            break;
                        }

                        // Check the inventory and process the order
                        inventory_checking(&inventory_clone, &mut order);
                    }
                    Err(_) => {
                        // No orders to process
                    }
                }
            }
        }

        // Add a small sleep to avoid busy waiting
        thread::sleep(std::time::Duration::from_millis(100));
    }
}
