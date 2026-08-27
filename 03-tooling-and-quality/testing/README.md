# Concept: Testing

## Overview

Testing is fundamental to building reliable software. Rust has excellent built-in testing support with a powerful testing framework in the standard library. Unlike languages where testing feels like an afterthought, Rust's testing capabilities are integrated directly into the language and tooling. This concept covers writing, running, and organizing tests effectively.

## Learning Objectives

By the end of this concept, you will understand:
- Unit tests and how to write them
- Test organization and modules
- Running tests selectively
- Test assertions and expectations
- Integration tests
- Benchmarking basics
- Test-driven development patterns
- Common testing patterns

## Theory

### What is Testing?

Testing verifies that code behaves as expected. Tests are mini-programs that validate specific functionality:

```rust
// Test: verify add function
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

// Code under test
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Unit Tests

Unit tests verify individual functions or modules. They live in the same file as the code:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-2, 3), 1);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }
}
```

**Key features:**
- `#[test]` attribute marks function as test
- `#[cfg(test)]` module compiles only for testing
- `use super::*` imports parent module items
- Tests are automatically discovered and run

### Assertions

Rust provides several assertion macros:

**assert!(condition)**

Panics if condition is false:

```rust
#[test]
fn test_is_even() {
    assert!(4 % 2 == 0);
}

#[test]
#[should_panic]
fn test_assertion_fails() {
    assert!(false);  // Will panic
}
```

**assert_eq!(left, right)**

Panics if values aren't equal:

```rust
#[test]
fn test_equals() {
    assert_eq!(add(2, 3), 5);
    assert_eq!("hello".len(), 5);
}
```

**assert_ne!(left, right)**

Panics if values are equal:

```rust
#[test]
fn test_not_equals() {
    assert_ne!(add(1, 1), 3);
}
```

**Custom messages:**

```rust
#[test]
fn test_with_message() {
    assert!(
        add(2, 3) == 5,
        "Expected 5, got {}",
        add(2, 3)
    );

    assert_eq!(
        add(1, 1),
        2,
        "Failed to add 1 + 1"
    );
}
```

### Testing for Panics

The `#[should_panic]` attribute tests that code panics:

```rust
#[test]
#[should_panic]
fn test_divide_by_zero() {
    let _ = 10 / 0;  // Will panic
}

// With expected message
#[test]
#[should_panic(expected = "divide")]
fn test_divide_panic() {
    panic!("Cannot divide");
}
```

### Testing Results

Return `Result` from tests instead of using assertions:

```rust
#[test]
fn test_with_result() -> Result<(), String> {
    if add(2, 3) == 5 {
        Ok(())
    } else {
        Err("Addition failed".to_string())
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_add

# Run with output even if passing
cargo test -- --nocapture

# Run single-threaded
cargo test -- --test-threads=1

# List all tests without running
cargo test -- --list
```

### Test Organization

**Module-level tests:**

```rust
// In main.rs or lib.rs
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add() {
            assert_eq!(add(2, 3), 5);
        }
    }
}
```

**Integration tests:**

```
src/
  lib.rs
  main.rs
tests/
  integration_test.rs    <- Compiled separately
```

Integration tests in `tests/` directory test public API:

```rust
// tests/integration_test.rs
use my_crate::calculator;

#[test]
fn test_calculator_add() {
    assert_eq!(calculator::add(2, 3), 5);
}
```

### Test Fixtures and Setup

Setting up test data:

```rust
struct Calculator;

impl Calculator {
    fn add(a: i32, b: i32) -> i32 { a + b }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Calculator {
        Calculator
    }

    #[test]
    fn test_add() {
        let calc = setup();
        assert_eq!(calc.add(2, 3), 5);
    }
}
```

### Benchmarking

Nightly Rust feature for performance testing:

```rust
#![feature(test)]
extern crate test;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_fib_20(b: &mut Bencher) {
        b.iter(|| fibonacci(20));
    }
}
```

Run with: `cargo +nightly bench`

### Common Testing Patterns

**Arrange-Act-Assert (AAA)**

```rust
#[test]
fn test_user_registration() {
    // Arrange: Set up test data
    let user_data = UserData::new("alice", "alice@example.com");

    // Act: Perform the action
    let user = User::from(user_data);

    // Assert: Verify results
    assert_eq!(user.name, "alice");
    assert_eq!(user.email, "alice@example.com");
}
```

**Testing edge cases:**

```rust
#[test]
fn test_divide_edge_cases() {
    // Normal case
    assert_eq!(divide(10, 2), Ok(5));

    // Division by zero
    assert!(divide(10, 0).is_err());

    // Zero divided
    assert_eq!(divide(0, 5), Ok(0));

    // Negative numbers
    assert_eq!(divide(-10, 2), Ok(-5));
}
```

