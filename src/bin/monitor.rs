use std::{
    thread,
    sync::{mpsc::{self, Sender, Receiver}, Arc, Mutex},
};

use rts_assignment::{
    rabbitmq::{recv_msg, send_msg},
    structs::{Order, Inventory},
};

use serde_json;
use rand::Rng;

fn main() {
    let (monitor_tx, monitor_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();
    let inventory = Arc::new(Mutex::new(Inventory::new()));

    // Spawn the thread to monitor orders
    {
        let monitor_tx = monitor_tx.clone();
        let inventory = Arc::clone(&inventory);
        thread::spawn(move || {
            monitor_order(monitor_tx, inventory);
        });
    }

    loop {
        match monitor_rx.recv() {
            Ok(mut received_order) => {
                println!("Monitor system received order:");
                println!("Order ID: {}", received_order.id);
                println!("Item: {}", received_order.item);
                println!("Quantity: {}", received_order.quantity);
                println!("Shipping Address: {}", received_order.shipping_address);
                println!("Payment Status: {}", received_order.payment_status);

                if received_order.payment_status == false {
                    // Attempt to process the payment again
                    repayment(&mut received_order);
                } else if received_order.payment_status == true && received_order.delivery_status == false {
                    // Attempt to process the delivery again
                    redelivery(&mut received_order, &inventory);
                }

                println!("--------------------------");
            }
            Err(e) => {
                println!("Error receiving order: {}", e);
                break;
            }
        }
    }
}

fn monitor_order(sender: Sender<Order>, inventory: Arc<Mutex<Inventory>>) {
    thread::spawn(move || {
        loop {
            let order = recv_msg("monitor_queue");
            if !order.is_empty() {
                let deserialized_order: Order = serde_json::from_str(&order).unwrap();
                sender.send(deserialized_order).unwrap();
            }
        }
    });
}

fn repayment(order: &mut Order) {
    println!("Attempting to process payment again.......");
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.7) {
        order.payment_status = true;
        println!("Payment is successful!");
        let serialized_order = serde_json::to_string(order).unwrap();
        send_msg(serialized_order, "inventory_queue").unwrap();
        println!("Order ID {} has been sent to the inventory system.", order.id);
    } else {
        order.payment_status = false;
        println!("Payment failed again!");
        println!("Order ID {} is being canceled due to repeated payment failure.", order.id);
        cancel_order(order);
    }
}

fn redelivery(order: &mut Order, inventory: &Arc<Mutex<Inventory>>) {
    println!("Attempting to deliver the order again.......");
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.1) {
        order.delivery_status = true;
        println!("The order was delivered successfully!");
        let serialized_order = serde_json::to_string(order).unwrap();
        send_msg(serialized_order, "database_queue").unwrap();
        println!("Recording information to database......");
    } else {
        order.delivery_status = false;
        println!("The order has not been successfully delivered!");
        println!("Order ID {} is being canceled due to repeated delivery failure.", order.id);
        cancel_order(order);
        return_item(order, inventory);
    }
}

fn cancel_order(order: &mut Order) {
    order.final_status = "Cancelled".to_string();
    let serialized_order = serde_json::to_string(order).unwrap();
    send_msg(serialized_order, "database_queue").unwrap();
    println!("Recording information to database......");
}

fn return_item(order: &Order, inventory: &Arc<Mutex<Inventory>>) {
    let mut inv = inventory.lock().unwrap();
    println!("Before refund, stock for {}: {}", order.item, inv.get_stock(&order.item)); // Debug print
    inv.add_stock(&order.item, order.quantity);
    println!("Refunded stock for item {}: {}", order.item, order.quantity);
    println!("After refund, stock for {}: {}", order.item, inv.get_stock(&order.item)); // Debug print
}
