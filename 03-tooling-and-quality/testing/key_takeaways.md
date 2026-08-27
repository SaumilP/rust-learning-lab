# Key Takeaways: Testing

## Quick Reference

### Basic Test

```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

### Test Module

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(actual, expected);
    }
}
```

### Assertions

```rust
assert!(condition);              // Panics if false
assert_eq!(left, right);         // Panics if not equal
assert_ne!(left, right);         // Panics if equal
debug_assert!(condition);        // Debug only
```

### Running Tests

```bash
cargo test               # Run all tests
cargo test test_name     # Run specific test
cargo test -- --nocapture   # Show output
cargo test -- --test-threads=1   # Single-threaded
```

## Essential Concepts

### 1. Test Anatomy

```rust
#[cfg(test)]          // Compile only for tests
mod tests {
    use super::*;     // Import parent items

    #[test]           // Mark as test
    fn test_name() {
        // Arrange: Setup
        let input = 5;

        // Act: Execute
        let result = add(input, 3);

        // Assert: Verify
        assert_eq!(result, 8);
    }
}
```

### 2. Three Assertion Types

| Macro | Panics If | Use |
|-------|-----------|-----|
| `assert!(cond)` | cond is false | Boolean checks |
| `assert_eq!(a, b)` | a ≠ b | Value equality |
| `assert_ne!(a, b)` | a = b | Value inequality |

```rust
assert!(true);                // OK
assert_eq!(2 + 2, 4);        // OK
assert_ne!(1, 2);            // OK
```

### 3. Test Execution

```bash
# Run all tests
cargo test

# Run specific test by name
cargo test test_add

# Run tests containing word
cargo test add

# Show println output
cargo test -- --nocapture

# Run single-threaded
cargo test -- --test-threads=1

# List tests without running
cargo test -- --list
```

### 4. Testing Panic Cases

```rust
#[test]
#[should_panic]
fn test_panic() {
    panic!("This is expected");  // OK
}

#[test]
#[should_panic(expected = "div")]
fn test_specific_panic() {
    panic!("Cannot divide");  // OK, contains "div"
}
```

### 5. Test Organization

**Unit tests:**
```rust
// In same file as code
#[cfg(test)]
mod tests { }
```

**Integration tests:**
```
tests/
  integration_test.rs  // Separate from code
```

**Integration tests are for public API**

### 6. Test with Result

```rust
#[test]
fn test_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err("Math is broken".to_string())
    }
}
```

### 7. Ignored Tests

```rust
#[test]
#[ignore]
fn expensive_test() {
    // Skipped by default
}

// Run with: cargo test -- --ignored
```

## Common Patterns

### Pattern 1: Success path
```rust
#[test]
fn test_normal_case() {
    assert_eq!(add(2, 3), 5);
}
```

### Pattern 2: Error handling
```rust
#[test]
fn test_error_case() {
    assert!(divide(10, 0).is_err());
}
```

### Pattern 3: Edge cases
```rust
#[test]
fn test_boundary() {
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(i32::MAX, 0), i32::MAX);
}
```

### Pattern 4: Multiple assertions
```rust
#[test]
fn test_struct() {
    let obj = create_object();
    assert_eq!(obj.field1, expected1);
    assert_eq!(obj.field2, expected2);
    assert!(obj.is_valid());
}
```

### Pattern 5: Parameterized
```rust
#[test]
fn test_values() {
    for (input, expected) in vec![
        (1, 2),
        (2, 4),
        (3, 6),
    ] {
        assert_eq!(double(input), expected);
    }
}
```

### Pattern 6: Test setup helper
```rust
fn create_test_user() -> User {
    User { name: "Test".to_string(), age: 25 }
}

#[test]
fn test_with_setup() {
    let user = create_test_user();
    assert_eq!(user.age, 25);
}
```

### Pattern 7: Custom error messages
```rust
#[test]
fn test_with_message() {
    assert_eq!(add(2, 3), 5, "Addition failed");
    assert_eq!(
        add(1, 1),
        2,
        "Expected 2, got {}", add(1, 1)
    );
}
```

### Pattern 8: Panic expected
```rust
#[test]
#[should_panic]
fn test_out_of_bounds() {
    let v = vec![1, 2, 3];
    let _ = v[10];
}
```

## Checklist: Good Tests

- [ ] Test has descriptive name
- [ ] Tests one thing clearly
- [ ] Uses specific assertions
- [ ] Tests both success and failure paths
- [ ] Independent (no shared state)
- [ ] Uses Arrange-Act-Assert pattern
- [ ] Has meaningful error messages
- [ ] No side effects

## Error Prevention

### ❌ DON'T: Test too much in one test
```rust
#[test]
fn test_everything() {  // Too broad
    test_add();
    test_multiply();
    test_parse();
    // 20 more things...
}
```

