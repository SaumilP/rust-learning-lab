// Exercise 5: Collection Manipulation (BROKEN CODE)
//
// This code has 3 bugs related to collection operations and iterators
// Your job: Find and fix the bugs so it compiles and produces correct output

use std::collections::HashMap;

fn main() {
    println!("=== Collection Manipulation ===\n");

    println!("=== Test 1: Vector Operations ===");
    vector_operations();

    println!("\n=== Test 2: Word Frequency ===");
    word_frequency();

    println!("\n=== Test 3: Number Grouping ===");
    number_grouping();
}

fn vector_operations() {
    let mut numbers = vec![2, 4, 1, 5, 3, 8];
    println!("Original vector: {:?}", numbers);
    println!("Length: {}", numbers.len());

    numbers.push(7);
    println!("After push 7: {:?}", numbers);

    numbers.pop();
    println!("After pop: {:?}", numbers);

    // BUG 1: Using into_iter() instead of iter() - this consumes the vector
    // After this operation, 'numbers' cannot be used again
    let evens: Vec<i32> = numbers.into_iter()
        .filter(|&n| n % 2 == 0)
        .collect();
    println!("Even numbers: {:?}", evens);

    // This won't compile because 'numbers' was consumed by into_iter()
    let squared: Vec<i32> = evens.iter()
        .map(|n| n * n)
        .collect();
    println!("Even numbers squared: {:?}", squared);

    // BUG 2: This tries to use 'numbers' but it was already consumed
    let doubled: Vec<i32> = numbers.iter()  // Error: numbers already moved
        .map(|n| n * 2)
        .collect();
    println!("Doubled all: {:?}", doubled);
}

fn word_frequency() {
    let text = "rust is great rust is fun";
    let mut word_count: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        // BUG 3: Not using the entry() API correctly for counting
        // This code doesn't actually count properly
        if word_count.contains_key(word) {
            word_count.insert(word, word_count[word] + 1);
        } else {
            word_count.insert(word, 1);
        }
    }

    println!("Word counts:");
    for (word, count) in word_count.iter() {
        println!("  {}: {}", word, count);
    }
}

fn number_grouping() {
    let numbers = vec![10, 15, 20, 25, 30, 35];
    println!("Numbers: {:?}", numbers);

    // Partition into even and odd
    // BUG 4: partition returns (true_vec, false_vec) in wrong order
    // partition(|x| x % 2 == 0) returns (even, odd) but code assumes opposite
    let (odd, even): (Vec<_>, Vec<_>) = numbers.iter()
        .partition(|&&n| n % 2 == 0);  // This returns EVEN in first tuple element

    println!("Even: {:?}", even);
    let even_sum: i32 = even.iter().map(|&&n| n).sum();
    let even_avg = even_sum as f64 / even.len() as f64;
    println!(" (sum: {}, avg: {})", even_sum, even_avg);

    println!("Odd: {:?}", odd);
    let odd_sum: i32 = odd.iter().map(|&&n| n).sum();
    let odd_avg = odd_sum as f64 / odd.len() as f64;
    println!(" (sum: {}, avg: {})", odd_sum, odd_avg);
}

// BUGS SUMMARY:
// 1. Using into_iter() instead of iter() - consumes the original vector
// 2. Trying to use 'numbers' after it was consumed by into_iter()
// 3. Word counting logic is inefficient - should use entry() API
// 4. Partition tuple unpacking is reversed - variable names are swapped

// EXPECTED BEHAVIOR:
// === Test 1: Vector Operations ===
// Original vector: [2, 4, 1, 5, 3, 8]
// Length: 6
// After push 7: [2, 4, 1, 5, 3, 8, 7]
// After pop: [2, 4, 1, 5, 3, 8]
// Even numbers: [2, 4, 8]
// Even numbers squared: [4, 16, 64]
// Doubled all: [4, 8, 2, 10, 6, 16]
//
// === Test 2: Word Frequency ===
// Word counts:
//   rust: 2
//   is: 2
//   great: 1
//   fun: 1
//
// === Test 3: Number Grouping ===
// Numbers: [10, 15, 20, 25, 30, 35]
// Even: [10, 20, 30] (sum: 60, avg: 20)
// Odd: [15, 25, 35] (sum: 75, avg: 25)
