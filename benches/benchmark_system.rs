use criterion::{criterion_group, criterion_main, Criterion};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use rts_assignment::{
    structs::{Order, Inventory},
    functions::{
        process_payment,
        inventory_checking,
        process_delivery,
        repayment,
        redelivery,
        handle_return,
        send_queue,
    },
};

const MAX_ITERATIONS: usize = 100;  // Limit the number of iterations for benchmarking

fn create_mock_order(id: i32) -> Order {
    Order {
        id,
        item: "T-Shirt".to_string(),
        quantity: 1,
        shipping_address: "Johor".to_string(),
        payment_status: false,
        delivery_status: false,
        final_status: "Pending".to_string(),
    }
}

fn receive_orders_benchmark(queue_name: &str, sender: Sender<Order>, max_iterations: usize) {
    for i in 0..max_iterations {
        let order = create_mock_order(i as i32);
        sender.send(order).unwrap();
    }
    // Send a termination order
    sender.send(Order {
        id: -1,
        item: String::new(),
        quantity: 0,
        shipping_address: String::new(),
        payment_status: false,
        delivery_status: false,
        final_status: String::new(),
    }).unwrap();
}

fn benchmark_system(c: &mut Criterion) {
    let mut group = c.benchmark_group("System Benchmark");

    group.bench_function("whole_system", |b| {
        b.iter(|| {
            let (order_tx, order_rx): (Sender<Order>, Receiver<Order>) = channel();
            let (payment_tx, payment_rx): (Sender<Order>, Receiver<Order>) = channel();
            let (inventory_tx, inventory_rx): (Sender<Order>, Receiver<Order>) = channel();
            let (return_tx, return_rx): (Sender<Order>, Receiver<Order>) = channel();
            let (delivery_tx, delivery_rx): (Sender<Order>, Receiver<Order>) = channel();
            let (monitor_tx, monitor_rx): (Sender<Order>, Receiver<Order>) = channel();
            let (database_tx, database_rx): (Sender<Order>, Receiver<Order>) = channel();

            // Start the order generation thread
            let order_thread = thread::spawn(move || {
                for i in 0..MAX_ITERATIONS {
                    order_tx.send(create_mock_order(i as i32)).unwrap();
                }
                // Send termination signal
                order_tx.send(Order {
                    id: -1,
                    item: String::new(),
                    quantity: 0,
                    shipping_address: String::new(),
                    payment_status: false,
                    delivery_status: false,
                    final_status: String::new(),
                }).unwrap();
            });

            // Start the payment system thread
            let payment_thread = thread::spawn(move || {
                receive_orders_benchmark("payment_queue", payment_tx.clone(), MAX_ITERATIONS);
                while let Ok(mut order) = payment_rx.recv() {
                    if order.id == -1 {
                        send_queue(&order, "inventory");
                        println!("Shutting down the payment system...");
                        break;
                    }
                    process_payment(&mut order);
                }
            });

            // Start the inventory system thread
            let inventory = Arc::new(Mutex::new(Inventory::new()));
            let inventory_clone = Arc::clone(&inventory);
            let inventory_thread = thread::spawn(move || {
                receive_orders_benchmark("inventory_queue", inventory_tx.clone(), MAX_ITERATIONS);
                while let Ok(mut order) = inventory_rx.recv() {
                    if order.id == -1 {
                        send_queue(&order, "return");
                        println!("Shutting down the inventory system...");
                        break;
                    }
                    inventory_checking(&inventory_clone, &mut order);
                }
            });

            // Start the return inventory system thread
            let inventory_clone_return = Arc::clone(&inventory);
            let return_thread = thread::spawn(move || {
                receive_orders_benchmark("return_inventory_queue", return_tx.clone(), MAX_ITERATIONS);
                while let Ok(return_order) = return_rx.recv() {
                    if return_order.id == -1 {
                        send_queue(&return_order, "delivery");
                        println!("Shutting down the return inventory system...");
                        break;
                    }
                    handle_return(&inventory_clone_return, &return_order);
                }
            });

            // Start the delivery system thread
            let delivery_thread = thread::spawn(move || {
                receive_orders_benchmark("delivery_queue", delivery_tx.clone(), MAX_ITERATIONS);
                while let Ok(mut order) = delivery_rx.recv() {
                    if order.id == -1 {
                        send_queue(&order, "monitor");
                        println!("Shutting down the delivery system...");
                        break;
                    }
                    process_delivery(&mut order);
                }
            });

            // Start the monitor system thread
            let monitor_thread = thread::spawn(move || {
                receive_orders_benchmark("monitor_queue", monitor_tx.clone(), MAX_ITERATIONS);
                while let Ok(mut order) = monitor_rx.recv() {
                    if order.id == -1 {
                        send_queue(&order, "database");
                        println!("Shutting down the monitor system...");
                        break;
                    }
                    println!("[Order ID {}] Monitoring system received order", order.id);
                    if !order.payment_status {
                        repayment(&mut order);
                    } else if order.payment_status && !order.delivery_status {
                        redelivery(&mut order);
                    }
                    println!("------------------------------------------------------------")
                }
            });

            // Start the database system thread
            let database_thread = thread::spawn(move || {
                receive_orders_benchmark("database_queue", database_tx.clone(), MAX_ITERATIONS);
                while let Ok(order) = database_rx.recv() {
                    if order.id == -1 {
                        println!("Shutting down the database system...");
                        break;
                    }
                    println!(
                        "Order ID: {}, Item: {}, Quantity: {}, Shipping Address: {}, Final Status: {}",
                        order.id, order.item, order.quantity, order.shipping_address, order.final_status
                    );
                    println!("---------------------------------------------------------------------------------------------")
                }
            });

            // Wait for all threads to complete
            order_thread.join().unwrap();
            payment_thread.join().unwrap();
            inventory_thread.join().unwrap();
            return_thread.join().unwrap();
            delivery_thread.join().unwrap();
            monitor_thread.join().unwrap();
            database_thread.join().unwrap();
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_system);
criterion_main!(benches);
