# Section 18: Performance & Optimization

## Overview

Master performance optimization in Rust. Learn profiling, benchmarking, SIMD, memory optimization, and compile-time techniques to squeeze every last bit of performance from your code.

## Why Rust for Performance?

✅ **Zero-Cost Abstractions** - High-level code compiles to efficient machine code <br />
✅ **Memory Control** - Fine-grained control without GC pauses <br />
✅ **LLVM Backend** - World-class optimizations <br />
✅ **Predictable Performance** - No hidden allocations or runtime surprises <br />
✅ **SIMD Support** - Native vectorization capabilities

## What You'll Learn

1. **Profiling** - flamegraph, perf, valgrind
2. **Benchmarking** - criterion, statistical analysis
3. **CPU Optimization** - Branch prediction, cache locality
4. **Memory Optimization** - Allocation patterns, arena allocators
5. **SIMD** - Vectorization for parallel data processing
6. **Compile-time Optimization** - const fn, monomorphization
7. **Algorithmic Improvements** - Better algorithms beat micro-optimizations
8. **Zero-Copy** - Avoiding unnecessary allocations

## Section Contents

### 01-benchmarking/
Comprehensive benchmarking with criterion

### 02-profiling/
CPU and memory profiling tools

### 03-simd-optimization/
SIMD vectorization examples

## Prerequisites

- Strong Rust fundamentals
- Basic understanding of computer architecture
- Knowledge of algorithmic complexity
- Familiarity with assembly (helpful but not required)

## Key Concepts

### Benchmarking with Criterion

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci recursive 20", |b| {
        b.iter(|| fibonacci_recursive(black_box(20)))
    });

    c.bench_function("fibonacci iterative 20", |b| {
        b.iter(|| fibonacci_iterative(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### CPU Profiling with flamegraph

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph (requires perf on Linux)
cargo flamegraph --bench my_benchmark

# Open flamegraph.svg in browser
```

### Memory Profiling

```bash
# Install valgrind
sudo apt install valgrind

# Run with massif
valgrind --tool=massif target/release/my_program

# Visualize with massif-visualizer
ms_print massif.out.*
```

### Allocation Optimization

```rust
// Bad: Multiple allocations
fn process_strings_bad(input: &str) -> String {
    let mut result = String::new();
    for word in input.split_whitespace() {
        result.push_str(word);  // Multiple reallocations
        result.push(' ');
    }
    result
}

// Good: Pre-allocate
fn process_strings_good(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for word in input.split_whitespace() {
        result.push_str(word);
        result.push(' ');
    }
    result
}

// Better: Avoid allocation entirely
fn process_strings_better(input: &str) -> impl Iterator<Item = &str> + '_ {
    input.split_whitespace()
}
```

### Arena Allocator

```rust
use typed_arena::Arena;

struct Node<'a> {
    value: i32,
    children: Vec<&'a Node<'a>>,
}

fn build_tree<'a>(arena: &'a Arena<Node<'a>>) -> &'a Node<'a> {
    let left = arena.alloc(Node {
        value: 1,
        children: vec![],
    });

    let right = arena.alloc(Node {
        value: 2,
        children: vec![],
    });

    arena.alloc(Node {
        value: 0,
        children: vec![left, right],
    })
}

fn main() {
    let arena = Arena::new();
    let tree = build_tree(&arena);
    // All nodes freed at once when arena drops
}
```

### SIMD Optimization

```rust
use std::simd::{f32x4, SimdFloat};

// Scalar version
fn dot_product_scalar(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .sum()
}

// SIMD version (4x parallelism)
fn dot_product_simd(a: &[f32], b: &[f32]) -> f32 {
    let chunks = a.len() / 4;
    let mut sum = f32x4::splat(0.0);

    for i in 0..chunks {
        let a_vec = f32x4::from_slice(&a[i * 4..]);
        let b_vec = f32x4::from_slice(&b[i * 4..]);
        sum += a_vec * b_vec;
    }

    sum.reduce_sum() +
        a[chunks * 4..].iter()
            .zip(&b[chunks * 4..])
            .map(|(x, y)| x * y)
            .sum::<f32>()
}
```

### Cache-Friendly Data Structures

```rust
// Bad: Array of Structs (AoS) - poor cache locality
struct ParticleBad {
    x: f32,
    y: f32,
    z: f32,
    vx: f32,
    vy: f32,
    vz: f32,
}

fn update_positions_aos(particles: &mut [ParticleBad], dt: f32) {
    for p in particles.iter_mut() {
        p.x += p.vx * dt;
        p.y += p.vy * dt;
        p.z += p.vz * dt;
    }
}

// Good: Struct of Arrays (SoA) - better cache locality
struct ParticlesGood {
    x: Vec<f32>,
    y: Vec<f32>,
    z: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,
    vz: Vec<f32>,
}

fn update_positions_soa(particles: &mut ParticlesGood, dt: f32) {
    for i in 0..particles.x.len() {
        particles.x[i] += particles.vx[i] * dt;
        particles.y[i] += particles.vy[i] * dt;
        particles.z[i] += particles.vz[i] * dt;
    }
}
```

### Branch Prediction

```rust
// Bad: Unpredictable branches
fn process_data_bad(data: &[i32]) -> i32 {
    let mut sum = 0;
    for &x in data {
        if x > 50 {  // Unpredictable
            sum += x;
        }
    }
    sum
}

// Good: Branchless
fn process_data_good(data: &[i32]) -> i32 {
    data.iter()
        .map(|&x| x * (x > 50) as i32)
        .sum()
}

// Better: Use iterators to let compiler optimize
fn process_data_better(data: &[i32]) -> i32 {
    data.iter()
        .filter(|&&x| x > 50)
        .sum()
}
```

### Small String Optimization

```rust
// Use smallvec for small collections
use smallvec::{SmallVec, smallvec};

// No heap allocation for <= 4 elements
let mut v: SmallVec<[i32; 4]> = smallvec![1, 2, 3];
v.push(4);  // Still on stack

v.push(5);  // Now moves to heap

// Use compact_str for small strings
use compact_str::CompactString;

let s = CompactString::new("hello");  // Inline storage
```

### Const Evaluation

```rust
// Compute at compile time
const fn fibonacci_const(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            let mut i = 2;
            while i <= n {
                let temp = a;
                a = b;
                b = temp + b;
                i += 1;
            }
            b
        }
    }
}

