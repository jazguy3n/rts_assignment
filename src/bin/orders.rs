use std::{
    sync::{mpsc::channel, atomic::{AtomicUsize, Ordering}, Arc},
    thread,
    time::Duration,
};

use rts_assignment::{
    structs::Order,
    rabbitmq::send_msg
};

use rand::{
    seq::SliceRandom,
    Rng,
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
            thread::sleep(Duration::from_millis(5000));
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
        println!("--------------------------");
        thread::sleep(Duration::from_secs(7));
    }
}

pub fn generate_order(rng: &mut rand::rngs::ThreadRng, order_id_counter: &Arc<AtomicUsize>) -> Order {
    let item_list = vec![
        "T-Shirt", "Hoodie", "Skirt",
        "Dress", "Wallet", "Shoes",
        "Socks", "Pants", "Shorts"];

    let location_list = vec![
        "Johor", "Kedah", "Kelantan",
        "Kuala Lumpur", "Labuan", "Melaka",
        "Negeri Sembilan", "Pahang", "Penang",
        "Perak", "Perlis", "Putrajaya",
        "Sabah", "Sarawak", "Selangor", "Terengganu"];

    let random_quantity: i32 = rng.gen_range(1..=10);
    let random_item = item_list.choose(rng).unwrap().to_string();
    let random_location = location_list.choose(rng).unwrap().to_string();
    let payment_status = false;
    let delivery_status = false;
    let final_status = "Pending".to_string();

    let order_id = order_id_counter.fetch_add(1, Ordering::SeqCst);

    Order {
        id: order_id as i32,
        item: random_item,
        quantity: random_quantity,
        shipping_address: random_location,
        payment_status,
        delivery_status,
        final_status,
    }
}
