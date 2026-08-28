# Exercise 1: Write Failing Tests Then Fix Code

## Difficulty: Medium
## Concepts Tested: Testing, Test modules, Assertions
## Prerequisites: Module 03 - Testing basics

---

## Problem Statement

You are given a Rust module with several utility functions that contain bugs. Test cases are provided that currently fail. Your task is to debug and fix the functions so all tests pass. This exercise teaches test-driven debugging - using tests to identify and fix issues.

## Requirements

- Understand the provided failing tests
- Analyze why each test fails
- Fix the bugs in the implementation functions
- Ensure all tests pass without modifying the tests themselves
- Handle both normal and edge cases

## Functions to Fix

### Function 1: `is_palindrome(s: &str) -> bool`
- Should return true if string is a palindrome (same forwards and backwards)
- Should be case-insensitive
- Should ignore non-alphanumeric characters

### Function 2: `fibonacci(n: u32) -> u32`
- Should return the nth Fibonacci number
- F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2)

### Function 3: `find_max(slice: &[i32]) -> Option<i32>`
- Should return the maximum value in a slice
- Should return None for empty slices

### Function 4: `count_vowels(s: &str) -> usize`
- Should count all vowels (a, e, i, o, u) in a string
- Should be case-insensitive

## Expected Test Output

```
running 12 tests
test tests::test_is_palindrome_simple ... ok
test tests::test_is_palindrome_with_spaces ... ok
test tests::test_is_palindrome_mixed_case ... ok
test tests::test_fibonacci_base_cases ... ok
test tests::test_fibonacci_sequence ... ok
test tests::test_fibonacci_larger ... ok
test tests::test_find_max_normal ... ok
test tests::test_find_max_negative ... ok
test tests::test_find_max_empty ... ok
test tests::test_count_vowels_simple ... ok
test tests::test_count_vowels_mixed_case ... ok
test tests::test_count_vowels_no_vowels ... ok

test result: ok. 12 passed; 0 failed; 0 ignored
```

## Notes

- Run tests with `cargo test`
- Use `cargo test -- --nocapture` to see println output
- The test assertions tell you what the expected values should be
- Read test names carefully - they hint at what's being tested
- Fix the implementation, not the tests!
