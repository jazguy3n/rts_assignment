use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use rand::{
    prelude::SliceRandom,
    Rng,
};
use crate::rabbitmq::{recv_msg, send_msg};
use crate::structs::Order;

// Common functions
pub fn receive_orders(queue_name: &str, sender: Sender<Order>) {
    loop {
        let order = recv_msg(queue_name);
        if !order.is_empty() {
            if let Ok(deserialized_order) = serde_json::from_str::<Order>(&order) {
                sender.send(deserialized_order).unwrap();
            }
        }
    }
}

pub fn receive_exit_signal(sender: Sender<()>) {
    loop {
        let exit_msg = recv_msg("exit_queue");
        if exit_msg == "exit" {
            sender.send(()).unwrap();
            break;
        }
    }
}

// Order system functions
pub fn generate_orders(order_tx: &std::sync::mpsc::Sender<Order>, limit: i32) {
    let mut rng = rand::thread_rng();
    let item_list = vec![
        "T-Shirt", "Hoodie", "Skirt", "Dress", "Wallet", "Shoes",
        "Socks", "Pants", "Shorts"
    ];
    let location_list = vec![
        "Johor", "Kedah", "Kelantan", "Kuala Lumpur", "Labuan", "Melaka",
        "Negeri Sembilan", "Pahang", "Penang", "Perak", "Perlis",
        "Putrajaya", "Sabah", "Sarawak", "Selangor", "Terengganu"
    ];

    for order_id in 1..=limit {
        let random_quantity: i32 = rng.gen_range(1..=10);
        let random_item = item_list.choose(&mut rng).unwrap().to_string();
        let random_location = location_list.choose(&mut rng).unwrap().to_string();
        let payment_status = false;
        let delivery_status = false;
        let final_status = "Pending".to_string();

        let order = Order {
            id: order_id,
            item: random_item,
            quantity: random_quantity,
            shipping_address: random_location,
            payment_status,
            delivery_status,
            final_status,
        };

        order_tx.send(order).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

// Payment system functions
pub fn process_payment(order: &mut Order) {
    println!("Payment system received order ID: {}", order.id);
    let mut rng = rand::thread_rng();
    // Simulate a payment process with a 70% success rate
    order.payment_status = rng.gen_bool(0.7);

    // Route the order based on the payment status
    if order.payment_status {
        println!("Payment successful for order ID: {}", order.id);
        send_to_inventory(order);
    } else {
        println!("Payment failed for order ID: {}", order.id);
        send_to_monitoring(order);
    }

    println!("--------------------------");
}

// Message sending functions
pub fn send_to_inventory(order: &Order) {
    let serialized_order = serde_json::to_string(&order).unwrap();
    send_msg(serialized_order, "inventory_queue").unwrap();
    println!("Order ID {} has been sent to the inventory system.", order.id);
}

pub fn send_to_monitoring(order: &Order) {
    let serialized_order = serde_json::to_string(&order).unwrap();
    send_msg(serialized_order, "monitor_queue").unwrap();
    println!("Order ID {} has been sent to the monitoring system.", order.id);
}

pub fn send_to_database(order: &Order) {
    let serialized_order = serde_json::to_string(&order).unwrap();
    send_msg(serialized_order, "database_queue").unwrap();
    println!("Order ID {} has been sent to the database system.", order.id);
}