**Testing with multiple scenarios:**

```rust
#[test]
fn test_validate_email() {
    // Valid emails
    assert!(validate_email("user@example.com"));

    // Invalid emails
    assert!(!validate_email("invalid"));
    assert!(!validate_email("@example.com"));
    assert!(!validate_email("user@.com"));
}
```

## Syntax

### Test Definition

```rust
#[test]
fn test_name() {
    assert_eq!(actual, expected);
}
```

### Test Attributes

```rust
#[test]                        // Mark as test
#[ignore]                      // Skip test
#[should_panic]                // Expect panic
#[should_panic(expected = "msg")]  // Expect specific panic
#[cfg(test)]                   // Compile only for tests
```

### Assertions

```rust
assert!(condition);                     // Panics if false
assert_eq!(left, right);                // Panics if not equal
assert_ne!(left, right);                // Panics if equal
debug_assert!(condition);               // Debug builds only
debug_assert_eq!(left, right);          // Debug builds only
```

### Test Module

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        assert_eq!(add(2, 3), 5);
    }
}
```

## Common Patterns

### Pattern 1: Testing success path

```rust
#[test]
fn test_success_case() {
    let result = operation();
    assert_eq!(result, expected);
}
```

### Pattern 2: Testing error cases

```rust
#[test]
fn test_error_handling() {
    let result = risky_operation();
    assert!(result.is_err());
}
```

### Pattern 3: Testing with data setup

```rust
#[test]
fn test_with_data() {
    let data = create_test_data();
    let result = process(&data);
    assert!(result.is_valid());
}
```

### Pattern 4: Testing expected panics

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_panic_on_bounds() {
    let v = vec![1, 2, 3];
    let _ = v[10];
}
```

### Pattern 5: Testing with assertions chain

```rust
#[test]
fn test_multiple_assertions() {
    let result = complex_operation();
    assert_eq!(result.field1, expected1);
    assert_eq!(result.field2, expected2);
    assert_eq!(result.field3, expected3);
}
```

### Pattern 6: Integration test

```rust
// tests/integration_test.rs
#[test]
fn test_public_api() {
    let result = my_lib::public_function();
    assert_eq!(result, expected);
}
```

### Pattern 7: Parameterized testing

```rust
#[test]
fn test_add_with_values() {
    let test_cases = vec![
        (1, 1, 2),
        (2, 3, 5),
        (0, 0, 0),
        (-1, 1, 0),
    ];

    for (a, b, expected) in test_cases {
        assert_eq!(add(a, b), expected);
    }
}
```

### Pattern 8: Testing helper functions

```rust
fn create_test_user() -> User {
    User {
        id: 1,
        name: "Test".to_string(),
        email: "test@example.com".to_string(),
    }
}

#[test]
fn test_user_operations() {
    let user = create_test_user();
    assert_eq!(user.id, 1);
}
```

## Common Mistakes

### Mistake 1: Tests that are too broad

```rust
// ❌ Bad: tests too many things
#[test]
fn test_user_system() {
    let user = create_user();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    let updated = update_user(&user);
    assert!(updated.is_ok());
    // ... 20 more assertions
}

// ✅ Good: focused tests
#[test]
fn test_create_user_with_valid_data() {
    let user = create_user("Alice", "alice@example.com");
    assert_eq!(user.name, "Alice");
}

#[test]
fn test_update_user_succeeds() {
    let user = create_user("Alice", "alice@example.com");
    let result = update_user(&user);
    assert!(result.is_ok());
}
```

### Mistake 2: Weak assertions

```rust
// ❌ Weak: just checks if truthy
#[test]
fn test_add() {
    assert!(add(2, 3) > 0);  // Not specific enough
}

// ✅ Strong: exact expectations
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);  // Verifies exact value
}
```

### Mistake 3: No negative tests

```rust
// ❌ Missing: only tests success
#[test]
fn test_divide() {
    assert_eq!(divide(10, 2), 5);
}

// ✅ Complete: tests both cases
#[test]
fn test_divide_success() {
    assert_eq!(divide(10, 2), Ok(5));
}

#[test]
fn test_divide_by_zero_fails() {
    assert!(divide(10, 0).is_err());
}
```

### Mistake 4: Tests with side effects

```rust
// ❌ Bad: global state shared between tests
static mut COUNTER: i32 = 0;

#[test]
fn test_increment() {
    unsafe { COUNTER += 1; }
    assert_eq!(unsafe { COUNTER }, 1);
}

// ✅ Good: isolated, no side effects
#[test]
fn test_increment() {
    let mut counter = 0;
    counter += 1;
    assert_eq!(counter, 1);
}
```

