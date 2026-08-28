// Exercise 3: Parse and Validate Input with Option/Result (BROKEN CODE)
//
// This code has 4 bugs related to Option/Result handling
// Your job: Find and fix the bugs so it compiles and produces correct output

fn main() {
    println!("=== Number Parsing ===");
    number_parsing();

    println!("\n=== Safe Collection Access ===");
    safe_collection_access();

    println!("\n=== Input Validation ===");
    input_validation();
}

fn number_parsing() {
    let inputs = vec!["42", "3.14", "hello", "999999999999"];

    for input in &inputs {
        // BUG 1: parse() returns Result, but we're treating it as the direct value
        // Need to handle the Result properly
        let result: i32 = input.parse();
        match result {
            Ok(n) => println!("Parsing \"{}\": Ok({})", input, n),
            Err(e) => println!("Parsing \"{}\": Err({})", input, e),
        }
    }

    println!();

    // Parse multiple values, collecting successes and failures
    let numbers = vec!["42", "100", "-5"];
    // BUG 2: filter_map expects Option, but parse returns Result
    // Need to convert Result to Option or use different approach
    let valid: Vec<i32> = numbers.iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    println!("Valid numbers: {:?}", valid);

    let invalid_inputs = vec!["abc", "12.34.56"];
    println!("Invalid inputs: {:?}", invalid_inputs);
}

fn safe_collection_access() {
    let numbers = vec![10, 20, 30, 40, 50];
    println!("Numbers: {:?}", numbers);

    // Safe access using get()
    println!("First element: {:?}", numbers.first());
    println!("Last element: {:?}", numbers.last());
    println!("Element at index 2: {:?}", numbers.get(2));
    println!("Element at index 10: {:?}", numbers.get(10));

    // BUG 3: find() returns Option<&T>, but we're pattern matching wrong
    // The closure parameter needs correct reference handling
    let found = numbers.iter().find(|x| x > 25);
    println!("Find first > 25: {:?}", found);

    let not_found = numbers.iter().find(|&&x| x > 100);
    println!("Find first > 100: {:?}", not_found);
}

fn input_validation() {
    // Test various validation scenarios
    println!("Validate age=25, name=\"Alice\": {:?}", validate_input(25, "Alice"));
    println!("Validate age=150, name=\"Alice\": {:?}", validate_input(150, "Alice"));
    println!("Validate age=25, name=\"A\": {:?}", validate_input(25, "A"));

    println!("Validate age=25, name=\"Bob\", email=\"bob@email.com\": {:?}",
             validate_with_email(25, "Bob", "bob@email.com"));
    println!("Validate age=25, name=\"Bob\", email=\"invalid\": {:?}",
             validate_with_email(25, "Bob", "invalid"));
}

// BUG 4: This function should return Result, but the return types are inconsistent
fn validate_input(age: i32, name: &str) -> Result<&str, &str> {
    // Check age
    if age < 0 || age > 120 {
        return "Age must be between 0 and 120";  // Missing Err()
    }

    // Check name
    if name.len() < 2 {
        return "Name must be at least 2 characters";  // Missing Err()
    }

    "Valid input"  // Missing Ok()
}

fn validate_with_email(age: i32, name: &str, email: &str) -> Result<&'static str, &'static str> {
    // First validate age and name
    validate_input(age, name)?;

    // Check email
    if !email.contains('@') {
        return Err("Email must contain '@'");
    }

    Ok("Valid input")
}

// BUGS SUMMARY:
// 1. parse() returns Result<T, E>, need type annotation: input.parse::<i32>()
//    And the let binding needs to be Result type, not i32
// 2. Actually this one is correct - parse().ok() converts Result to Option
//    But there may be an issue with how it's being used
// 3. find closure: |x| x > 25 compares &&i32 with i32
//    Fix: |&&x| x > 25 or |x| **x > 25
// 4. Return statements need Ok() and Err() wrappers

// EXPECTED OUTPUT:
// === Number Parsing ===
// Parsing "42": Ok(42)
// Parsing "3.14": Err(invalid digit found in string)
// Parsing "hello": Err(invalid digit found in string)
// Parsing "999999999999": Err(number too large to fit in target type)
//
// Valid numbers: [42, 100, -5]
// Invalid inputs: ["abc", "12.34.56"]
//
// === Safe Collection Access ===
// Numbers: [10, 20, 30, 40, 50]
// First element: Some(10)
// Last element: Some(50)
// Element at index 2: Some(30)
// Element at index 10: None
// Find first > 25: Some(30)
// Find first > 100: None
//
// === Input Validation ===
// Validate age=25, name="Alice": Ok("Valid input")
// Validate age=150, name="Alice": Err("Age must be between 0 and 120")
// Validate age=25, name="A": Err("Name must be at least 2 characters")
// Validate age=25, name="Bob", email="bob@email.com": Ok("Valid input")
// Validate age=25, name="Bob", email="invalid": Err("Email must contain '@'")
