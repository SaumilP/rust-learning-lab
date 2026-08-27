// Exercise 9: Testing Basics (BROKEN CODE)
//
// This code has 3 bugs related to test implementation and assertions
// Your job: Find and fix the bugs so all tests pass

// Function: Check if number is even
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Function: Calculate factorial
pub fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}

// Function: Reverse a string
pub fn string_reverse(s: &str) -> String {
    s.chars().rev().collect()
}

// Function: Find maximum in slice
pub fn find_max(vec: &[i32]) -> Option<i32> {
    if vec.is_empty() {
        None
    } else {
        Some(*vec.iter().max().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(is_even(0));
        assert!(is_even(-2));
    }

    #[test]
    fn test_is_even_negative() {
        // BUG 1: Wrong assertion - should be assert!(), not assert_eq!()
        // assert_eq!() is for equality, not boolean conditions
        assert_eq!(is_even(3), false);  // Works but should use assert! for clarity
    }

    #[test]
    fn test_factorial_zero() {
        // BUG 2: Incorrect assertion value
        // factorial(0) should be 1, not 0
        assert_eq!(factorial(0), 0);  // WRONG - should be 1
    }

    #[test]
    fn test_factorial_positive() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(3), 6);
    }

    #[test]
    fn test_string_reverse_normal() {
        assert_eq!(string_reverse("hello"), "olleh");
        assert_eq!(string_reverse("Rust"), "tsuR");
    }

    #[test]
    fn test_string_reverse_empty() {
        assert_eq!(string_reverse(""), "");
    }

    #[test]
    fn test_find_max_normal() {
        assert_eq!(find_max(&[3, 1, 4, 1, 5]), Some(5));
        assert_eq!(find_max(&[-5, -2, -10]), Some(-2));
    }

    #[test]
    fn test_find_max_empty() {
        // BUG 3: Comparing Option with wrong value
        // find_max returns Option<i32>, comparing with integer
        assert_eq!(find_max(&[]), 0);  // WRONG - should be None, not 0
    }

    // Additional edge case test (missing)
    #[test]
    fn test_is_even_single_odd() {
        assert!(!is_even(1));
    }
}

// BUGS SUMMARY:
// 1. Using assert_eq!() for boolean check - should use assert!() macro
//    assert_eq!(is_even(3), false) works but assert!(!is_even(3)) is better
// 2. Wrong expected value - factorial(0) = 1, not 0
// 3. Comparing Option with primitive - should compare with None, not 0
//    assert_eq!(find_max(&[]), None) instead of assert_eq!(find_max(&[]), 0)

// EXPECTED TEST OUTPUT:
// running 8 tests
// test tests::test_factorial_positive ... ok
// test tests::test_factorial_zero ... ok
// test tests::test_find_max_empty ... ok
// test tests::test_find_max_normal ... ok
// test tests::test_is_even ... ok
// test tests::test_is_even_negative ... ok
// test tests::test_is_even_single_odd ... ok
// test tests::test_string_reverse_empty ... ok
// test tests::test_string_reverse_normal ... ok
//
// test result: ok. 8 passed; 0 failed; 0 ignored
