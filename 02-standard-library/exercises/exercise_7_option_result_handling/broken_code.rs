// Exercise 7: Option and Result Handling (BROKEN CODE)
//
// This code has 3 bugs related to Result/Option handling and error management
// Your job: Find and fix the bugs so it compiles and produces correct output

fn main() {
    println!("=== Option and Result Handling ===\n");

    println!("=== Test 1: Number Parsing ===");
    number_parsing();

    println!("\n=== Test 2: Collection Access ===");
    collection_access();

    println!("\n=== Test 3: Validation ===");
    validation();
}

fn number_parsing() {
    let strings = vec!["42", "abc", "87", "xyz", "-15"];

    println!("Parsing strings:");
    let mut successes = Vec::new();
    let mut failures = Vec::new();

    for s in strings {
        // BUG 1: Using unwrap() instead of proper error handling
        // This will panic when parsing "abc" or "xyz"
        let num: i32 = s.parse().unwrap();
        successes.push(num);
    }

    println!("Success: {:?}", successes);
    println!("Failed: {:?}", failures);
}

fn collection_access() {
    let numbers = vec![10, 20, 30, 40, 50];
    println!("Vector: {:?}", numbers);

    // First element
    // BUG 2: Using indexing [0] instead of .first() or .get()
    // This panics if vector is empty
    let first = numbers[0];
    println!("First: Some({})", first);  // Should use Option pattern

    // Last element
    let last = numbers.get(numbers.len() - 1);
    println!("Last: {:?}", last);

    // Element at index 2
    let at_two = numbers.get(2);
    println!("At index 2: {:?}", at_two);

    // Out of bounds
    let at_ten = numbers.get(10);
    println!("At index 10: {:?}", at_ten);

    // Find element > 25
    // BUG 3: Result of find() is not being used correctly
    let found = numbers.iter().find(|&&n| n > 25);
    // Can't print this directly - needs proper Option handling
    match found {
        Some(n) => println!("Find > 25: Some({})", n),
        None => println!("Find > 25: None"),
    }
}

fn validation() {
    fn validate_user(age: u32, name: &str) -> Result<(), String> {
        if age < 18 {
            return Err("age too young".to_string());
        }
        if age > 120 {
            return Err("age out of range".to_string());
        }
        if name.len() < 2 {
            return Err("name too short".to_string());
        }
        Ok(())
    }

    let test_cases = vec![
        (12, "Alice"),
        (150, "Alice"),
        (25, "A"),
        (25, "Bob"),
    ];

    println!("Validation results:");
    for (age, name) in test_cases {
        match validate_user(age, name) {
            Ok(()) => println!("Validate ({}, \"{}\"): Ok, valid age and name", age, name),
            Err(e) => println!("Validate ({}, \"{}\"): Err: {}", age, name, e),
        }
    }
}

// BUGS SUMMARY:
// 1. Using unwrap() on parse result will panic on invalid input
//    Should use match or if let to handle both Ok and Err cases
// 2. Using direct indexing [0] instead of .first() or .get()
//    Direct indexing panics on empty or out-of-bounds access
// 3. Not handling Option return from find() correctly
//    Should match on the Option or use map/unwrap_or

// EXPECTED BEHAVIOR:
// === Test 1: Number Parsing ===
// Parsing strings:
// Success: [42, 87, -15]
// Failed: ["abc", "xyz"], total failures: 2
//
// === Test 2: Collection Access ===
// Vector: [10, 20, 30, 40, 50]
// First: Some(10)
// Last: Some(50)
// At index 2: Some(30)
// At index 10: None
// Find > 25: Some(30)
//
// === Test 3: Validation ===
// Validation results:
// Validate (12, "Alice"): Err: age too young
// Validate (150, "Alice"): Err: age out of range
// Validate (25, "A"): Err: name too short
// Validate (25, "Bob"): Ok, valid age and name
