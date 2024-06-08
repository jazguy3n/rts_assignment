// use criterion::{criterion_group, criterion_main, Criterion};
// use rts_assignment::functions::generate_order;
// use rand::thread_rng;
// use std::sync::Arc;
// use std::sync::atomic::AtomicUsize;
//
// fn benchmark_generate_order(c: &mut Criterion) {
//     let mut rng = thread_rng();
//     let order_id_counter = Arc::new(AtomicUsize::new(0));
//
//     c.bench_function("generate_order", |b| {
//         b.iter(|| generate_order(&mut rng, &order_id_counter))
//     });
// }
//
// criterion_group!(benches, benchmark_generate_order);
// criterion_main!(benches);

