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
    let (payment_tx, payment_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();

    // Spawn the order-receiving thread
    payment_order(payment_tx);

    loop {
        match payment_rx.recv() {
            Ok(mut received_order) => {
                println!("Payment system received order:");
                println!("Order ID: {}", received_order.id);
                println!("Item: {}", received_order.item);
                println!("Quantity: {}", received_order.quantity);
                println!("Shipping Address: {}", received_order.shipping_address);

                //Processing payment of the order
                process_payment(&mut received_order);
                println!("--------------------------");
            }
            Err(e) => {
                println!("Error receiving order: {}", e);
                break;
            }
        }
    }
}

fn payment_order(sender: Sender<Order>) {
    thread::spawn(move || {
        loop {
            let order = recv_msg("order_queue");
            if !order.is_empty() {
                let deserialized_order: Order = serde_json::from_str(&order).unwrap();
                sender.send(deserialized_order).unwrap();
            }
        }
    });
}

fn process_payment(order: &mut Order) {
    println!("Payment is processing......");
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.7) {
        order.payment_status = true;
        println!("Payment is successful!");
        //Send the order to the inventory system
        let serialized_order = serde_json::to_string(&order).unwrap();
        send_msg(serialized_order, "inventory_queue").unwrap();
        println!("Order ID {} has been sent to the inventory system.", order.id);
    } else {
        order.payment_status = false;
        println!("Payment has failed!");
        //Send the order to the monitoring system
        let serialized_order = serde_json::to_string(&order).unwrap();
        send_msg(serialized_order, "monitor_queue").unwrap();
        println!("Order ID {} has been sent to the monitoring system.", order.id);
    }
}

