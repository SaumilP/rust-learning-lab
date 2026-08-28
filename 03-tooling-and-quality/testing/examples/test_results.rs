// Example: Result<T, E> in Tests
//
// Demonstrates:
// - Using Result return type in test functions
// - The ? operator in tests
// - Testing fallible operations
// - Combining Results with assertions

fn main() {
    println!("This file demonstrates using Result<T, E> in tests.");
    println!("Run tests with: rustc --test test_results.rs && ./test_results");
    println!();

    // Demo the functions
    println!("=== Demo ===");
    println!("parse_number(\"42\") = {:?}", parse_number("42"));
    println!("parse_number(\"invalid\") = {:?}", parse_number("invalid"));
    println!("divide(10.0, 2.0) = {:?}", divide(10.0, 2.0));
    println!("divide(10.0, 0.0) = {:?}", divide(10.0, 0.0));
}

// --- Functions that return Result ---

/// Parses a string into an i32.
pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.trim().parse()
}

/// Divides two numbers.
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

/// Validates and processes user input.
pub fn process_input(input: &str) -> Result<i32, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(String::from("Input cannot be empty"));
    }

    let number: i32 = trimmed
        .parse()
        .map_err(|_| format!("Invalid number: '{}'", trimmed))?;

    if number < 0 {
        return Err(String::from("Number must be non-negative"));
    }

    Ok(number * 2)
}

/// Reads configuration (simulated).
pub fn read_config(key: &str) -> Result<String, String> {
    match key {
        "database_url" => Ok(String::from("postgres://localhost/db")),
        "port" => Ok(String::from("8080")),
        _ => Err(format!("Unknown config key: {}", key)),
    }
}

