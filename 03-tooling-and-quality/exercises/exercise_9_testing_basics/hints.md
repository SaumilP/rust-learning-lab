# Hints for Exercise 9: Testing Basics

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: Using assert_eq!() for Boolean Conditions**
- The code: `assert_eq!(is_even(3), false)`
- This works because it compares the boolean result with false
- But it's not idiomatic Rust - better to use `assert!()` for boolean checks
- When result should be false, use: `assert!(!is_even(3))`
- This reads more naturally: "assert that NOT is_even(3)"
- Different assertion macros for different purposes:
  - `assert!(condition)` - boolean condition must be true
  - `assert!(!condition)` - boolean condition must be false
  - `assert_eq!(a, b)` - two values must be equal
  - `assert_ne!(a, b)` - two values must not be equal
- Fix: Change to `assert!(!is_even(3));` to be idiomatic

**Bug 2: Wrong Expected Value in Assertion**
- The code: `assert_eq!(factorial(0), 0);`
- By definition of factorial: 0! = 1, not 0
- The function correctly returns 1 for input 0
- The test is asserting the wrong value
- Fix: `assert_eq!(factorial(0), 1);`
- This is a common test bug - incorrect test, not incorrect code

**Bug 3: Comparing Option with Wrong Type**
- The code: `assert_eq!(find_max(&[]), 0);`
- `find_max()` returns `Option<i32>`, not `i32`
- The function correctly returns `None` for empty slice
- Comparing `Option<i32>` with `i32` is a type error OR wrong expected value
- Fix: `assert_eq!(find_max(&[]), None);`
- This correctly compares Option with Option

### Assertion Macro Guide:

```rust
// Use assert!() for boolean results
assert!(is_even(4));        // Value is true
assert!(!is_even(3));       // Value is false (NOT true)

// Use assert_eq!() for value comparisons
assert_eq!(factorial(5), 120);      // Two values equal
assert_eq!(find_max(&[1,2,3]), Some(3));  // Option values

// Use assert_ne!() for inequality
assert_ne!(is_even(4), is_even(5)); // Results differ

// Use debug_assert!() for performance-critical code
debug_assert!(n > 0);  // Only checked in debug builds
```

### Testing Organization Best Practices:

```rust
#[cfg(test)]
mod tests {
    use super::*;  // Import items from parent module

    #[test]
    fn test_function_happy_path() {
        // Normal, expected behavior
    }

    #[test]
    fn test_function_edge_case() {
        // Boundary conditions
    }

    #[test]
    fn test_function_error_case() {
        // Error conditions
    }
}
```

### Testing your fix:

After fixing the bugs, run:
```bash
cargo test
```

You should see:
- All 8 tests pass
- No assertion failures
- Output shows: "test result: ok. 8 passed"

### Debugging tips:

1. Read test failure messages - they show actual vs expected
2. For assertion failures, compiler shows which line failed
3. Test names should describe what's being tested
4. Organize tests logically by function
5. Test edge cases: empty, zero, negative, single element

### Key concepts to remember:

- **#[test] attribute**: Marks function as a test
- **#[cfg(test)]**: Module only compiled for testing
- **use super::***:  Import items to test from parent module
- **assert!() family**: Different macros for different purposes
- **Option handling**: Compare Option with Option, not with primitives
- **Test naming**: Convention is test_function_case_scenario

### Common Test Patterns:

```rust
// Pattern 1: Test normal case
#[test]
fn test_function_normal() {
    let result = function(input);
    assert_eq!(result, expected);
}

// Pattern 2: Test error case with Option
#[test]
fn test_function_none() {
    let result = function_returning_option(bad_input);
    assert_eq!(result, None);
}

// Pattern 3: Test multiple assertions
#[test]
fn test_function_multiple() {
    assert!(condition1);
    assert_eq!(value1, expected1);
    assert_ne!(value2, unexpected2);
}

// Pattern 4: Test panic with #[should_panic]
#[test]
#[should_panic]
fn test_function_panics() {
    function_that_should_panic();
}
```

### Type Compatibility in Assertions:

```rust
// CORRECT - boolean with assert!()
assert!(is_even(4));

// CORRECT - boolean with assert_eq!()
assert_eq!(is_even(4), true);  // Works but verbose

// CORRECT - Option with Option
assert_eq!(find_max(&[]), None);

// WRONG - Option with primitive
assert_eq!(find_max(&[]), 0);  // Type error

// CORRECT - i32 with i32
assert_eq!(factorial(5), 120);

// WRONG - u32 with i32
assert_eq!(factorial(5), 120i32);  // Type mismatch if types differ
```

### If still stuck:

1. **Assertion failing**: Check if expected value matches function's actual return
2. **factorial(0) test**: Should expect 1 (0! = 1 by definition)
3. **find_max empty**: Should expect None, not any number
4. **Type errors**: Make sure comparing same types (Option with Option, etc.)
5. **Boolean tests**: Use `assert!(!cond)` when expecting false

The fixes are usually 3 line changes!

### Related concepts:

- **Test-driven development**: Write tests first, then implementation
- **Edge cases**: Empty collections, zero values, negative numbers
- **Test coverage**: Aim to test all code paths
- **Assertions**: Different macros for different types
- **Test organization**: Group related tests together

