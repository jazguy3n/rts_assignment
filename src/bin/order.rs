use std::{
    sync::mpsc::channel,
    thread,
};

use rts_assignment::{
    rabbitmq::send_msg,
    functions::generate_orders,
};

use serde_json;

const ORDER_LIMIT: i32 = 3;

fn main() {
    let (order_tx, order_rx) = channel();

    // Order generation thread
    thread::spawn(move || {
        generate_orders(&order_tx, ORDER_LIMIT);
    });

    // Order processing in the main thread
    for _ in 0..ORDER_LIMIT {
        if let Ok(order) = order_rx.recv() {
            let serialized_order = serde_json::to_string(&order).unwrap();
            send_msg(serialized_order, "payment_queue").unwrap();
            println!("Order ID: {}", order.id);
            println!("Item: {}", order.item);
            println!("Quantity: {}", order.quantity);
            println!("Shipping Address: {}", order.shipping_address);
            println!("Payment Status: {}", order.payment_status);
            println!("Delivery Status: {}", order.delivery_status);
            println!("Final Status: {}", order.final_status);
            println!("--------------------------");
        }
    }

    // Signal to indicate no more orders will be sent
    send_msg("exit".to_string(), "exit_queue").unwrap();
    println!("All orders processed. Shutdown system....");
}