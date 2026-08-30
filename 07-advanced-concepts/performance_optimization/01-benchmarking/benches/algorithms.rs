use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

// Fibonacci implementations
fn fibonacci_recursive(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let next = a + b;
        a = b;
        b = next;
    }
    a
}

fn fibonacci_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    for n in [10u64, 15, 20].iter() {
        group.bench_with_input(BenchmarkId::new("iterative", n), n, |b, &n| {
            b.iter(|| fibonacci_iterative(black_box(n)))
        });

        if *n <= 20 {
            // Recursive is too slow for larger n
            group.bench_with_input(BenchmarkId::new("recursive", n), n, |b, &n| {
                b.iter(|| fibonacci_recursive(black_box(n)))
            });
        }
    }

    group.finish();
}

// Sorting comparisons
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn sorting_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");

    for size in [10, 100, 1000].iter() {
        let data: Vec<i32> = (0..*size).rev().collect();

        group.bench_with_input(BenchmarkId::new("bubble_sort", size), &data, |b, data| {
            b.iter(|| {
                let mut arr = data.clone();
                bubble_sort(&mut arr);
                arr
            })
        });

        group.bench_with_input(BenchmarkId::new("std_sort", size), &data, |b, data| {
            b.iter(|| {
                let mut arr = data.clone();
                arr.sort();
                arr
            })
        });

        group.bench_with_input(
            BenchmarkId::new("std_sort_unstable", size),
            &data,
            |b, data| {
                b.iter(|| {
                    let mut arr = data.clone();
                    arr.sort_unstable();
                    arr
                })
            },
        );
    }

    group.finish();
}

// String concatenation
fn concat_with_push_str(strings: &[&str]) -> String {
    let mut result = String::new();
    for s in strings {
        result.push_str(s);
    }
    result
}

fn concat_with_capacity(strings: &[&str]) -> String {
    let total_len: usize = strings.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(total_len);
    for s in strings {
        result.push_str(s);
    }
    result
}

fn concat_with_join(strings: &[&str]) -> String {
    strings.join("")
}

fn string_concat_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_concat");
    let strings: Vec<&str> = vec!["hello", " ", "world", " ", "from", " ", "rust"];

    group.bench_function("push_str", |b| {
        b.iter(|| concat_with_push_str(black_box(&strings)))
    });

    group.bench_function("with_capacity", |b| {
        b.iter(|| concat_with_capacity(black_box(&strings)))
    });

    group.bench_function("join", |b| b.iter(|| concat_with_join(black_box(&strings))));

    group.finish();
}

// Sum implementations
fn sum_loop(data: &[i32]) -> i32 {
    let mut sum = 0;
    for &x in data {
        sum += x;
    }
    sum
}

fn sum_iterator(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[allow(clippy::unnecessary_fold)] // This benchmark intentionally compares fold with Iterator::sum.
fn sum_fold(data: &[i32]) -> i32 {
    data.iter().fold(0, |acc, &x| acc + x)
}

fn sum_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum");
    let data: Vec<i32> = (0..10000).collect();

    group.bench_function("loop", |b| b.iter(|| sum_loop(black_box(&data))));

    group.bench_function("iterator", |b| b.iter(|| sum_iterator(black_box(&data))));

    group.bench_function("fold", |b| b.iter(|| sum_fold(black_box(&data))));

    group.finish();
}

criterion_group!(
    benches,
    fibonacci_bench,
    sorting_bench,
    string_concat_bench,
    sum_bench
);
criterion_main!(benches);
