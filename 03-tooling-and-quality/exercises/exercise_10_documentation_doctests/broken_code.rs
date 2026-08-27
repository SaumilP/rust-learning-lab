//! # Mathematical Utilities Library
//!
//! This library provides common mathematical functions with comprehensive documentation.
//!
//! ## Functions
//!
//! - `gcd` - Calculate greatest common divisor
//! - `is_valid_email` - Validate email format
//! - `calculate_average` - Calculate average of numbers

// BUG 1: Missing documentation and doc tests for this function
// Should have:
// - Description of what function does
// - Example code block with ///
// - Doc test showing usage
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Checks if a string is a valid email format.
///
/// This is a simple validation that checks for @ symbol and domain.
/// Not suitable for production use - real email validation is complex.
///
/// # Examples
///
/// ```
/// let email = "user@example.com";
/// assert!(is_valid_email(email));
/// ```
///
/// BUG 2: Doc test above is incomplete - doesn't show what module it's from
/// Should be: use exercise_10::is_valid_email;
pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

/// Calculate the average of a slice of numbers.
///
/// Returns `None` if the slice is empty.
///
/// # Arguments
///
/// * `numbers` - A slice of floating point numbers
///
/// # Returns
///
/// `Some(average)` if the slice is not empty, `None` otherwise
///
/// # Examples
///
/// ```
/// let nums = vec![1.0, 2.0, 3.0, 4.0];
/// let avg = calculate_average(&nums);
/// // BUG 3: Doc test doesn't match reality
/// // This assertion is wrong - average of [1,2,3,4] is 2.5, not 2.0
/// assert_eq!(avg, Some(2.5));
/// ```
pub fn calculate_average(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        None
    } else {
        Some(numbers.iter().sum::<f64>() / numbers.len() as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(100, 50), 50);
    }

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("user@example.com"));
        assert!(!is_valid_email("notanemail"));
    }

    #[test]
    fn test_average() {
        assert_eq!(calculate_average(&[1.0, 2.0, 3.0]), Some(2.0));
        assert_eq!(calculate_average(&[]), None);
    }
}

// BUGS SUMMARY:
// 1. gcd() function has no documentation - should have doc comments with ///, examples, etc.
// 2. is_valid_email() doc test is incomplete - needs proper module reference
// 3. calculate_average() doc test has wrong expected value - should be 2.5 for [1,2,3,4], not 2.0

// Note: In actual code, you'd need proper module setup for doc tests to reference functions

// EXPECTED BEHAVIOR:
// cargo test --doc should show:
// - Doc test for is_valid_email passes
// - Doc test for calculate_average passes
// - If gcd() had docs, those tests would run too
//
// cargo doc should generate:
// - Module documentation displayed
// - Function documentation with examples
// - Links to functions
