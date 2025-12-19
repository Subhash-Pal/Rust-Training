use criterion::{criterion_group, criterion_main, Criterion};

fn sort_vec_benchmark(c: &mut Criterion) {
    c.bench_function("vec sort 1M", |b| {
        b.iter(|| {
            let mut data: Vec<i32> = (0..1_000_000).rev().collect();
            data.sort();
        })
    });
}

criterion_group!(benches, sort_vec_benchmark);
criterion_main!(benches);
