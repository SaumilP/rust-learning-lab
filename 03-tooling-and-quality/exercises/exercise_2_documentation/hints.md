# Hints for Exercise 2: Add Documentation and Pass Doc Tests

## Stuck? Here are some hints:

### About the documentation issues:

**Bug 1: Missing Module Documentation**
- Module docs use `//!` at the start of the file
- They describe what the module/crate provides
- Fix: Add at the very top of the file
  ```rust
  //! # Math Utilities
  //!
  //! This module provides common mathematical functions including:
  //! - Greatest common divisor calculation
  //! - Prime number checking
  //! - FizzBuzz implementation
  //! - Temperature conversion
  ```

**Bug 2: Doc Test Not Finding Function**
- Doc tests run as if they're external code
- They need to import or define functions they use
- Option 1: Include function definition in test (hidden with `#`)
- Option 2: Use `no_run` to skip actually running
- Fix with hidden setup:
  ```rust
  /// # Examples
  ///
  /// ```
  /// # fn gcd(a: u32, b: u32) -> u32 { if b == 0 { a } else { gcd(b, a % b) } }
  /// assert_eq!(gcd(48, 18), 6);
  /// assert_eq!(gcd(100, 25), 25);
  /// ```
  ```

**Bug 3: Missing Documentation for is_prime**
- Add complete doc comment with description, examples
- Document edge cases (0, 1, 2)
- Fix:
  ```rust
  /// Checks if a number is prime.
  ///
  /// A prime number is a natural number greater than 1 that has no positive
  /// divisors other than 1 and itself.
  ///
  /// # Arguments
  ///
  /// * `n` - The number to check for primality
  ///
  /// # Returns
  ///
  /// `true` if the number is prime, `false` otherwise.
  ///
  /// # Examples
  ///
  /// ```
  /// # fn is_prime(n: u32) -> bool { /* impl */ true }
  /// assert!(is_prime(2));
  /// assert!(is_prime(7));
  /// assert!(!is_prime(4));
  /// assert!(!is_prime(1));
  /// ```
  pub fn is_prime(n: u32) -> bool {
  ```

**Bug 4: Wrong Assertion in fizzbuzz**
- The example tests `fizzbuzz(15)` expecting "Fizz"
- But 15 is divisible by both 3 and 5, so result is "FizzBuzz"
- Fix the assertion:
  ```rust
  /// # Examples
  ///
  /// ```
  /// # fn fizzbuzz(n: u32) -> String { /* impl */ String::new() }
  /// assert_eq!(fizzbuzz(15), "FizzBuzz");
  /// assert_eq!(fizzbuzz(9), "Fizz");
  /// assert_eq!(fizzbuzz(10), "Buzz");
  /// assert_eq!(fizzbuzz(7), "7");
  /// ```
  ```

**Bug 5: Missing celsius_to_fahrenheit Documentation**
- Add description, formula, and examples
- Fix:
  ```rust
  /// Converts temperature from Celsius to Fahrenheit.
  ///
  /// Uses the formula: F = C * 9/5 + 32
  ///
  /// # Arguments
  ///
  /// * `celsius` - Temperature in degrees Celsius
  ///
  /// # Returns
  ///
  /// Temperature in degrees Fahrenheit
  ///
  /// # Examples
  ///
  /// ```
  /// # fn celsius_to_fahrenheit(c: f64) -> f64 { c * 9.0/5.0 + 32.0 }
  /// assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
  /// assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
  /// ```
  ```

### Testing your documentation:

Run doc tests:
```bash
cargo test --doc
```

Generate HTML docs:
```bash
cargo doc --open
```

### Doc comment syntax:

```rust
/// Single-line doc comment for items

/// Multi-line doc comment
/// continues on next line
///
/// # Sections use markdown headers
///
/// Regular markdown works: **bold**, *italic*, `code`
///
/// # Examples
///
/// ```
/// // Code here is compiled and run as a test
/// let x = 5;
/// assert_eq!(x, 5);
/// ```

//! Module-level documentation
//! Uses //! instead of ///
```

### Hiding lines in doc tests:

```rust
/// ```
/// # // Lines starting with # are hidden but still run
/// # fn setup() -> i32 { 42 }
/// let value = setup();
/// assert_eq!(value, 42);
/// ```
```

### Special doc test attributes:

```rust
/// ```ignore
/// // This code won't be run
/// ```

/// ```no_run
/// // This compiles but won't execute
/// ```

/// ```should_panic
/// // This should panic
/// panic!("expected");
/// ```

/// ```compile_fail
/// // This should NOT compile
/// let x: i32 = "string";
/// ```
```

### Key documentation sections:

- Description (at the top)
- `# Arguments` - describe parameters
- `# Returns` - describe return value
- `# Examples` - show usage
- `# Panics` - when it might panic
- `# Errors` - when it returns errors
- `# Safety` - for unsafe functions

### If still stuck:

1. Run `cargo test --doc` to see which tests fail
2. Read the error messages - they show expected vs actual
3. Use `# ` to hide setup code in examples
4. Make sure assertions match actual function behavior

The fix involves adding/correcting about 50-80 lines of documentation!

## Testing Checklist

- [ ] Module-level documentation present
- [ ] All functions have doc comments
- [ ] Each function has at least one example
- [ ] All doc tests pass (`cargo test --doc`)
- [ ] Documentation generates correctly (`cargo doc`)
