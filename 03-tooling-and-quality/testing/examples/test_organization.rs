// Example: Test Organization and Module Structure
//
// Demonstrates:
// - Organizing tests in modules
// - Separate test modules for different functionality
// - Testing internal vs public APIs
// - Common test utilities

fn main() {
    println!("This file demonstrates test organization.");
    println!("Run tests with: rustc --test test_organization.rs && ./test_organization");
    println!();

    // Demo the public API
    println!("=== Calculator Demo ===");
    let calc = Calculator::new();
    println!("Calculator.add(5, 3) = {}", calc.add(5, 3));
    println!("Calculator.subtract(10, 4) = {}", calc.subtract(10, 4));
    println!("Calculator.multiply(6, 7) = {}", calc.multiply(6, 7));
    println!("Calculator.divide(20, 4) = {:?}", calc.divide(20, 4));
    println!("is_positive(7) = {}", validation::is_positive(7));
    println!(
        "is_in_range(7, 1, 10) = {}",
        validation::is_in_range(7, 1, 10)
    );
    println!(
        "validate_input(\"42\") = {:?}",
        validation::validate_input("42")
    );
}

// --- Public API ---

/// A simple calculator struct with basic operations.
pub struct Calculator {
    precision: u32,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator { precision: 2 }
    }

    pub fn with_precision(precision: u32) -> Self {
        Calculator { precision }
    }

    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn divide(&self, a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }

    // Private method for internal calculations
    fn round(&self, value: f64) -> f64 {
        let factor = 10_f64.powi(self.precision as i32);
        (value * factor).round() / factor
    }

    pub fn average(&self, numbers: &[f64]) -> Option<f64> {
        if numbers.is_empty() {
            return None;
        }
        let sum: f64 = numbers.iter().sum();
        Some(self.round(sum / numbers.len() as f64))
    }
}

// --- Validation Module ---

mod validation {
    pub fn is_positive(n: i32) -> bool {
        n > 0
    }

    pub fn is_in_range(n: i32, min: i32, max: i32) -> bool {
        n >= min && n <= max
    }

    pub fn validate_input(input: &str) -> Result<i32, String> {
        input
            .trim()
            .parse()
            .map_err(|_| format!("Invalid input: '{}'", input))
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    // --- Test Utilities ---
    // Create helper functions for common test setup

    fn create_test_calculator() -> Calculator {
        Calculator::new()
    }

    fn sample_numbers() -> Vec<f64> {
        vec![1.0, 2.0, 3.0, 4.0, 5.0]
    }

    // --- Calculator Tests - Basic Operations ---

    mod calculator_basic_tests {
        use super::*;

        #[test]
        fn new_creates_calculator_with_default_precision() {
            let calc = Calculator::new();
            assert_eq!(calc.precision, 2);
        }

        #[test]
        fn with_precision_sets_custom_precision() {
            let calc = Calculator::with_precision(4);
            assert_eq!(calc.precision, 4);
        }
    }

    // --- Calculator Tests - Arithmetic ---

    mod calculator_arithmetic_tests {
        use super::*;

        #[test]
        fn add_returns_correct_sum() {
            let calc = create_test_calculator();
            assert_eq!(calc.add(2, 3), 5);
            assert_eq!(calc.add(-1, 1), 0);
            assert_eq!(calc.add(0, 0), 0);
        }

        #[test]
        fn subtract_returns_correct_difference() {
            let calc = create_test_calculator();
            assert_eq!(calc.subtract(5, 3), 2);
            assert_eq!(calc.subtract(3, 5), -2);
        }

        #[test]
        fn multiply_returns_correct_product() {
            let calc = create_test_calculator();
            assert_eq!(calc.multiply(4, 5), 20);
            assert_eq!(calc.multiply(-2, 3), -6);
            assert_eq!(calc.multiply(0, 100), 0);
        }

        #[test]
        fn divide_returns_some_for_valid_division() {
            let calc = create_test_calculator();
            assert_eq!(calc.divide(10, 2), Some(5));
            assert_eq!(calc.divide(7, 2), Some(3)); // Integer division
        }

        #[test]
        fn divide_returns_none_for_division_by_zero() {
            let calc = create_test_calculator();
            assert_eq!(calc.divide(10, 0), None);
        }
    }

    // --- Calculator Tests - Advanced Operations ---

    mod calculator_advanced_tests {
        use super::*;

        #[test]
        fn average_returns_correct_average() {
            let calc = Calculator::new();
            let numbers = sample_numbers();
            assert_eq!(calc.average(&numbers), Some(3.0));
        }

        #[test]
        fn average_returns_none_for_empty_slice() {
            let calc = Calculator::new();
            assert_eq!(calc.average(&[]), None);
        }

        #[test]
        fn average_rounds_to_precision() {
            let calc = Calculator::with_precision(1);
            let numbers = vec![1.0, 2.0, 3.5];
            // Average is 2.1666..., should round to 2.2 with precision 1
            assert_eq!(calc.average(&numbers), Some(2.2));
        }
    }

    // --- Validation Module Tests ---

    mod validation_tests {
        use super::validation::*;

        #[test]
        fn is_positive_returns_true_for_positive_numbers() {
            assert!(is_positive(1));
            assert!(is_positive(100));
        }

        #[test]
        fn is_positive_returns_false_for_zero_and_negative() {
            assert!(!is_positive(0));
            assert!(!is_positive(-1));
        }

        #[test]
        fn is_in_range_returns_true_when_in_range() {
            assert!(is_in_range(5, 0, 10));
            assert!(is_in_range(0, 0, 10)); // Boundary
            assert!(is_in_range(10, 0, 10)); // Boundary
        }

        #[test]
        fn is_in_range_returns_false_when_out_of_range() {
            assert!(!is_in_range(-1, 0, 10));
            assert!(!is_in_range(11, 0, 10));
        }

        #[test]
        fn validate_input_parses_valid_number() {
            assert_eq!(validate_input("42"), Ok(42));
            assert_eq!(validate_input("  123  "), Ok(123));
            assert_eq!(validate_input("-5"), Ok(-5));
        }

        #[test]
        fn validate_input_returns_error_for_invalid_input() {
            let result = validate_input("not a number");
            assert!(result.is_err());
            assert!(result.unwrap_err().contains("Invalid input"));
        }
    }

    // --- Integration Tests ---
    // Tests that exercise multiple components together

    mod integration_tests {
        use super::*;

        #[test]
        fn calculator_chain_operations() {
            let calc = create_test_calculator();

            // (5 + 3) * 2 - 4 = 12
            let step1 = calc.add(5, 3);
            let step2 = calc.multiply(step1, 2);
            let result = calc.subtract(step2, 4);

            assert_eq!(result, 12);
        }

        #[test]
        fn validation_with_calculator() {
            let calc = create_test_calculator();

            // Validate input then calculate
            let input = "10";
            if let Ok(n) = validation::validate_input(input) {
                let result = calc.multiply(n, 2);
                assert_eq!(result, 20);
            }
        }
    }

    // --- Edge Case Tests ---

    mod edge_case_tests {
        use super::*;

        #[test]
        fn calculator_handles_large_numbers() {
            let calc = create_test_calculator();
            assert_eq!(calc.add(i32::MAX - 1, 1), i32::MAX);
        }

        #[test]
        fn calculator_handles_negative_results() {
            let calc = create_test_calculator();
            assert_eq!(calc.subtract(0, 100), -100);
        }

        #[test]
        fn average_handles_single_element() {
            let calc = Calculator::new();
            assert_eq!(calc.average(&[42.0]), Some(42.0));
        }
    }
}
