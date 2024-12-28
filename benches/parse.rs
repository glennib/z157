use criterion::Criterion;
use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use z157::Tree;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse_large_input", |b| {
        b.iter(|| Tree::parse(black_box(include_str!("inputs/large-input.txt"))));
    });

    c.bench_function("parse_small_input", |b| {
        b.iter(|| Tree::parse(black_box(include_str!("inputs/small-input.txt"))));
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .significance_level(0.03)
        .noise_threshold(0.03);
    targets = criterion_benchmark
);
criterion_main!(benches);
