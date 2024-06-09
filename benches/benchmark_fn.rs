use criterion::{criterion_group, criterion_main, Criterion, black_box};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use rts_assignment::{
    structs::{Order, Inventory},
    functions::{
        generate_orders,
        process_payment,
        inventory_checking,
        process_delivery,
        repayment,
        redelivery,
        handle_return,
        send_queue,
    },
};

const ORDER_LIMIT: i32 = 10;

// Mock function to simulate receiving messages from the queue
fn recv_msg_mock(_queue_name: &str) -> String {
    let order = Order {
        id: 1,
        item: "T-Shirt".to_string(),
        quantity: 1,
        shipping_address: "Johor".to_string(),
        payment_status: false,
        delivery_status: false,
        final_status: "Pending".to_string(),
    };
    serde_json::to_string(&order).unwrap()
}

// Updated receive_orders function using the mock with limited iterations
fn receive_orders_mock(queue_name: &str, sender: Sender<Order>, max_iterations: usize) {
    for _ in 0..max_iterations {
        let order = recv_msg_mock(queue_name);
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

fn benchmark_generate_orders(c: &mut Criterion) {
    let mut group = c.benchmark_group("Generate Orders");
    group.sample_size(100);  // Attempt to get closer to 100 iterations
    group.bench_function("generate_orders", |b| {
        b.iter(|| {
            let (order_tx, _order_rx): (Sender<Order>, _) = channel();
            generate_orders(black_box(&order_tx), black_box(ORDER_LIMIT));
        })
    });
    group.finish();
}

fn benchmark_process_payment(c: &mut Criterion) {
    let mut group = c.benchmark_group("Process Payment");
    group.sample_size(100);  // Attempt to get closer to 100 iterations
    group.bench_function("process_payment", |b| {
        b.iter(|| {
            let mut order = Order {
                id: 1,
                item: "T-Shirt".to_string(),
                quantity: 1,
                shipping_address: "Johor".to_string(),
                payment_status: false,
                delivery_status: false,
                final_status: "Pending".to_string(),
            };
            process_payment(black_box(&mut order));
        })
    });
    group.finish();
}

fn benchmark_inventory_checking(c: &mut Criterion) {
    let mut group = c.benchmark_group("Inventory Checking");
    group.sample_size(100);  // Attempt to get closer to 100 iterations
    group.bench_function("inventory_checking", |b| {
        b.iter(|| {
            let inventory = Arc::new(Mutex::new(Inventory::new()));
            let mut order = Order {
                id: 1,
                item: "T-Shirt".to_string(),
                quantity: 1,
                shipping_address: "Johor".to_string(),
                payment_status: false,
                delivery_status: false,
                final_status: "Pending".to_string(),
            };
            inventory_checking(black_box(&inventory), black_box(&mut order));
        })
    });
    group.finish();
}

fn benchmark_process_delivery(c: &mut Criterion) {
    let mut group = c.benchmark_group("Process Delivery");
    group.sample_size(100);  // Attempt to get closer to 100 iterations
    group.bench_function("process_delivery", |b| {
        b.iter(|| {
            let mut order = Order {
                id: 1,
                item: "T-Shirt".to_string(),
                quantity: 1,
                shipping_address: "Johor".to_string(),
                payment_status: true,
                delivery_status: false,
                final_status: "Pending".to_string(),
            };
            process_delivery(black_box(&mut order));
        })
    });
    group.finish();
}

fn benchmark_repayment(c: &mut Criterion) {
    let mut group = c.benchmark_group("Repayment");
    group.sample_size(100);  // Attempt to get closer to 100 iterations
    group.bench_function("repayment", |b| {
        b.iter(|| {
            let mut order = Order {
                id: 1,
                item: "T-Shirt".to_string(),
                quantity: 1,
                shipping_address: "Johor".to_string(),
                payment_status: false,
                delivery_status: false,
                final_status: "Pending".to_string(),
            };
            repayment(black_box(&mut order));
        })
    });
    group.finish();
}

fn benchmark_redelivery(c: &mut Criterion) {
    let mut group = c.benchmark_group("Redelivery");
    group.sample_size(100);  // Attempt to get closer to 100 iterations
    group.bench_function("redelivery", |b| {
        b.iter(|| {
            let mut order = Order {
                id: 1,
                item: "T-Shirt".to_string(),
                quantity: 1,
                shipping_address: "Johor".to_string(),
                payment_status: true,
                delivery_status: false,
                final_status: "Pending".to_string(),
            };
            redelivery(black_box(&mut order));
        })
    });
    group.finish();
}

fn benchmark_handle_return(c: &mut Criterion) {
    let mut group = c.benchmark_group("Handle Return Order");
    group.sample_size(100);  // Attempt to get closer to 100 iterations
    group.bench_function("handle_return", |b| {
        b.iter(|| {
            let inventory = Arc::new(Mutex::new(Inventory::new()));
            let return_order = Order {
                id: 1,
                item: "T-Shirt".to_string(),
                quantity: 1,
                shipping_address: "Johor".to_string(),
                payment_status: false,
                delivery_status: false,
                final_status: "Pending".to_string(),
            };
            handle_return(black_box(&inventory), black_box(&return_order));
        })
    });
    group.finish();
}

fn benchmark_send_queue(c: &mut Criterion) {
    let mut group = c.benchmark_group("Send Queue");
    group.sample_size(100);  // Attempt to get closer to 100 iterations
    group.bench_function("send_queue", |b| {
        b.iter(|| {
            let order = Order {
                id: 1,
                item: "T-Shirt".to_string(),
                quantity: 1,
                shipping_address: "Johor".to_string(),
                payment_status: false,
                delivery_status: false,
                final_status: "Pending".to_string(),
            };
            send_queue(black_box(&order), black_box("payment"));
        })
    });
    group.finish();
}

fn benchmark_receive_orders(c: &mut Criterion) {
    let mut group = c.benchmark_group("Receive Orders");
    group.sample_size(100);  // Attempt to get closer to 100 iterations

    group.bench_function("receive_orders", |b| {
        b.iter(|| {
            let (order_tx, order_rx): (Sender<Order>, Receiver<Order>) = channel();

            // Simulate receiving orders from the queue with limited iterations
            receive_orders_mock(black_box("payment_queue"), black_box(order_tx.clone()), 10);

            // Process messages from the receiver end
            while let Ok(_order) = order_rx.try_recv() {
                // Simulate processing the received order
            }
        })
    });
    group.finish();
}

criterion_group!(
    function_benches,
    benchmark_generate_orders,
    benchmark_process_payment,
    benchmark_inventory_checking,
    benchmark_process_delivery,
    benchmark_repayment,
    benchmark_redelivery,
    benchmark_handle_return,
    benchmark_send_queue,
    benchmark_receive_orders
);
criterion_main!(function_benches);
