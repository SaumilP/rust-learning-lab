// Exercise 11: Code Quality and Clippy (BROKEN CODE)
//
// This code has clippy warnings related to code quality and idioms
// Your job: Run cargo clippy and fix the warnings for better Rust code

fn main() {
    println!("=== Code Quality Exercise ===\n");

    println!("=== Test 1: Process numbers ===");
    process_numbers();

    println!("\n=== Test 2: String operations ===");
    string_operations();

    println!("\n=== Test 3: Data validation ===");
    data_validation();
}

fn process_numbers() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", numbers);

    // BUG 1: Using manual loop instead of iterator methods
    // Clippy warning: manual_flatten or similar
    let doubled = Vec::new();
    let mut doubled = doubled;
    for i in 0..numbers.len() {
        doubled.push(numbers[i] * 2);
    }
    println!("Doubled: {:?}", doubled);

    // BUG 2: Unnecessary clone
    // Clippy warning: clone_on_copy or clone_double_ref
    let cloned = numbers.clone();
    let reference = cloned;  // Cloned unnecessarily
    println!("Reference: {:?}", reference);

    // BUG 3: Inefficient filtering and mapping
    // Could be one iterator chain
    let mut evens = Vec::new();
    for num in &numbers {
        if num % 2 == 0 {
            evens.push(num * num);
        }
    }
    println!("Even squares: {:?}", evens);
}

fn string_operations() {
    let text = "Hello, World!";
    println!("Original: \"{}\"", text);

    // BUG 4: Unnecessary else after return
    // Clippy warning: needless_else
    let result = if text.contains("Hello") {
        return "Found greeting".to_string();
    } else {
        "No greeting".to_string()
    };
    println!("Result: {}", result);
}

fn data_validation() {
    let ages = vec![25, 30, 35];

    // BUG 5: Checking Result incorrectly
    // Could use is_ok/is_err or match
    let results: Vec<Result<(), String>> = ages.iter()
        .map(|age| {
            if *age >= 18 {
                Ok(())
            } else {
                Err("Too young".to_string())
            }
        })
        .collect();

    // BUG 6: Unnecessary iteration with for loop
    // Could use iterator methods like for_each
    let mut valid_count = 0;
    for result in results {
        match result {
            Ok(()) => {
                valid_count += 1;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    println!("Valid count: {}", valid_count);

    // BUG 7: Using push in loop instead of collecting
    let filtered = Vec::new();
    let mut filtered = filtered;
    for age in ages {
        if age > 25 {
            filtered.push(age);
        }
    }
    println!("Filtered ages: {:?}", filtered);
}

// BUGS SUMMARY:
// 1. Manual loop with index access - use iterator methods instead
// 2. Unnecessary clone - use references
// 3. Multiple operations in loops - chain iterator methods
// 4. Unnecessary else after return - remove else
// 5. Result handling in map - could be cleaner
// 6. For loop counting - use iterator methods
// 7. Push in loop - use filter/collect pattern

// EXPECTED BEHAVIOR AFTER FIXES:
// === Code Quality Exercise ===
//
// === Test 1: Process numbers ===
// Original: [1, 2, 3, 4, 5]
// Doubled: [2, 4, 6, 8, 10]
// Reference: [1, 2, 3, 4, 5]
// Even squares: [4, 16]
//
// === Test 2: String operations ===
// Original: "Hello, World!"
// Result: Found greeting
//
// === Test 3: Data validation ===
// Valid count: 3
// Filtered ages: [30, 35]
//
// No clippy warnings!
