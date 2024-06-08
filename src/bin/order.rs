use std::{
    sync::mpsc::channel,
    thread,
};

use rts_assignment::{
    functions::{
        generate_orders,
        send_queue,
    },
};

const ORDER_LIMIT: i32 = 10;

fn main() {
    let (order_tx, order_rx) = channel();

    // Order generation thread
    thread::spawn(move || {
        generate_orders(&order_tx, ORDER_LIMIT);
    });

    // Order processing in the main thread
    loop {
        if let Ok(order) = order_rx.recv() {
            if order.id == -1 {
                send_queue(&order, "payment");
                println!("All orders have been processed. Shutting the down order system...");
                break;
            }
            println!("Order ID: {}", order.id);
            println!("Item: {}", order.item);
            println!("Quantity: {}", order.quantity);
            println!("Shipping Address: {}", order.shipping_address);
            println!("Payment Status: {}", order.payment_status);
            println!("Delivery Status: {}", order.delivery_status);
            println!("Final Status: {}", order.final_status);
            send_queue(&order, "payment");
            println!("------------------------------------------------------------------");
        }
    }
}