### Mistake 5: Ignoring edge cases

```rust
// ❌ Incomplete: missing edge cases
#[test]
fn test_process() {
    assert_eq!(process(5), expected);
}

// ✅ Complete: covers edge cases
#[test]
fn test_process_with_zero() {
    assert_eq!(process(0), expected_zero);
}

#[test]
fn test_process_with_negative() {
    assert_eq!(process(-5), expected_negative);
}

#[test]
fn test_process_with_max() {
    assert_eq!(process(i32::MAX), expected_max);
}
```

## Real-World Examples

### Example 1: Testing a validator

```rust
fn validate_email(email: &str) -> Result<(), String> {
    if email.is_empty() {
        Err("Email cannot be empty".to_string())
    } else if !email.contains('@') {
        Err("Email must contain @".to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        assert_eq!(validate_email("user@example.com"), Ok(()));
    }

    #[test]
    fn test_empty_email() {
        assert!(validate_email("").is_err());
    }

    #[test]
    fn test_missing_at_symbol() {
        assert!(validate_email("userexample.com").is_err());
    }
}
```

### Example 2: Testing data structures

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack { items: Vec::new() };
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
    }

    #[test]
    fn test_pop_empty_stack() {
        let mut stack: Stack<i32> = Stack { items: Vec::new() };
        assert_eq!(stack.pop(), None);
    }
}
```

### Example 3: Integration test

```rust
// tests/integration_test.rs
#[test]
fn test_end_to_end_workflow() {
    let config = load_config().expect("Config load");
    let client = Client::new(&config);
    let response = client.get("/api/data").expect("Request");
    assert_eq!(response.status(), 200);
}
```

### Example 4: Parameterized testing

```rust
#[test]
fn test_fibonacci_series() {
    let test_cases = vec![
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (4, 3),
        (5, 5),
        (6, 8),
    ];

    for (input, expected) in test_cases {
        assert_eq!(
            fibonacci(input),
            expected,
            "fibonacci({}) should equal {}",
            input,
            expected
        );
    }
}
```

## Related Concepts

### Prerequisites
- **Functions** - Test functions
- **Assertions** - Used in testing
- **Modules** - Test organization

### What comes next
- **Documentation** - Doc tests
- **Code Quality** - Coverage tools
- **Debugging** - Debugging failed tests

### Cross-references
- Module 02: Testing collections
- Module 03: Documentation and testing
- CI/CD: Automated test running

## Best Practices

### Test names should be descriptive

```rust
// ✅ Good: clear what's being tested
#[test]
fn test_add_two_positive_numbers() { }

#[test]
fn test_divide_by_zero_returns_error() { }

// ❌ Vague: unclear intent
#[test]
fn test1() { }

#[test]
fn test_works() { }
```

### Follow Arrange-Act-Assert

```rust
#[test]
fn test_user_creation() {
    // Arrange: Set up
    let name = "Alice";
    let email = "alice@example.com";

    // Act: Do something
    let user = User::new(name, email);

    // Assert: Verify
    assert_eq!(user.name, name);
    assert_eq!(user.email, email);
}
```

### Test edge cases and error paths

```rust
#[test]
fn test_normal_case() { /* ... */ }

#[test]
fn test_empty_input() { /* ... */ }

#[test]
fn test_boundary_values() { /* ... */ }

#[test]
fn test_error_case() { /* ... */ }
```

### Keep tests independent

```rust
// ✅ Good: each test is independent
#[test]
fn test_case_1() {
    let data = setup_data();
    // Use data
}

#[test]
fn test_case_2() {
    let data = setup_data();
    // Use different data
}
```

## Summary

- **Unit tests** verify individual functions/modules
- **Tests live** with code in `#[cfg(test)]` modules
- **Assertions** verify expected behavior
- **Integration tests** in `tests/` directory test public API
- **Edge cases** and error paths should be tested
- **Tests should be** independent and focused
- **Test names** should clearly describe what's tested

## Key Takeaways

1. Tests are first-class citizens in Rust
2. Unit tests live in the same file as code
3. Integration tests live in `tests/` directory
4. `#[test]` marks test functions
5. Assertions verify expected behavior
6. Tests should be focused and independent
7. Use descriptive test names
8. Test both success and failure paths

## Practice Exercise Ideas

1. Write unit tests for a simple function
2. Test edge cases and error conditions
3. Create integration tests for a module
4. Use test fixtures for setup
5. Test with multiple assertions
6. Practice parameterized testing
7. Write tests before implementation (TDD)

---

**Time to complete this concept**: 2-2.5 hours
**Difficulty**: Intermediate
**Prerequisite**: Functions, Error Handling
**Next concept**: Documentation

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
