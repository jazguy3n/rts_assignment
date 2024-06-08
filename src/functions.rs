use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::{
    prelude::SliceRandom,
    Rng,
};
use crate::rabbitmq::{recv_msg, send_msg};
use crate::structs::{Inventory, Order};

// Common functions
pub fn send_queue(order: &Order, queue_name: &str) {
    let serialized_order = serde_json::to_string(&order).unwrap();

    match queue_name {
        "payment" => send_msg(serialized_order, "payment_queue").unwrap(),
        "inventory" => send_msg(serialized_order, "inventory_queue").unwrap(),
        "monitoring" => send_msg(serialized_order, "monitor_queue").unwrap(),
        "database" => send_msg(serialized_order, "database_queue").unwrap(),
        "delivery" => send_msg(serialized_order, "delivery_queue").unwrap(),
        "return_inventory" => send_msg(serialized_order, "return_inventory_queue").unwrap(),
        _ => {
            println!("Unknown queue name: {}", queue_name);
            return;
        },
    }
}

pub fn receive_orders(queue_name: &str, sender: Sender<Order>) {
    loop {
        let order = recv_msg(queue_name);
        if !order.is_empty() {
            if let Ok(deserialized_order) = serde_json::from_str::<Order>(&order) {
                if deserialized_order.id == -1 {
                    sender.send(deserialized_order).unwrap();
                    break;
                } else {
                    sender.send(deserialized_order).unwrap();
                }
            }
        }
    }
}

// Order system functions
pub fn generate_orders(order_tx: &Sender<Order>, limit: i32) {
    let mut rng = rand::thread_rng();
    let item_list = vec![
        "T-Shirt","Hoodie", "Skirt", "Dress", "Wallet", "Shoes",
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

    // Send a sentinel order to indicate the end of orders
    let end_order = Order {
        id: -1, // Use a specific id or other field to indicate the end
        item: String::new(),
        quantity: 0,
        shipping_address: String::new(),
        payment_status: false,
        delivery_status: false,
        final_status: String::new(),
    };

    order_tx.send(end_order).unwrap();
}


// Payment system functions
pub fn process_payment(order: &mut Order) {
    println!("[Order ID {}] Payment system received order", order.id);
    let mut rng = rand::thread_rng();
    // Simulate a payment process with a 50% success rate
    order.payment_status = rng.gen_bool(0.5);

    // Route the order based on the payment status
    if order.payment_status {
        println!("[Order ID {}] Payment successful", order.id);
        println!("[Order ID {}] Send to inventory system for processing...", order.id);
        send_queue(order, "inventory");
    } else {
        println!("[Order ID {}] Payment failed", order.id);
        println!("[Order ID {}] Send to monitoring system...", order.id);
        send_queue(order, "monitoring");
    }

    println!("--------------------------------------------------");
}

// Inventory system functions
pub fn handle_return(inventory: &Arc<Mutex<Inventory>>, order: &Order) {
    let mut inv = inventory.lock().unwrap();
    println!("[Return ID {}] Handling return Item: {}, Quantity: {}", order.id, order.item, order.quantity);
    inv.add_stock(&order.item, order.quantity);
    println!("[Return ID {}] Return item successfully. New stock {}: {}", order.id, order.item, inv.get_stock(&order.item));
}

pub fn inventory_checking(inventory: &Arc<Mutex<Inventory>>, order: &mut Order) {
    let mut inv = inventory.lock().unwrap();
    println!("------------------------------------------------------------------");
    println!("[Order ID {}] Inventory system received order, Item: {}, Quantity: {}", order.id, order.item, order.quantity);
    if inv.is_stock_available(&order.item, order.quantity) {
        inv.deduct_stock(&order.item, order.quantity);
        println!("[Order ID {}] Order processed successfully - Remaining stock: {}", order.id, inv.get_stock(&order.item));
        send_queue(order, "delivery");
    } else {
        println!("[Order ID {}] Insufficient stock, restock processing...", order.id);
        inv.restock(&order.item);
        if inv.is_stock_available(&order.item, order.quantity) {
            inv.deduct_stock(&order.item, order.quantity);
            println!("[Order ID {}] Order processed - Item: {}, Quantity: {}, Remaining stock: {}", order.id, order.item, order.quantity, inv.get_stock(&order.item));
            send_queue(order, "delivery");
        } else {
            println!("[Order ID {}] Order processing failed - Insufficient stock after restocking.", order.id);
        }
    }
}

// Delivery system functions
pub fn process_delivery(order: &mut Order) {
    println!("[Order ID {}] Delivery system received order", order.id);
    println!("[Order ID {}] Shipping to the address in the order...", order.id);
    let mut rng = rand::thread_rng();

    // Simulate a delivery process with a 50% success rate
    if rng.gen_bool(0.5) {
        order.delivery_status = true;
        println!("[Order ID {}] Deliver successfully!", order.id);
        order.final_status = "Delivered".to_string();
        //Send the order to the database system
        send_queue(&order, "database");
        println!("[Order ID {}] Recording to the database!", order.id);
    } else {
        order.delivery_status = false;
        println!("[Order ID {}] Failure delivery!", order.id);
        //Send the order to the monitoring system
        send_queue(&order, "monitoring");
        println!("[Order ID {}] Send to monitoring system!", order.id);
    }

    println!("---------------------------------------------");
}

// Monitoring system functions
pub fn repayment(order: &mut Order) {
    println!("[Order ID {}] Attempting to process payment again.......", order.id);
    let mut rng = rand::thread_rng();

    // Simulate repayment process with a 50% success rate
    if rng.gen_bool(0.5) {
        order.payment_status = true;
        println!("[Order ID {}] Payment is successful!", order.id);
        send_queue(order, "inventory");
    } else {
        order.payment_status = false;
        println!("[Order ID {}] Payment failed again!", order.id);
        println!("[Order ID {}] is being canceled due to repeated payment failure.", order.id);
        cancel_order(order);
    }
}

pub fn redelivery(order: &mut Order) {
    println!("[Order ID {}] Attempting to deliver the order again.......", order.id);
    let mut rng = rand::thread_rng();

    // Simulate redelivery process with a 50% success rate
    if rng.gen_bool(0.5) {
        order.delivery_status = true;
        order.final_status = "Delivered".to_string();
        println!("[Order ID {}] The order was delivered successfully!", order.id);
        send_queue(order, "database");
    } else {
        order.delivery_status = false;
        println!("[Order ID {}] The order has not been successfully delivered!", order.id);
        println!("[Order ID {}] is being canceled due to repeated delivery failure.", order.id);
        println!("[Order ID {}] Return items back to inventory......", order.id);
        send_queue(order, "return_inventory");
        cancel_order(order);
    }
}

pub fn cancel_order(order: &mut Order) {
    order.final_status = "Cancelled".to_string();
    send_queue(order, "database");
}