/// Combines multiple operations.
pub fn complex_operation(input: &str) -> Result<String, String> {
    let number = parse_number(input).map_err(|e| format!("Parse error: {}", e))?;

    let doubled = number * 2;

    if doubled > 1000 {
        return Err(String::from("Result too large"));
    }

    Ok(format!("Result: {}", doubled))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==========================================
    // Basic Test with Result Return Type
    // ==========================================

    // Tests can return Result<(), E>
    // The test passes if Ok(()) is returned
    // The test fails if Err is returned

    #[test]
    fn test_parse_valid_number() -> Result<(), std::num::ParseIntError> {
        let result = parse_number("42")?; // ? propagates errors
        assert_eq!(result, 42);
        Ok(())
    }

    #[test]
    fn test_parse_with_whitespace() -> Result<(), std::num::ParseIntError> {
        let result = parse_number("  123  ")?;
        assert_eq!(result, 123);
        Ok(())
    }

    #[test]
    fn test_parse_negative() -> Result<(), std::num::ParseIntError> {
        let result = parse_number("-42")?;
        assert_eq!(result, -42);
        Ok(())
    }

    // ==========================================
    // Tests with String Error Type
    // ==========================================

    #[test]
    fn test_divide_valid() -> Result<(), String> {
        let result = divide(10.0, 2.0)?;
        assert_eq!(result, 5.0);
        Ok(())
    }

    #[test]
    fn test_process_input_valid() -> Result<(), String> {
        let result = process_input("21")?;
        assert_eq!(result, 42);
        Ok(())
    }

    #[test]
    fn test_read_config() -> Result<(), String> {
        let port = read_config("port")?;
        assert_eq!(port, "8080");
        Ok(())
    }

    // ==========================================
    // Using Box<dyn Error> for Mixed Error Types
    // ==========================================

    #[test]
    fn test_mixed_errors() -> Result<(), Box<dyn std::error::Error>> {
        // Can use ? with different error types
        let number: i32 = "42".parse()?; // ParseIntError
        assert_eq!(number, 42);
        Ok(())
    }

    #[test]
    fn test_complex_chain() -> Result<(), Box<dyn std::error::Error>> {
        let input = "21";
        let number: i32 = input.parse()?;
        let result = number * 2;
        assert_eq!(result, 42);
        Ok(())
    }

    // ==========================================
    // Testing Error Cases
    // ==========================================

    // For testing that errors occur, don't use Result return type
    // Use pattern matching or is_err() instead

    #[test]
    fn test_parse_invalid_returns_error() {
        let result = parse_number("not a number");
        assert!(result.is_err());
    }

    #[test]
    fn test_divide_by_zero_returns_error() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }

    #[test]
    fn test_process_empty_input_error() {
        let result = process_input("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_process_negative_error() {
        let result = process_input("-5");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("non-negative"));
    }

    #[test]
    fn test_unknown_config_key() {
        let result = read_config("unknown_key");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown config key"));
    }

    // ==========================================
    // Combining Result Tests with Match
    // ==========================================

    #[test]
    fn test_with_pattern_matching() {
        match parse_number("42") {
            Ok(n) => assert_eq!(n, 42),
            Err(e) => panic!("Expected Ok, got Err: {}", e),
        }
    }

    #[test]
    fn test_error_with_pattern_matching() {
        match parse_number("invalid") {
            Ok(n) => panic!("Expected Err, got Ok: {}", n),
            Err(_) => (), // Test passes
        }
    }

    // ==========================================
    // Multiple Operations with ?
    // ==========================================

    #[test]
    fn test_chained_operations() -> Result<(), String> {
        // Chain multiple fallible operations
        let a = divide(100.0, 5.0)?; // 20.0
        let b = divide(a, 2.0)?; // 10.0
        let c = divide(b, 2.5)?; // 4.0

        assert_eq!(c, 4.0);
        Ok(())
    }

    #[test]
    fn test_complex_operation_success() -> Result<(), String> {
        let result = complex_operation("100")?;
        assert_eq!(result, "Result: 200");
        Ok(())
    }

    // ==========================================
    // Helper Functions in Tests
    // ==========================================

    fn setup_test_data() -> Result<Vec<i32>, String> {
        let data = vec!["1", "2", "3", "4", "5"];
        data.iter()
            .map(|s| parse_number(s).map_err(|e| e.to_string()))
            .collect()
    }

    #[test]
    fn test_with_setup() -> Result<(), String> {
        let numbers = setup_test_data()?;
        assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
        assert_eq!(numbers.iter().sum::<i32>(), 15);
        Ok(())
    }

    // ==========================================
    // Custom Test Helper
    // ==========================================

    fn assert_error_contains<T>(result: Result<T, String>, expected: &str) {
        match result {
            Ok(_) => panic!("Expected error containing '{}', but got Ok", expected),
            Err(e) => {
                assert!(
                    e.contains(expected),
                    "Error '{}' should contain '{}'",
                    e,
                    expected
                );
            }
        }
    }

    #[test]
    fn test_using_custom_helper() {
        assert_error_contains(process_input(""), "empty");
        assert_error_contains(process_input("-1"), "non-negative");
        assert_error_contains(process_input("abc"), "Invalid number");
    }

    // ==========================================
    // Real-World Example
    // ==========================================

    fn parse_config_line(line: &str) -> Result<(String, String), String> {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid config line: '{}'", line));
        }

        let key = parts[0].trim().to_string();
        let value = parts[1].trim().to_string();

        if key.is_empty() {
            return Err(String::from("Config key cannot be empty"));
        }

        Ok((key, value))
    }

    #[test]
    fn test_parse_config_line() -> Result<(), String> {
        let (key, value) = parse_config_line("host = localhost")?;
        assert_eq!(key, "host");
        assert_eq!(value, "localhost");
        Ok(())
    }

    #[test]
    fn test_parse_config_line_errors() {
        assert!(parse_config_line("invalid").is_err());
        assert!(parse_config_line("= value").is_err());
    }

    #[test]
    fn test_multiple_config_lines() -> Result<(), String> {
        let lines = vec!["host = localhost", "port = 8080", "debug = true"];

        let mut config = std::collections::HashMap::new();
        for line in lines {
            let (key, value) = parse_config_line(line)?;
            config.insert(key, value);
        }

        assert_eq!(config.get("host"), Some(&String::from("localhost")));
        assert_eq!(config.get("port"), Some(&String::from("8080")));
        assert_eq!(config.get("debug"), Some(&String::from("true")));
        Ok(())
    }
}
