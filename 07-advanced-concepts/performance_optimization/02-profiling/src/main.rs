use std::collections::HashMap;

// Intentionally inefficient function for profiling
fn inefficient_string_concat(count: usize) -> String {
    let mut result = String::new();
    for i in 0..count {
        // Bad: Multiple reallocations
        result = format!("{}{}", result, i);
    }
    result
}

// Better version
fn efficient_string_concat(count: usize) -> String {
    let mut result = String::with_capacity(count * 10);
    for i in 0..count {
        use std::fmt::Write;
        write!(&mut result, "{}", i).unwrap();
    }
    result
}

// Simulate CPU-intensive work
fn compute_primes(limit: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    for n in 2..=limit {
        let mut is_prime = true;
        for p in &primes {
            if n % p == 0 {
                is_prime = false;
                break;
            }
            if p * p > n {
                break;
            }
        }
        if is_prime {
            primes.push(n);
        }
    }
    primes
}

// Memory-intensive work
fn create_large_map(size: usize) -> HashMap<usize, Vec<usize>> {
    let mut map = HashMap::new();
    for i in 0..size {
        map.insert(i, vec![i; 100]);
    }
    map
}

// Hot path simulation
fn hot_path_demo() {
    let data: Vec<i32> = (0..1_000_000).collect();

    // This loop will show up prominently in profiling
    let sum: i64 = data
        .iter()
        .map(|&x| x as i64)
        .map(|x| x * x)
        .map(|x| x + 1)
        .sum();

    println!("Sum: {}", sum);
}

fn main() {
    println!("🔍 Profiling Demo\n");

    println!("=== String Concatenation ===");
    let start = std::time::Instant::now();
    let result = inefficient_string_concat(1000);
    println!("Inefficient took: {:?} (len: {})", start.elapsed(), result.len());

    let start = std::time::Instant::now();
    let result = efficient_string_concat(1000);
    println!("Efficient took: {:?} (len: {})", start.elapsed(), result.len());

    println!("\n=== Prime Computation ===");
    let start = std::time::Instant::now();
    let primes = compute_primes(10000);
    println!("Found {} primes in {:?}", primes.len(), start.elapsed());

    println!("\n=== Memory Allocation ===");
    let start = std::time::Instant::now();
    let map = create_large_map(10000);
    println!("Created map with {} entries in {:?}", map.len(), start.elapsed());

    println!("\n=== Hot Path ===");
    let start = std::time::Instant::now();
    hot_path_demo();
    println!("Hot path took: {:?}", start.elapsed());

    println!("\n✅ Profiling demo completed!");
    println!("\nTo profile this program:");
    println!("1. CPU: cargo flamegraph");
    println!("2. Memory: valgrind --tool=massif ./target/release/profiling");
    println!("3. Cachegrind: valgrind --tool=cachegrind ./target/release/profiling");
}
