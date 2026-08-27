# Exercise 9: Testing Basics

## Difficulty: Medium
## Concepts Tested: Unit Tests, Test Attributes, Assertions, Test Organization
## Prerequisites: Module 03 - Testing, Module 01-02 Foundations

---

## Problem Statement

Write a Rust library with functions and corresponding unit tests. The program should:

1. Implement several mathematical and utility functions
2. Create comprehensive unit tests for each function
3. Use various assertion macros correctly
4. Organize tests in test modules
5. Demonstrate test attributes and filtering

## Requirements

- Implement at least 4 functions with clear behavior
- Write unit tests for each function
- Use `#[test]` attribute correctly
- Use `#[cfg(test)]` for test modules
- Implement positive and negative test cases
- Test edge cases and boundary conditions
- Use appropriate assertion macros
- Show test organization and naming conventions

## Functions to Implement and Test

### Function 1: is_even(n: i32) -> bool
- Returns true if n is even
- Returns false if n is odd

### Function 2: factorial(n: u32) -> u32
- Returns factorial of n
- Returns 1 for 0
- Should handle reasonable inputs

### Function 3: string_reverse(s: &str) -> String
- Returns reversed string
- Handles empty strings
- Preserves special characters

### Function 4: find_max(vec: &[i32]) -> Option<i32>
- Returns max value or None for empty
- Works with negative numbers

## Expected Output

### Function Tests
```
test is_even ... ok
test is_even_negative ... ok
test factorial_zero ... ok
test factorial_positive ... ok
test string_reverse_normal ... ok
test string_reverse_empty ... ok
test find_max_normal ... ok
test find_max_empty ... ok

test result: ok. 8 passed
```

## Hints

1. **Hint 1**: Use `#[cfg(test)]` to create test module
2. **Hint 2**: Use `#[test]` attribute on test functions
3. **Hint 3**: Use `assert!()` for boolean conditions
4. **Hint 4**: Use `assert_eq!()` for equality checks
5. **Hint 5**: Use descriptive test names (test_function_case)
6. **Hint 6**: The broken code has missing tests or incorrect assertions

## Testing

Run tests with:
```bash
cargo test
```

Run specific test:
```bash
cargo test is_even
```

Run with output:
```bash
cargo test -- --nocapture
```

## Learning Objectives

After completing this exercise, you should understand:
- Writing unit tests in Rust
- Test attributes and configuration
- Assertion macros and their use
- Test organization and naming
- Organizing tests by function
- Testing edge cases
- Running tests selectively
- Test-driven development basics