### ✅ DO: Focused tests
```rust
#[test]
fn test_add_two_numbers() {  // Specific
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_multiply_two_numbers() {  // Separate
    assert_eq!(multiply(2, 3), 6);
}
```

### ❌ DON'T: Weak assertions
```rust
#[test]
fn test_add() {
    assert!(add(2, 3) > 0);  // Too loose
}
```

### ✅ DO: Strong assertions
```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);  // Exact
}
```

### ❌ DON'T: Only test success cases
```rust
#[test]
fn test_divide() {
    assert_eq!(divide(10, 2), 5);  // Missing error case
}
```

### ✅ DO: Test both paths
```rust
#[test]
fn test_divide_success() {
    assert_eq!(divide(10, 2), Ok(5));
}

#[test]
fn test_divide_error() {
    assert!(divide(10, 0).is_err());
}
```

### ❌ DON'T: Rely on test order
```rust
static mut STATE: i32 = 0;

#[test]
fn test_1() {
    unsafe { STATE = 1; }
}

#[test]
fn test_2() {
    assert_eq!(unsafe { STATE }, 1);  // Depends on test 1
}
```

### ✅ DO: Independent tests
```rust
#[test]
fn test_add_one() {
    let mut state = 0;
    state += 1;
    assert_eq!(state, 1);
}

#[test]
fn test_add_two() {
    let mut state = 0;
    state += 2;
    assert_eq!(state, 2);
}
```

## Test Attributes

| Attribute | Purpose |
|-----------|---------|
| `#[test]` | Mark as test function |
| `#[cfg(test)]` | Compile only for tests |
| `#[should_panic]` | Expect panic |
| `#[should_panic(expected="msg")]` | Expect specific panic |
| `#[ignore]` | Skip test |

## Assertion Macros

| Macro | Purpose | Example |
|-------|---------|---------|
| `assert!()` | Check condition | `assert!(x > 0)` |
| `assert_eq!()` | Check equality | `assert_eq!(a, b)` |
| `assert_ne!()` | Check inequality | `assert_ne!(a, b)` |
| `debug_assert!()` | Debug only | Debug builds |
| `debug_assert_eq!()` | Debug only | Debug builds |

## Organization Patterns

**Unit test in same file:**
```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 { a + b }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() { }
}
```

**Integration test separate file:**
```rust
// tests/integration_test.rs
use my_crate::add;

#[test]
fn test_add() { }
```

**Test helper module:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_data() -> Data { /* ... */ }

    #[test]
    fn test_case_1() {
        let data = setup_data();
    }

    #[test]
    fn test_case_2() {
        let data = setup_data();
    }
}
```

## Common Test Scenarios

| Scenario | Pattern | Example |
|----------|---------|---------|
| Normal operation | `assert_eq!` | `assert_eq!(add(2,3), 5)` |
| Error handling | `assert!(is_err)` | `assert!(op().is_err())` |
| Boundary values | Multiple asserts | Test 0, -1, max |
| Panic expected | `#[should_panic]` | Expected panic |
| Complex setup | Helper function | `setup_data()` |
| Multiple values | Parameterized | Loop with test cases |

## Tips for Better Tests

### Clear naming
```rust
#[test]
fn test_add_returns_sum_of_two_positive_numbers() { }
```

### Arrange-Act-Assert
```rust
#[test]
fn test_calculation() {
    // Arrange
    let a = 2;
    let b = 3;

    // Act
    let result = add(a, b);

    // Assert
    assert_eq!(result, 5);
}
```

### Edge cases
```rust
#[test]
fn test_zero_values() { }

#[test]
fn test_negative_values() { }

#[test]
fn test_boundary_values() { }
```

### Helper functions for setup
```rust
fn new_user(name: &str) -> User {
    User { name: name.to_string(), age: 25 }
}

#[test]
fn test_user_validation() {
    let user = new_user("Alice");
    assert_eq!(user.name, "Alice");
}
```

## Related Concepts

- **Error Handling** - Testing error cases
- **Assertions** - Verifying behavior
- **Modules** - Test organization
- **Traits** - Testing trait implementations

## Time Estimates

- Reading this takeaway: 10-15 minutes
- Reviewing patterns: 10 minutes
- Practice drills: 20-30 minutes
- Total: 40-55 minutes

## Practice Questions

1. What attribute marks a function as a test?
2. What's the difference between assert! and assert_eq!?
3. How do you test code that should panic?
4. Where should unit tests live?
5. Where should integration tests live?
6. What does #[cfg(test)] do?
7. How do you run a specific test?
8. What's the AAA pattern?

## Cargo Test Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Show println output
cargo test -- --nocapture

# Run single-threaded
cargo test -- --test-threads=1

# List all tests
cargo test -- --list

# Run ignored tests
cargo test -- --ignored

# Run only integration tests
cargo test --test integration_test

# Run with backtrace
RUST_BACKTRACE=1 cargo test
```

---

**Status**: Quick reference guide
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Intermediate
**Part of**: Module 03 - Tooling and Quality
