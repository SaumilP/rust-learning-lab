// Example: break and continue
//
// Demonstrates:
// - Using break to exit loops early
// - Using continue to skip iterations
// - Break with values
// - Labeled breaks and continues
// - Common patterns with loop control

fn main() {
    println!("=== Basic break ===\n");

    // break exits the loop immediately
    let mut count = 0;
    loop {
        count += 1;
        println!("Count: {}", count);
        if count >= 5 {
            println!("Breaking loop at count = {}", count);
            break;
        }
    }

    println!("\n=== break in while Loop ===\n");

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 7;
    let mut found_index = None;

    let mut i = 0;
    while i < numbers.len() {
        if numbers[i] == target {
            found_index = Some(i);
            break; // Exit early when found
        }
        i += 1;
    }

    match found_index {
        Some(idx) => println!("Found {} at index {}", target, idx),
        None => println!("{} not found", target),
    }

    println!("\n=== break in for Loop ===\n");

    let items = ["apple", "banana", "cherry", "date", "elderberry"];

    for (i, item) in items.iter().enumerate() {
        if item.starts_with('c') {
            println!("Found item starting with 'c': '{}' at index {}", item, i);
            break;
        }
        println!("Checking: {}", item);
    }
    println!("Search complete");

    println!("\n=== break with Value ===\n");

    // loop can return a value via break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter * 2; // Returns 20
        }
    };
    println!("Loop result: {}", result);

    // Finding with loop
    let numbers = [4, 8, 15, 16, 23, 42];
    let found = loop {
        break numbers.iter().find(|&&x| x > 20);
    };
    println!("First number > 20: {:?}", found);

    println!("\n=== Basic continue ===\n");

    // continue skips to the next iteration
    println!("Odd numbers from 1 to 10:");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        println!("  {}", i);
    }

    println!("\n=== continue in while Loop ===\n");

    let mut i = 0;
    println!("Numbers not divisible by 3 (1-10):");
    while i < 10 {
        i += 1;
        if i % 3 == 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!();

    println!("\n=== Filtering with continue ===\n");

    let data = vec![
        ("Alice", 25),
        ("Bob", 17),
        ("Charlie", 30),
        ("Diana", 15),
        ("Eve", 22),
    ];

    println!("Adults only:");
    for (name, age) in &data {
        if *age < 18 {
            continue; // Skip minors
        }
        println!("  {} (age {})", name, age);
    }

    println!("\n=== Labeled break ===\n");

    // Breaking out of nested loops
    'outer: for i in 0..5 {
        println!("Outer loop: i = {}", i);
        for j in 0..5 {
            println!("  Inner loop: j = {}", j);
            if i == 2 && j == 2 {
                println!("  Breaking out of both loops!");
                break 'outer;
            }
        }
    }
    println!("After nested loops");

    println!("\n=== Labeled continue ===\n");

    // Continuing the outer loop from inner loop
    'rows: for row in 1..=4 {
        print!("Row {}: ", row);
        for col in 1..=4 {
            if col == 3 {
                println!("(skip row at col 3)");
                continue 'rows; // Skip rest of this row
            }
            print!("{} ", col);
        }
        println!();
    }

    println!("\n=== Complex Loop Control ===\n");

    // Matrix search with labeled breaks
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let target = 5;
    let mut position = None;

    'search: for (row, row_vec) in matrix.iter().enumerate() {
        for (col, &value) in row_vec.iter().enumerate() {
            if value == target {
                position = Some((row, col));
                break 'search;
            }
        }
    }

    match position {
        Some((r, c)) => println!("Found {} at ({}, {})", target, r, c),
        None => println!("{} not found in matrix", target),
    }

    println!("\n=== Validation Loop ===\n");

    // Simulating input validation
    let inputs = vec!["invalid", "bad", "good", "also good"];

    for input in inputs {
        // Early skip for invalid inputs
        if input == "invalid" || input == "bad" {
            println!("Skipping invalid input: '{}'", input);
            continue;
        }
        println!("Processing valid input: '{}'", input);
    }

    println!("\n=== break vs continue Summary ===\n");

    // break example
    println!("break: stop at first even number");
    for i in 1..10 {
        print!("{} ", i);
        if i % 2 == 0 {
            println!("<- break here");
            break;
        }
    }

    // continue example
    println!("\ncontinue: skip even numbers");
    for i in 1..10 {
        if i % 2 == 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!();

    println!("\n=== Loop Control in Functions ===\n");

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    if let Some(first_even) = find_first_even(&nums) {
        println!("First even number: {}", first_even);
    }

    let result = sum_until_negative(&[1, 2, 3, -1, 4, 5]);
    println!("Sum until negative: {}", result);

    println!("\n=== Processing with Early Exit ===\n");

    let result = process_items(&["start", "data", "error", "more"]);
    println!("Processing result: {:?}", result);
}

// --- Helper functions ---

/// Find the first even number in a slice
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num); // return also acts as break
        }
    }
    None
}

/// Sum numbers until a negative number is encountered
fn sum_until_negative(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in numbers {
        if num < 0 {
            break; // Stop summing
        }
        sum += num;
    }
    sum
}

/// Process items, stopping at first error
fn process_items(items: &[&str]) -> Result<Vec<String>, &'static str> {
    let mut results = Vec::new();

    for item in items {
        if *item == "error" {
            return Err("Error encountered during processing");
        }
        results.push(item.to_uppercase());
    }

    Ok(results)
}
