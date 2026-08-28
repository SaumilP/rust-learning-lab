// Exercise 2: Iterator Chain Puzzle (BROKEN CODE)
//
// This code has 4 bugs related to iterator chaining and type handling
// Your job: Find and fix the bugs so it compiles and produces correct output

fn main() {
    println!("=== Number Processing Pipeline ===");
    number_pipeline();

    println!("\n=== String Processing ===");
    string_processing();

    println!("\n=== Data Aggregation ===");
    data_aggregation();
}

fn number_pipeline() {
    println!("Range: 1..=20");

    // Get even numbers
    let evens: Vec<i32> = (1..=20).filter(|x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);

    // BUG 1: Need to square the values, but the map closure has wrong syntax
    let squared: Vec<i32> = evens.iter().map(|x| x * x).collect();
    println!("Squared: {:?}", squared);

    // Skip first 2, take 3
    // BUG 2: Using wrong method - skip() and take() work on iterators, not vectors directly
    let final_three: Vec<i32> = squared.skip(2).take(3).collect();
    println!("Skip 2, take 3: {:?}", final_three);

    // Calculate sum
    let sum: i32 = final_three.iter().sum();
    println!("Sum: {}", sum);

    // BUG 3: product() needs the iterator element type to implement Product trait
    // Also need to handle the reference type correctly
    let product: i32 = final_three.iter().product();
    println!("Product: {}", product);
}

fn string_processing() {
    let text = "the quick brown fox jumps over the lazy dog";
    println!("Original: \"{}\"", text);

    // Filter words longer than 3 characters
    let long_words: Vec<&str> = text
        .split_whitespace()
        .filter(|word| word.len() > 3)
        .collect();
    println!("Words > 3 chars: {:?}", long_words);

    // BUG 4: to_uppercase() returns String, but we're collecting wrong type
    let uppercase: Vec<&str> = long_words
        .iter()
        .map(|word| word.to_uppercase())
        .collect();
    println!("Uppercase: {:?}", uppercase);

    // Join with separator
    let joined = uppercase.join("-");
    println!("Joined: \"{}\"", joined);
}

fn data_aggregation() {
    let numbers: Vec<i32> = (1..=10).collect();
    println!("Numbers: {:?}", numbers);

    // Running sum using fold
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Running sum using fold: {}", sum);

    // Running product using fold
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Running product using fold: {}", product);

    // Count of odd numbers
    let count_odds = numbers.iter().filter(|&&x| x % 2 != 0).count();
    println!("Count of odds: {}", count_odds);

    // Sum of squares of even numbers
    let sum_squares_evens: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    println!("Sum of squares of evens: {}", sum_squares_evens);
}

// BUGS SUMMARY:
// 1. When mapping over iter(), x is &i32, so x * x tries to multiply references
//    Fix: use |&x| x * x or |x| *x * *x
// 2. skip() and take() are iterator methods, not Vec methods
//    Fix: squared.iter().skip(2).take(3).cloned().collect()
// 3. product() over iter() gives references; need to clone or use into_iter()
//    Fix: final_three.iter().cloned().product() or final_three.into_iter().product()
// 4. to_uppercase() returns String, can't collect into Vec<&str>
//    Fix: Change type to Vec<String>

// EXPECTED OUTPUT:
// === Number Processing Pipeline ===
// Range: 1..=20
// Even numbers: [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
// Squared: [4, 16, 36, 64, 100, 144, 196, 256, 324, 400]
// Skip 2, take 3: [36, 64, 100]
// Sum: 200
// Product: 230400
//
// === String Processing ===
// Original: "the quick brown fox jumps over the lazy dog"
// Words > 3 chars: ["quick", "brown", "jumps", "over", "lazy"]
// Uppercase: ["QUICK", "BROWN", "JUMPS", "OVER", "LAZY"]
// Joined: "QUICK-BROWN-JUMPS-OVER-LAZY"
//
// === Data Aggregation ===
// Numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// Running sum using fold: 55
// Running product using fold: 3628800
// Count of odds: 5
// Sum of squares of evens: 220
