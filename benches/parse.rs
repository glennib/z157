use std::time::Duration;

use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use z157::Tree;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_large_input", |b| {
        b.iter(|| Tree::parse(black_box(include_str!("large-input.txt"))));
    });

    c.bench_function("parse_small_input", |b| {
        b.iter(|| Tree::parse(black_box(include_str!("small-input.txt"))));
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(100_000);
    targets = criterion_benchmark
);
criterion_main!(benches);
