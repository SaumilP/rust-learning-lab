// Example: Basic Unit Testing with #[test]
//
// Demonstrates:
// - Writing tests with #[test] attribute
// - Running tests with cargo test
// - Test functions that pass and fail
// - Basic test structure
//
// Run with: rustc --test unit_test_basics.rs && ./unit_test_basics
// Or: cargo test (if in a cargo project)

fn main() {
    println!("This file contains unit tests.");
    println!("Run with: rustc --test unit_test_basics.rs && ./unit_test_basics");
    println!("Or use: cargo test");
    println!();

    // Demo the functions being tested
    println!("=== Demonstrating Functions ===\n");
    println!("add(2, 3) = {}", add(2, 3));
    println!("multiply(4, 5) = {}", multiply(4, 5));
    println!("is_even(4) = {}", is_even(4));
    println!("is_even(7) = {}", is_even(7));
    println!("greet(\"World\") = \"{}\"", greet("World"));
}

// --- Functions to be tested ---

/// Adds two numbers together.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers.
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Returns true if the number is even.
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Returns a greeting string.
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Divides two numbers, panics on division by zero.
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Division by zero!");
    }
    a / b
}

// --- Test Module ---
// Tests are placed in a module with #[cfg(test)]
// This means tests are only compiled when running `cargo test`

#[cfg(test)]
mod tests {
    // Import functions from parent module
    use super::*;

    // --- Basic Test ---
    // A test function is marked with #[test]
    // It passes if it doesn't panic

    #[test]
    fn test_add() {
        // This test will pass
        let result = add(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_add_negative() {
        // Test with negative numbers
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(-5, -3), -8);
    }

    // --- Multiple Assertions ---
    // A test can have multiple assertions

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(0, 100), 0);
        assert_eq!(multiply(-2, 5), -10);
    }

    // --- Testing Boolean Functions ---

    #[test]
    fn test_is_even() {
        assert!(is_even(0)); // 0 is even
        assert!(is_even(2)); // 2 is even
        assert!(is_even(100)); // 100 is even
        assert!(!is_even(1)); // 1 is not even
        assert!(!is_even(99)); // 99 is not even
    }

    // --- Testing String Output ---

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn test_greet_empty() {
        assert_eq!(greet(""), "Hello, !");
    }

    // --- Testing for Panic ---
    // Use #[should_panic] to test that code panics

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        divide(10, 0); // This should panic
    }

    // More specific: check the panic message
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_divide_by_zero_message() {
        divide(10, 0);
    }

    // --- Normal Division Test ---

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5);
        assert_eq!(divide(100, 10), 10);
    }

    // --- Ignored Tests ---
    // Use #[ignore] for tests that are slow or require special setup

    #[test]
    #[ignore]
    fn test_slow_operation() {
        // This test is ignored by default
        // Run with: cargo test -- --ignored
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(true);
    }

    // --- Test Naming Conventions ---

    // Good: descriptive names showing what is being tested
    #[test]
    fn add_returns_sum_of_two_numbers() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn multiply_with_zero_returns_zero() {
        assert_eq!(multiply(0, 999), 0);
    }

    #[test]
    fn is_even_returns_false_for_odd_numbers() {
        assert!(!is_even(3));
        assert!(!is_even(7));
    }

    // --- Test with Setup ---

    #[test]
    fn test_with_setup() {
        // Setup
        let numbers = vec![1, 2, 3, 4, 5];

        // Test
        let sum: i32 = numbers.iter().sum();

        // Assert
        assert_eq!(sum, 15);
    }

    // --- Test Private Functions ---
    // Tests in the same module can access private functions

    fn helper_function(x: i32) -> i32 {
        x * 2
    }

    #[test]
    fn test_helper() {
        assert_eq!(helper_function(5), 10);
    }
}
