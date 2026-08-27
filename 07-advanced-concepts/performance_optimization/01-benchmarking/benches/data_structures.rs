use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::collections::{HashMap, BTreeMap, HashSet, BTreeSet};

// HashMap vs BTreeMap
fn map_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("maps");

    for size in [100, 1000, 10000].iter() {
        // Insert
        group.bench_with_input(BenchmarkId::new("hashmap_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut map = HashMap::new();
                for i in 0..size {
                    map.insert(black_box(i), black_box(i * 2));
                }
                map
            })
        });

        group.bench_with_input(BenchmarkId::new("btreemap_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut map = BTreeMap::new();
                for i in 0..size {
                    map.insert(black_box(i), black_box(i * 2));
                }
                map
            })
        });

        // Lookup
        let hashmap: HashMap<i32, i32> = (0..*size).map(|i| (i, i * 2)).collect();
        let btreemap: BTreeMap<i32, i32> = (0..*size).map(|i| (i, i * 2)).collect();

        group.bench_with_input(BenchmarkId::new("hashmap_get", size), &hashmap, |b, map| {
            b.iter(|| {
                for i in 0..*size {
                    black_box(map.get(&i));
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("btreemap_get", size), &btreemap, |b, map| {
            b.iter(|| {
                for i in 0..*size {
                    black_box(map.get(&i));
                }
            })
        });
    }

    group.finish();
}

// HashSet vs BTreeSet
fn set_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("sets");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("hashset_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut set = HashSet::new();
                for i in 0..size {
                    set.insert(black_box(i));
                }
                set
            })
        });

        group.bench_with_input(BenchmarkId::new("btreeset_insert", size), size, |b, &size| {
            b.iter(|| {
                let mut set = BTreeSet::new();
                for i in 0..size {
                    set.insert(black_box(i));
                }
                set
            })
        });

        let hashset: HashSet<i32> = (0..*size).collect();
        let btreeset: BTreeSet<i32> = (0..*size).collect();

        group.bench_with_input(BenchmarkId::new("hashset_contains", size), &hashset, |b, set| {
            b.iter(|| {
                for i in 0..*size {
                    black_box(set.contains(&i));
                }
            })
        });

        group.bench_with_input(BenchmarkId::new("btreeset_contains", size), &btreeset, |b, set| {
            b.iter(|| {
                for i in 0..*size {
                    black_box(set.contains(&i));
                }
            })
        });
    }

    group.finish();
}

// Vec operations
fn vec_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec");

    for size in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("push_no_capacity", size), size, |b, &size| {
            b.iter(|| {
                let mut v = Vec::new();
                for i in 0..size {
                    v.push(black_box(i));
                }
                v
            })
        });

        group.bench_with_input(BenchmarkId::new("push_with_capacity", size), size, |b, &size| {
            b.iter(|| {
                let mut v = Vec::with_capacity(size);
                for i in 0..size {
                    v.push(black_box(i));
                }
                v
            })
        });

        group.bench_with_input(BenchmarkId::new("from_iter", size), size, |b, &size| {
            b.iter(|| {
                let v: Vec<i32> = (0..size).collect();
                v
            })
        });
    }

    group.finish();
}

criterion_group!(benches, map_bench, set_bench, vec_bench);
criterion_main!(benches);
