use std::{
    sync::{mpsc::{self, Sender, Receiver}, Arc, Mutex},
    thread,
    time::Duration,
};

use rts_assignment::{
    rabbitmq::{send_msg, recv_msg},
    structs::{Order, Inventory},
};

use serde_json;

fn main() {
    let (inventory_tx, inventory_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();
    let inventory = Arc::new(Mutex::new(Inventory::new()));

    // Spawn the thread to receive and process orders
    {
        let inventory_tx = inventory_tx.clone();
        let inventory = Arc::clone(&inventory);
        thread::Builder::new().name("OrderReceiverThread".to_string()).spawn(move || {
            inventory_order(inventory_tx, inventory);
        }).expect("Failed to spawn OrderReceiverThread");
    }

    loop {
        match inventory_rx.recv() {
            Ok(mut received_order) => {
                println!("Inventory system received order:");
                println!("Order ID: {}", received_order.id);
                println!("Item: {}", received_order.item);
                println!("Quantity: {}", received_order.quantity);
                println!("Shipping Address: {}", received_order.shipping_address);
                println!("Payment Status: {}", received_order.payment_status);

                // Check inventory and process the order
                process_order(&mut received_order, &inventory);
                println!("--------------------------");
            }
            Err(e) => {
                println!("Error receiving order: {}", e);
                break;
            }
        }
    }
}

fn inventory_order(sender: Sender<Order>, _inventory: Arc<Mutex<Inventory>>) {
    loop {
        let order = recv_msg("inventory_queue");
        if !order.is_empty() {
            let deserialized_order: Order = serde_json::from_str(&order).unwrap();
            sender.send(deserialized_order).unwrap();
        }
    }
}

fn process_order(order: &mut Order, inventory: &Arc<Mutex<Inventory>>) {
    loop {
        let mut inv = inventory.lock().unwrap();
        if inv.is_stock_available(&order.item, order.quantity) {
            inv.deduct_stock(&order.item, order.quantity);
            println!(
                "Order ID {} is confirmed. Stock for {}: {}",
                order.id, order.item, inv.get_stock(&order.item)
            );
            // Send the order to the delivery system
            let serialized_order = serde_json::to_string(order).unwrap();
            if send_msg(serialized_order, "delivery_queue").is_err() {
                eprintln!("Failed to send order ID {} to the delivery system.", order.id);
            } else {
                println!("Order ID {} has been sent to the delivery system.", order.id);
            }
            break;
        } else {
            println!("Insufficient stock for {}. Waiting for restock...", order.item);
            drop(inv); // Release the lock before restocking
            thread::sleep(Duration::from_secs(1));
            let mut inv = inventory.lock().unwrap();
            inv.restock(); // Restock the inventory
        }
    }
}