const FIB_10: u32 = fibonacci_const(10);  // Computed at compile time

// Look-up tables
const POWERS_OF_TWO: [u64; 64] = {
    let mut table = [0; 64];
    let mut i = 0;
    while i < 64 {
        table[i] = 1 << i;
        i += 1;
    }
    table
};

fn get_power_of_two(n: usize) -> u64 {
    POWERS_OF_TWO[n]  // O(1) lookup
}
```

### Zero-Copy Parsing

```rust
// Bad: Copies strings
fn parse_csv_bad(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.to_string())  // Allocation!
                .collect()
        })
        .collect()
}

// Good: Zero-copy with lifetimes
fn parse_csv_good(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split(',').collect())
        .collect()
}

// Better: Lazy iteration
fn parse_csv_better(input: &str) -> impl Iterator<Item = impl Iterator<Item = &str>> {
    input.lines().map(|line| line.split(','))
}
```

### Memory Pool

```rust
use std::sync::Mutex;

struct Pool<T> {
    objects: Mutex<Vec<T>>,
}

impl<T> Pool<T> {
    fn new() -> Self {
        Pool {
            objects: Mutex::new(Vec::new()),
        }
    }

    fn acquire(&self) -> Option<T> {
        self.objects.lock().unwrap().pop()
    }

    fn release(&self, obj: T) {
        self.objects.lock().unwrap().push(obj);
    }
}

// Usage
lazy_static::lazy_static! {
    static ref BUFFER_POOL: Pool<Vec<u8>> = Pool::new();
}

fn process_data() {
    let mut buffer = BUFFER_POOL.acquire()
        .unwrap_or_else(|| Vec::with_capacity(4096));

    // Use buffer...

    buffer.clear();
    BUFFER_POOL.release(buffer);
}
```

### Inline Hints

```rust
// Force inline for small, hot functions
#[inline(always)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Suggest inline
#[inline]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Never inline (for better debugging)
#[inline(never)]
fn cold_function() {
    // Rarely called
}
```

### Lazy Static and OnceCell

```rust
use std::sync::OnceLock;

// Lazy initialization
static EXPENSIVE_RESOURCE: OnceLock<Vec<i32>> = OnceLock::new();

fn get_resource() -> &'static Vec<i32> {
    EXPENSIVE_RESOURCE.get_or_init(|| {
        // Expensive computation
        (0..1000000).collect()
    })
}

// Thread-local cache
use std::cell::RefCell;

thread_local! {
    static CACHE: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(4096));
}

fn process() {
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        // Use cache
        cache.clear();
    });
}
```

### Parallel Processing with Rayon

```rust
use rayon::prelude::*;

// Sequential
fn process_sequential(data: &[i32]) -> Vec<i32> {
    data.iter()
        .map(|&x| expensive_computation(x))
        .collect()
}

// Parallel
fn process_parallel(data: &[i32]) -> Vec<i32> {
    data.par_iter()
        .map(|&x| expensive_computation(x))
        .collect()
}

fn expensive_computation(x: i32) -> i32 {
    x * x + 42
}
```

### Copy-on-Write (CoW)

```rust
use std::borrow::Cow;

