use criterion::{black_box, criterion_group, criterion_main, Criterion};
use Friday::prelude::*;
use ndarray::*;

fn bench_create_network(c: &mut Criterion) {
    c.bench_function("create_network", |b| b.iter(|| black_box(Network::new())));
}

fn bench_network_forward(c: &mut Criterion) {
    let mut network = Network::new();
    network.add_layer(DenseLayer::new(2, 3));
    network.add_layer(DenseLayer::new(3, 1));
    
    let input = array![1.0, 2.0];
    c.bench_function("network_forward", |b| b.iter(|| black_box(network.forward(input.clone()))));
}

criterion_group!(benches, bench_create_network, bench_network_forward);
criterion_main!(benches);