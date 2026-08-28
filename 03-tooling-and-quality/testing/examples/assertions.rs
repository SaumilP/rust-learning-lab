// Example: Assertion Macros - assert!, assert_eq!, assert_ne!
//
// Demonstrates:
// - Different assertion macros and when to use them
// - Custom error messages
// - Debug output with assertions
// - Partial assertions

fn main() {
    println!("This file demonstrates assertion macros in tests.");
    println!("Run tests with: rustc --test assertions.rs && ./assertions");
    println!();

    // Demo some functions
    println!("=== Demo ===");
    println!("add(2, 3) = {}", add(2, 3));
    println!("is_prime(7) = {}", is_prime(7));
    println!("is_prime(10) = {}", is_prime(10));
}

// --- Functions to test ---

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

#[derive(Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================
    // assert! - Basic Boolean Assertions
    // ==========================================

    mod assert_examples {
        use super::*;

        // assert! checks that a condition is true
        #[test]
        fn assert_true_condition() {
            assert!(true);
            assert!(1 + 1 == 2);
            assert!(5 > 3);
        }

        // assert! with boolean functions
        #[test]
        fn assert_with_function() {
            assert!(is_prime(7));
            assert!(is_prime(11));
            assert!(is_prime(2));
        }

        // assert! with negation
        #[test]
        fn assert_false_condition() {
            assert!(!is_prime(4));
            assert!(!is_prime(1));
            assert!(!is_prime(0));
        }

        // assert! with custom message
        #[test]
        fn assert_with_message() {
            let value = 42;
            assert!(value > 0, "Value should be positive, got {}", value);
        }

        // assert! for method results
        #[test]
        fn assert_option_is_some() {
            let result: Option<i32> = Some(42);
            assert!(result.is_some());
        }

        #[test]
        fn assert_result_is_ok() {
            let result: Result<i32, &str> = Ok(42);
            assert!(result.is_ok());
        }
    }

    // ==========================================
    // assert_eq! - Equality Assertions
    // ==========================================

    mod assert_eq_examples {
        use super::*;

        // assert_eq! checks that two values are equal
        #[test]
        fn assert_eq_numbers() {
            assert_eq!(2 + 2, 4);
            assert_eq!(add(5, 3), 8);
        }

        // assert_eq! shows both values on failure
        #[test]
        fn assert_eq_strings() {
            let expected = "hello";
            let actual = "hello";
            assert_eq!(expected, actual);
        }

        // assert_eq! with Option
        #[test]
        fn assert_eq_option() {
            let result: Option<i32> = Some(42);
            assert_eq!(result, Some(42));

            let none: Option<i32> = None;
            assert_eq!(none, None);
        }

        // assert_eq! with Result
        #[test]
        fn assert_eq_result() {
            let result = divide(10.0, 2.0);
            assert_eq!(result, Ok(5.0));
        }

        // assert_eq! with custom structs (requires PartialEq)
        #[test]
        fn assert_eq_structs() {
            let p1 = Point::new(3, 4);
            let p2 = Point::new(3, 4);
            assert_eq!(p1, p2);
        }

        // assert_eq! with collections
        #[test]
        fn assert_eq_vectors() {
            let expected = vec![1, 2, 3];
            let actual = vec![1, 2, 3];
            assert_eq!(expected, actual);
        }

        // assert_eq! with custom message
        #[test]
        fn assert_eq_with_message() {
            let expected = 10;
            let actual = add(7, 3);
            assert_eq!(
                expected, actual,
                "add(7, 3) should equal {}, but got {}",
                expected, actual
            );
        }
    }

    // ==========================================
    // assert_ne! - Inequality Assertions
    // ==========================================

    mod assert_ne_examples {
        use super::*;

        // assert_ne! checks that two values are NOT equal
        #[test]
        fn assert_ne_numbers() {
            assert_ne!(2 + 2, 5);
            assert_ne!(add(1, 1), 3);
        }

        // assert_ne! with strings
        #[test]
        fn assert_ne_strings() {
            assert_ne!("hello", "world");
        }

        // assert_ne! with Option
        #[test]
        fn assert_ne_option() {
            let some_value: Option<i32> = Some(42);
            let none_value: Option<i32> = None;
            assert_ne!(some_value, none_value);
        }

        // assert_ne! with custom types
        #[test]
        fn assert_ne_structs() {
            let p1 = Point::new(0, 0);
            let p2 = Point::new(1, 1);
            assert_ne!(p1, p2);
        }

        // assert_ne! with message
        #[test]
        fn assert_ne_with_message() {
            let a = 5;
            let b = 10;
            assert_ne!(a, b, "Values should be different: {} vs {}", a, b);
        }
    }

    // ==========================================
    // Debug Output
    // ==========================================

    mod debug_output {
        // Debug trait is needed for assertion failure messages
        #[derive(Debug, PartialEq)]
        struct TestData {
            id: u32,
            name: String,
        }

        #[test]
        fn assert_eq_with_debug() {
            let expected = TestData {
                id: 1,
                name: String::from("test"),
            };
            let actual = TestData {
                id: 1,
                name: String::from("test"),
            };
            // If these differ, the Debug output will show the difference
            assert_eq!(expected, actual);
        }

        // Without Debug, you can't use assert_eq!
        // This struct can only use assert! with custom comparison
        struct NoDebug {
            value: i32,
        }

        #[test]
        fn assert_without_debug() {
            let a = NoDebug { value: 42 };
            let b = NoDebug { value: 42 };
            // Can't use: assert_eq!(a, b);
            // Use: assert!(a.value == b.value);
            assert!(a.value == b.value, "Values should be equal");
        }
    }

    // ==========================================
    // Approximate Equality for Floats
    // ==========================================

    mod float_assertions {
        use super::*;

        // Floats should use approximate comparison
        fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
            (a - b).abs() < epsilon
        }

        #[test]
        fn assert_float_approximately_equal() {
            let result = 0.1 + 0.2;
            // Don't use: assert_eq!(result, 0.3); // May fail!
            assert!(approx_eq(result, 0.3, 1e-10));
        }

        #[test]
        fn assert_distance_calculation() {
            let p = Point::new(3, 4);
            let distance = p.distance_from_origin();
            // Distance should be 5.0
            assert!(approx_eq(distance, 5.0, 1e-10));
        }
    }

    // ==========================================
    // Multiple Assertions
    // ==========================================

    mod multiple_assertions {
        #[test]
        fn multiple_assertions_in_one_test() {
            let value = 42;

            // Multiple related assertions
            assert!(value > 0, "Should be positive");
            assert!(value < 100, "Should be less than 100");
            assert_eq!(value % 2, 0, "Should be even");
            assert_ne!(value, 0, "Should not be zero");
        }

        #[test]
        fn assertions_with_setup() {
            // Setup
            let numbers = vec![1, 2, 3, 4, 5];

            // Multiple assertions about the data
            assert!(!numbers.is_empty());
            assert_eq!(numbers.len(), 5);
            assert_eq!(numbers.first(), Some(&1));
            assert_eq!(numbers.last(), Some(&5));
        }
    }

    // ==========================================
    // Assertion in Loops
    // ==========================================

    mod loop_assertions {
        use super::*;

        #[test]
        fn assert_all_elements() {
            let numbers = vec![2, 4, 6, 8, 10];

            for n in &numbers {
                assert!(n % 2 == 0, "All numbers should be even, but found {}", n);
            }
        }

        #[test]
        fn assert_primes() {
            let primes = vec![2, 3, 5, 7, 11, 13];

            for &p in &primes {
                assert!(is_prime(p), "{} should be prime", p);
            }
        }

        #[test]
        fn assert_no_primes() {
            let non_primes = vec![0, 1, 4, 6, 8, 9, 10];

            for &n in &non_primes {
                assert!(!is_prime(n), "{} should NOT be prime", n);
            }
        }
    }

    // ==========================================
    // Conditional Assertions
    // ==========================================

    mod conditional_assertions {
        use super::*;

        #[test]
        fn conditional_test() {
            let result = divide(10.0, 2.0);

            if let Ok(value) = result {
                assert_eq!(value, 5.0);
            } else {
                panic!("Expected Ok, got Err");
            }
        }

        #[test]
        fn pattern_matching_assertion() {
            let result = divide(10.0, 0.0);

            match result {
                Ok(_) => panic!("Expected error for division by zero"),
                Err(msg) => assert!(msg.contains("zero")),
            }
        }
    }
}
