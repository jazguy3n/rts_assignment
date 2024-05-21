use std::{
    thread,
    sync::mpsc::{self, Sender, Receiver},
};

use rts_assignment::{
    rabbitmq::{recv_msg, send_msg},
    structs::Order,
};

use serde_json;
use rand::Rng;

fn main() {
    let (monitor_tx, monitor_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();

    monitor_order(monitor_tx);

    loop {
        match monitor_rx.recv() {
            Ok(mut received_order) => {
                println!("Monitor system received order:");
                println!("Order ID: {}", received_order.id);
                println!("Item: {}", received_order.item);
                println!("Quantity: {}", received_order.quantity);
                println!("Shipping Address: {}", received_order.shipping_address);
                println!("Payment Status: {}", received_order.payment_status);

                // Attempt to process the payment again
                attempt_repayment(&mut received_order);
                println!("--------------------------");
            }
            Err(e) => {
                println!("Error receiving order: {}", e);
                break;
            }
        }
    }
}

fn monitor_order(sender: Sender<Order>) {
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

fn attempt_repayment(order: &mut Order) {
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
        cancel_order(order);
    }
}

fn cancel_order(order: &mut Order) {
    order.final_status = "Cancelled".to_string();
    println!("Order ID {} is being canceled due to repeated payment failure.", order.id);
    let serialized_order = serde_json::to_string(order).unwrap();
    send_msg(serialized_order, "cancel_queue").unwrap();
    println!("Recording information to database......");
}
