// Exercise 1: Write Failing Tests Then Fix Code (BROKEN CODE)
//
// This code has bugs in 4 functions. The tests below will fail.
// Your job: Fix the functions (NOT the tests) to make all tests pass.

// BUG 1: This function doesn't handle case-insensitivity or non-alphanumeric chars
pub fn is_palindrome(s: &str) -> bool {
    // Current implementation: simple reversal check
    // Missing: lowercase conversion, filtering non-alphanumeric
    let reversed: String = s.chars().rev().collect();
    s == reversed
}

// BUG 2: Base case is wrong and recursion has off-by-one error
pub fn fibonacci(n: u32) -> u32 {
    // Wrong: F(0) should be 0, F(1) should be 1
    // Current code returns 1 for both F(0) and F(1)
    if n <= 1 {
        return 1;  // Should return n (0 for n=0, 1 for n=1)
    }
    // The recursion itself is correct
    fibonacci(n - 1) + fibonacci(n - 2)
}

// BUG 3: Empty slice handling is wrong, and max logic has error
pub fn find_max(slice: &[i32]) -> Option<i32> {
    // Wrong: Returns Some(0) for empty slice instead of None
    if slice.is_empty() {
        return Some(0);  // Should return None
    }

    // Wrong: Starting with 0 fails for all-negative slices
    let mut max = 0;  // Should start with slice[0]
    for &val in slice {
        if val > max {
            max = val;
        }
    }
    Some(max)
}

// BUG 4: Missing uppercase vowels and using wrong comparison
pub fn count_vowels(s: &str) -> usize {
    let vowels = "aeiou";  // Missing uppercase: "aeiouAEIOU"
    let mut count = 0;
    for c in s.chars() {
        // Wrong: checking if c is in vowels string incorrectly
        if vowels.contains(c) {
            count += 1;
        }
    }
    count
    // Alternative bug: could return count - 1 which would be off by one
}

// =============================================================================
// TESTS - DO NOT MODIFY THESE TESTS
// Your job is to fix the functions above to make these tests pass
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for is_palindrome
    #[test]
    fn test_is_palindrome_simple() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("madam"));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_is_palindrome_with_spaces() {
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(is_palindrome("Was it a car or a cat I saw"));
    }

    #[test]
    fn test_is_palindrome_mixed_case() {
        assert!(is_palindrome("RaceCar"));
        assert!(is_palindrome("Madam"));
    }

    // Tests for fibonacci
    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn test_fibonacci_sequence() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
    }

    #[test]
    fn test_fibonacci_larger() {
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(15), 610);
    }

    // Tests for find_max
    #[test]
    fn test_find_max_normal() {
        assert_eq!(find_max(&[1, 5, 3, 9, 2]), Some(9));
        assert_eq!(find_max(&[100]), Some(100));
    }

    #[test]
    fn test_find_max_negative() {
        assert_eq!(find_max(&[-5, -2, -10, -1]), Some(-1));
        assert_eq!(find_max(&[-100, -50]), Some(-50));
    }

    #[test]
    fn test_find_max_empty() {
        assert_eq!(find_max(&[]), None);
    }

    // Tests for count_vowels
    #[test]
    fn test_count_vowels_simple() {
        assert_eq!(count_vowels("hello"), 2);
        assert_eq!(count_vowels("aeiou"), 5);
    }

    #[test]
    fn test_count_vowels_mixed_case() {
        assert_eq!(count_vowels("HELLO"), 2);
        assert_eq!(count_vowels("HeLLo WoRLd"), 3);
    }

    #[test]
    fn test_count_vowels_no_vowels() {
        assert_eq!(count_vowels("xyz"), 0);
        assert_eq!(count_vowels("rhythm"), 0);
    }
}

// BUGS SUMMARY:
// 1. is_palindrome: Not case-insensitive, doesn't filter non-alphanumeric
//    Fix: Convert to lowercase, filter to only alphanumeric before comparing
// 2. fibonacci: Returns 1 for n=0, should return 0
//    Fix: Change `return 1` to `return n`
// 3. find_max: Returns Some(0) for empty, and starts max at 0
//    Fix: Return None for empty, start max at first element
// 4. count_vowels: Only checks lowercase vowels
//    Fix: Either add uppercase to vowels string or convert input to lowercase

// To run tests:
// cargo test
//
// Expected: All 12 tests should pass after fixes
