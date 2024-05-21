use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::prelude::SliceRandom;
use rand::Rng;
use crate::structs::Order;

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