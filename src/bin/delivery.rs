use std::{
    thread,
    sync::mpsc::{self, Sender, Receiver},
};

use rts_assignment::{
    rabbitmq::{send_msg, recv_msg},
    structs::Order,
};

use serde_json;
use rand::Rng;

fn main() {
    let (delivery_tx, delivery_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();

    // Spawn the order-receiving thread
    delivery_order(delivery_tx);

    loop {
        match delivery_rx.recv() {
            Ok(mut received_order) => {
                println!("Payment system received order:");
                println!("Order ID: {}", received_order.id);
                println!("Item: {}", received_order.item);
                println!("Quantity: {}", received_order.quantity);
                println!("Shipping Address: {}", received_order.shipping_address);

                //Processing payment of the order
                process_delivery(&mut received_order);
                println!("--------------------------");
            }
            Err(e) => {
                println!("Error receiving order: {}", e);
                break;
            }
        }
    }
}

fn delivery_order(sender: Sender<Order>) {
    thread::spawn(move || {
        loop {
            let order = recv_msg("delivery_queue");
            if !order.is_empty() {
                let deserialized_order: Order = serde_json::from_str(&order).unwrap();
                sender.send(deserialized_order).unwrap();
            }
        }
    });
}

fn process_delivery(order: &mut Order) {
    println!("Delivery to the address in the order...");
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.7) {
        order.delivery_status = true;
        println!("The order was delivered successfully!");
        order.final_status = "Delivered".to_string();
        //Send the order to the database system
        let serialized_order = serde_json::to_string(&order).unwrap();
        send_msg(serialized_order, "database_queue").unwrap();
        println!("Recording information to database......");
    } else {
        order.delivery_status = false;
        println!("The order has not been successfully delivered!");
        //Send the order to the monitoring system
        let serialized_order = serde_json::to_string(&order).unwrap();
        send_msg(serialized_order, "monitor_queue").unwrap();
        println!("Order ID {} has been sent to the monitoring system.", order.id);
    }
}