fn process_maybe_modify<'a>(input: &'a str, should_modify: bool) -> Cow<'a, str> {
    if should_modify {
        Cow::Owned(input.to_uppercase())  // Allocates
    } else {
        Cow::Borrowed(input)  // No allocation
    }
}

fn main() {
    let result1 = process_maybe_modify("hello", false);
    // No allocation: result1 is Borrowed

    let result2 = process_maybe_modify("hello", true);
    // Allocates: result2 is Owned("HELLO")
}
```

### Profile-Guided Optimization (PGO)

```bash
# Step 1: Build with instrumentation
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
    cargo build --release

# Step 2: Run with typical workload
./target/release/my_program

# Step 3: Merge profile data
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

# Step 4: Build with profile data
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
    cargo build --release
```

### Link-Time Optimization (LTO)

```toml
# Cargo.toml
[profile.release]
lto = true              # Enable LTO
codegen-units = 1       # Better optimization, slower compile
opt-level = 3           # Maximum optimization
```

## Performance Checklist

### Before Optimizing

1. **Profile First** - Measure, don't guess
2. **Algorithm** - Better algorithm beats micro-optimizations
3. **Bottlenecks** - Focus on hot paths (80/20 rule)
4. **Measure Impact** - Benchmark before and after

### Common Optimizations

1. **Reduce Allocations**
   - Pre-allocate with capacity
   - Reuse buffers
   - Use stack allocation when possible
   - Consider arena allocators

2. **Improve Cache Locality**
   - Use SoA instead of AoS
   - Process data sequentially
   - Keep hot data together

3. **Minimize Copies**
   - Pass by reference
   - Use iterators
   - Zero-copy parsing
   - Copy-on-write

4. **Leverage Parallelism**
   - Use rayon for data parallelism
   - SIMD for vector operations
   - Async for I/O-bound work

5. **Compile-time Computation**
   - const fn for constants
   - Build-time code generation
   - Macros for specialization

### Profiling Workflow

```bash
# 1. CPU profiling with perf
perf record --call-graph=dwarf cargo run --release
perf report

# 2. Flamegraph
cargo flamegraph

# 3. Memory profiling
valgrind --tool=massif ./target/release/app

# 4. Heap profiling
valgrind --tool=dhat ./target/release/app

# 5. Cache profiling
valgrind --tool=cachegrind ./target/release/app
perf stat -e cache-misses,cache-references ./target/release/app
```

## Benchmarking Best Practices

1. **Use criterion** - Statistical analysis, outlier detection
2. **Black box** - Prevent optimizer from eliminating code
3. **Warm up** - Account for CPU scaling, caches
4. **Iterations** - Multiple runs for statistical significance
5. **Isolate** - Benchmark one thing at a time
6. **Real data** - Use realistic inputs

## Common Pitfalls

1. **Premature Optimization** - Profile first!
2. **Micro-optimizations** - Focus on algorithms
3. **Debug Builds** - Always benchmark in release mode
4. **Cold Cache** - Warm up before measuring
5. **Compiler Optimization** - Use black_box to prevent elimination

## Platform-Specific Optimizations

### x86/x64

```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[target_feature(enable = "avx2")]
unsafe fn sum_avx2(data: &[f32]) -> f32 {
    // AVX2 implementation
    // 8 floats at a time
    0.0
}
```

### ARM NEON

```rust
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

#[target_feature(enable = "neon")]
unsafe fn sum_neon(data: &[f32]) -> f32 {
    // NEON implementation
    0.0
}
```

## Tools and Crates

### Profiling
- `flamegraph` - Flame graph generation
- `perf` - Linux performance counters
- `valgrind` - Memory profiling
- `heaptrack` - Heap profiling

### Benchmarking
- `criterion` - Statistical benchmarking
- `iai` - Cachegrind-based benchmarking
- `divan` - Fast benchmarking

### Optimization
- `rayon` - Data parallelism
- `parking_lot` - Faster locks
- `ahash` - Fast hashing
- `smallvec` - Small vector optimization
- `compact_str` - Small string optimization
- `typed-arena` - Arena allocator

## Resources

- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion Documentation](https://bheisler.github.io/criterion.rs/book/)
- [LLVM Optimization Remarks](https://llvm.org/docs/Remarks.html)
- [Intel Intrinsics Guide](https://software.intel.com/sites/landingpage/IntrinsicsGuide/)
- [Godbolt Compiler Explorer](https://godbolt.org/)

## Next Steps

After completing this section:
- Profile a real application
- Optimize hot paths
- Implement SIMD algorithms
- Study assembly output
- Contribute performance improvements to open source

---

**Estimated Time**: 20-25 hours
**Difficulty**: ★★★★★ (Expert)
**Prerequisites**: Strong Rust, computer architecture, assembly basics
