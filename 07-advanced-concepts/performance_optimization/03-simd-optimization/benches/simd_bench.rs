use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use simd_optimization::*;

fn dot_product_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("dot_product");

    for size in [100, 1000, 10000].iter() {
        let a: Vec<f32> = (0..*size).map(|x| x as f32).collect();
        let b: Vec<f32> = (0..*size).map(|x| (x * 2) as f32).collect();

        group.bench_with_input(
            BenchmarkId::new("scalar", size),
            &(&a, &b),
            |bench, (a, b)| bench.iter(|| dot_product_scalar(black_box(a), black_box(b))),
        );

        group.bench_with_input(
            BenchmarkId::new("simd4", size),
            &(&a, &b),
            |bench, (a, b)| bench.iter(|| dot_product_simd4(black_box(a), black_box(b))),
        );

        group.bench_with_input(
            BenchmarkId::new("simd8", size),
            &(&a, &b),
            |bench, (a, b)| bench.iter(|| dot_product_simd8(black_box(a), black_box(b))),
        );
    }

    group.finish();
}

fn vec_add_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_add");

    for size in [100, 1000, 10000].iter() {
        let a: Vec<f32> = (0..*size).map(|x| x as f32).collect();
        let b: Vec<f32> = (0..*size).map(|x| (x * 2) as f32).collect();

        group.bench_with_input(
            BenchmarkId::new("scalar", size),
            &(&a, &b),
            |bench, (a, b)| bench.iter(|| vec_add_scalar(black_box(a), black_box(b))),
        );

        group.bench_with_input(
            BenchmarkId::new("simd", size),
            &(&a, &b),
            |bench, (a, b)| bench.iter(|| vec_add_simd(black_box(a), black_box(b))),
        );
    }

    group.finish();
}

fn sum_of_squares_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum_of_squares");

    for size in [100, 1000, 10000].iter() {
        let data: Vec<f32> = (0..*size).map(|x| x as f32).collect();

        group.bench_with_input(BenchmarkId::new("scalar", size), &data, |bench, data| {
            bench.iter(|| sum_of_squares_scalar(black_box(data)))
        });

        group.bench_with_input(BenchmarkId::new("simd", size), &data, |bench, data| {
            bench.iter(|| sum_of_squares_simd(black_box(data)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    dot_product_bench,
    vec_add_bench,
    sum_of_squares_bench
);
criterion_main!(benches);
