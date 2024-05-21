use std::{
    sync::{mpsc::channel, atomic::AtomicUsize, Arc},
    thread,
    time::Duration,
};

use rts_assignment::{
    rabbitmq::send_msg,
    functions::generate_order,
};

use serde_json;

fn main() {
    let (order_tx, order_rx) = channel();
    let order_id_counter = Arc::new(AtomicUsize::new(1));

    let order_id_counter_clone = Arc::clone(&order_id_counter);
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            let order = generate_order(&mut rng, &order_id_counter_clone);
            order_tx.send(order).unwrap();
            thread::sleep(Duration::from_secs(5));
        }
    });

    loop {
        let order = order_rx.recv().unwrap();
        let serialized_order = serde_json::to_string(&order).unwrap();
        send_msg(serialized_order, "order_queue").unwrap();
        println!("Order ID: {}", order.id);
        println!("Item: {}", order.item);
        println!("Quantity: {}", order.quantity);
        println!("Shipping Address: {}", order.shipping_address);
        println!("Payment Status: {}", order.payment_status);
        println!("Delivery Status: {}", order.delivery_status);
        println!("Final Status: {}", order.final_status);
        println!("--------------------------");
        thread::sleep(Duration::from_secs(5));
    }
}

