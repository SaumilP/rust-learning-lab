// Exercise 6: Iterator Chains and Transformations (BROKEN CODE)
//
// This code has 3 bugs related to iterator chaining and method ordering
// Your job: Find and fix the bugs so it compiles and produces correct output

fn main() {
    println!("=== Iterator Chains and Transformations ===\n");

    println!("=== Test 1: Number Pipeline ===");
    number_pipeline();

    println!("\n=== Test 2: String Processing ===");
    string_processing();

    println!("\n=== Test 3: Statistics ===");
    statistics();
}

fn number_pipeline() {
    let numbers: Vec<i32> = (1..=20).collect();
    println!("Original: 1..=20");

    // Filter evens
    let evens: Vec<i32> = numbers.iter()
        .filter(|&&n| n % 2 == 0)
        .map(|n| *n)
        .collect();
    println!("Evens: {:?}", evens);

    // Square the evens
    let squared: Vec<i32> = evens.iter()
        .map(|n| n * n)
        .collect();
    println!("Evens squared: {:?}", squared);

    // BUG 1: Method order is wrong
    // Can't call collect() in the middle of a chain
    // collect() is a terminal operation that ends the chain
    let result: Vec<i32> = squared.iter()
        .collect()  // This ends the chain too early
        .skip(2)    // Error: can't call skip on Vec<i32>
        .take(3)
        .map(|&n| n)
        .collect();
    println!("Skip 2, take 3: {:?}", result);

    // Calculate sum - BUG 1 affects this
    let sum: i32 = result.iter().sum();
    println!("Sum of result: {}", sum);

    // Calculate product
    let product: i32 = result.iter().map(|&n| n).product();
    println!("Product of result: {}", product);
}

fn string_processing() {
    let text = "the quick brown fox jumps";
    println!("Original: \"{}\"", text);

    // BUG 2: Not dereferencing properly in map closure
    let words: Vec<&str> = text.split_whitespace()
        .filter(|w| w.len() > 3)
        .collect();
    println!("Words with length > 3: {:?}", words);

    // Convert to uppercase
    let uppercase: Vec<String> = words.iter()
        .map(|w| w.to_uppercase())  // w is &&str, need to dereference
        .collect();
    println!("Uppercase: {:?}", uppercase);

    // Enumerate
    let enumerated: Vec<(usize, &String)> = uppercase.iter()
        .enumerate()
        .map(|(i, w)| (i, w))  // This is redundant but type is wrong
        .collect();
    println!("Enumerated: {:?}", enumerated);

    // Join with dashes
    // BUG 3: Can't join String references directly
    let joined = uppercase.iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("-");
    println!("Joined: \"{}\"", joined);
}

fn statistics() {
    let numbers = vec![5, 12, 8, 15, 3, 20, 7, 9];
    println!("Numbers: {:?}", numbers);

    // Filter and collect
    let filtered: Vec<i32> = numbers.iter()
        .filter(|&&n| n > 5)
        .map(|&n| n)
        .collect();
    println!("Greater than 5: {:?}", filtered);

    let count = filtered.len();
    println!("Count: {}", count);

    // Sum
    let sum: i32 = filtered.iter().sum();
    println!("Sum: {}", sum);

    // Average
    let average = sum as f64 / count as f64;
    println!("Average: {:.2}", average);
}

// BUGS SUMMARY:
// 1. Collect() is a terminal operation - can't chain methods after it
//    Should use skip().take() before collect()
// 2. Dereferencing issues in closures - may need ** or proper type handling
// 3. Type mismatches in string operations - Vec<String> vs Vec<&str>

// EXPECTED BEHAVIOR:
// === Test 1: Number Pipeline ===
// Original: 1..=20
// Evens: [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
// Evens squared: [4, 16, 36, 64, 100, 144, 196, 256, 324, 400]
// Skip 2, take 3: [36, 64, 100]
// Sum of result: 200
// Product of result: 230400
//
// === Test 2: String Processing ===
// Original: "the quick brown fox jumps"
// Words with length > 3: ["quick", "brown", "jumps"]
// Uppercase: ["QUICK", "BROWN", "JUMPS"]
// Enumerated: [(0, "QUICK"), (1, "BROWN"), (2, "JUMPS")]
// Joined: "QUICK-BROWN-JUMPS"
//
// === Test 3: Statistics ===
// Numbers: [5, 12, 8, 15, 3, 20, 7, 9]
// Greater than 5: [12, 8, 15, 20, 7, 9]
// Count: 6
// Sum: 71
// Average: 11.83
