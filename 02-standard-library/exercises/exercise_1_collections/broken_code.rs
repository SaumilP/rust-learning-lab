// Exercise 1: Collection Manipulation (BROKEN CODE)
//
// This code has 4 bugs related to collections and iteration
// Your job: Find and fix the bugs so it compiles and produces correct output

use std::collections::HashMap;

fn main() {
    println!("=== Vector Operations ===");
    vector_operations();

    println!("\n=== Word Frequency ===");
    word_frequency();

    println!("\n=== Statistics ===");
    statistics();
}

fn vector_operations() {
    // BUG 1: Vector needs to be mutable for push/pop operations
    let numbers = vec![5, 2, 8, 1, 9, 3, 7];
    println!("Original: {:?}", numbers);

    numbers.push(4);
    println!("After push 4: {:?}", numbers);

    numbers.pop();
    println!("After pop: {:?}", numbers);

    numbers.remove(2);
    println!("After remove index 2: {:?}", numbers);

    // BUG 2: filter() needs a reference comparison, not direct comparison
    let even: Vec<i32> = numbers.iter().filter(|x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even);

    // Find min and max
    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();
    println!("Min: {}, Max: {}", min, max);
}

fn word_frequency() {
    let text = "the quick brown fox jumps over the lazy dog the";
    println!("Text: \"{}\"", text);

    // BUG 3: Need to handle the entry API correctly for counting
    let mut word_counts = HashMap::new();
    for word in text.split_whitespace() {
        // This doesn't increment correctly - .or_insert returns &mut but we need to dereference
        let count = word_counts.entry(word).or_insert(0);
        count + 1;  // This doesn't actually modify count
    }

    println!("Word counts:");
    for (word, count) in &word_counts {
        println!("  {}: {}", word, count);
    }

    // Find most common word
    // BUG 4: max_by_key returns Option, need to handle it properly
    let most_common = word_counts.iter().max_by_key(|(_, count)| count);
    println!("Most common word: \"{}\" ({} times)", most_common.0, most_common.1);
}

fn statistics() {
    let numbers = vec![10, 20, 30, 40, 50];
    println!("Numbers: {:?}", numbers);

    // Calculate sum
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    // Calculate average
    let average = sum / numbers.len() as i32;
    println!("Average: {}", average);

    // Count elements greater than 25
    let count_gt_25 = numbers.iter().filter(|&&x| x > 25).count();
    println!("Count > 25: {}", count_gt_25);

    // Find elements above average
    let above_avg: Vec<&i32> = numbers.iter().filter(|&&x| x >= average).collect();
    println!("Above average: {:?}", above_avg);
}

// BUGS SUMMARY:
// 1. Vector `numbers` needs `mut` keyword to allow push/pop/remove operations
// 2. In filter closure, `x` is &&i32, need to dereference: `|x| *x % 2 == 0` or `|&&x| x % 2 == 0`
// 3. Entry API: `count + 1` doesn't modify count - need `*count += 1`
// 4. max_by_key returns Option<(&K, &V)> - need to unwrap or handle None case

// EXPECTED OUTPUT:
// === Vector Operations ===
// Original: [5, 2, 8, 1, 9, 3, 7]
// After push 4: [5, 2, 8, 1, 9, 3, 7, 4]
// After pop: [5, 2, 8, 1, 9, 3, 7]
// After remove index 2: [5, 2, 1, 9, 3, 7]
// Even numbers: [2]
// Min: 1, Max: 9
//
// === Word Frequency ===
// Text: "the quick brown fox jumps over the lazy dog the"
// Word counts:
//   the: 3
//   quick: 1
//   ... (other words)
// Most common word: "the" (3 times)
//
// === Statistics ===
// Numbers: [10, 20, 30, 40, 50]
// Sum: 150
// Average: 30.0
// Count > 25: 3
// Above average: [30, 40, 50]
