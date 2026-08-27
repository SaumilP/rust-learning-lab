# Concept: Documentation

## Overview

Documentation is often overlooked but is critical for maintainability and usability. Rust has excellent built-in support for documentation through doc comments, which are compiled and tested as part of the build process. This concept covers writing clear documentation, generating documentation with cargo doc, and best practices for documenting Rust code.

## Learning Objectives

By the end of this concept, you will understand:
- Doc comments syntax (///)
- Writing effective documentation
- Documenting functions, structs, and modules
- Doc tests (running code in documentation)
- Generating and viewing documentation
- Markdown support in documentation
- Documentation best practices
- Common documentation patterns

## Theory

### Doc Comments

Doc comments use `///` for item documentation:

```rust
/// Adds two numbers together.
///
/// # Arguments
/// * `a` - The first number
/// * `b` - The second number
///
/// # Returns
/// The sum of a and b
///
/// # Examples
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Key features:**
- `///` marks documentation
- Supports Markdown formatting
- Can include code examples (doc tests)
- Automatically validated by compiler

### Documentation Structure

**Best practices for documentation:**

```rust
/// Brief description of what it does.
///
/// Longer explanation if needed. This is where you can explain
/// the behavior in more detail, edge cases, and important notes.
///
/// # Arguments
/// * `param1` - What it is
/// * `param2` - What it is
///
/// # Returns
/// What the function returns
///
/// # Panics
/// When the function panics
///
/// # Errors
/// What errors it can return
///
/// # Examples
/// ```
/// // Example code
/// ```
pub fn function(param1: Type1, param2: Type2) -> ReturnType {
    // implementation
}
```

### Common Documentation Sections

**# Arguments**

Documents function parameters:

```rust
/// # Arguments
/// * `name` - The user's name
/// * `age` - The user's age in years
```

**# Returns**

Documents return value:

```rust
/// # Returns
/// A User struct with the provided information
```

**# Panics**

Documents panic conditions:

```rust
/// # Panics
/// Panics if index is out of bounds
```

**# Errors**

Documents possible errors:

```rust
/// # Errors
/// Returns `Err` if the file doesn't exist
/// Returns `Err` if the file cannot be read
```

**# Examples**

Provides usage examples (with testing):

```rust
/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
```

**# Safety**

Documents unsafe behavior:

```rust
/// # Safety
/// This function is only safe if `ptr` is a valid pointer
```

### Documenting Different Items

**Functions:**

```rust
/// Calculates the factorial of a number.
pub fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}
```

**Structs:**

```rust
/// Represents a user in the system.
///
/// Users have a unique ID, name, and email address.
#[derive(Debug)]
pub struct User {
    /// Unique identifier for the user
    pub id: u32,
    /// User's full name
    pub name: String,
    /// User's email address
    pub email: String,
}
```

**Enums:**

```rust
/// Represents the result of an operation.
pub enum Status {
    /// Operation completed successfully
    Success,
    /// Operation encountered an error
    Error(String),
    /// Operation is still pending
    Pending,
}
```

**Modules:**

```rust
//! This module provides utility functions for string manipulation.
//!
//! It includes functions for trimming, padding, and case conversion.

pub fn trim_spaces(s: &str) -> &str {
    s.trim()
}
```

### Doc Tests

Doc tests are code examples in documentation that are compiled and run:

```rust
/// Adds two numbers.
///
/// # Examples
/// ```
/// assert_eq!(my_crate::add(2, 3), 5);
/// assert_eq!(my_crate::add(-1, 1), 0);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run with: `cargo test --doc`

**Doc test with setup:**

```rust
/// # Examples
/// ```
/// let mut vec = vec![1, 2, 3];
/// vec.push(4);
/// assert_eq!(vec.len(), 4);
/// ```
pub fn example_function() { }
```

**Ignoring doc tests:**

```rust
/// # Examples
/// ```ignore
/// // This example is not tested
/// ```
```

**Hiding setup code:**

```rust
/// # Examples
/// ```
/// # fn hidden_setup() {}
/// # hidden_setup();
/// // Only this line is shown
/// assert_eq!(2 + 2, 4);
/// ```
```

### Generating Documentation

```bash
# Generate and open documentation
cargo doc --open

# Generate without opening
cargo doc

# Generate for all dependencies
cargo doc --document-private-items
```

### Markdown in Documentation

Documentation supports full Markdown:

```rust
/// # Heading
/// This is a paragraph with **bold** and *italic* text.
///
/// ## Subheading
/// Code inline: `some_function()`
///
/// Code block:
/// ```rust
/// let x = 5;
/// ```
///
/// Lists:
/// * Item 1
/// * Item 2
///
/// [Link text](https://example.com)
pub fn function() { }
```

### Referencing Items

Reference other items with backticks:

```rust
/// See [`add`] for adding two numbers.
/// See [`User`] struct for user information.
pub fn function() { }
```

### Common Documentation Patterns

**Simple function:**

```rust
/// Doubles a number.
pub fn double(x: i32) -> i32 {
    x * 2
}
```

**Function with error:**

```rust
/// Parses a string into an integer.
///
/// # Errors
/// Returns `Err` if the string is not a valid number.
///
/// # Examples
/// ```
/// assert_eq!(parse_int("42"), Ok(42));
/// assert!(parse_int("abc").is_err());
/// ```
pub fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}
```

**Struct with field docs:**

```rust
/// A point in 2D space.
#[derive(Debug)]
pub struct Point {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}
```

**Module documentation:**

```rust
//! String utilities for text processing.
//!
//! This module provides common string operations.

/// Reverses a string.
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}
```

## Syntax

### Doc Comment

```rust
/// Documentation for an item
pub fn function() { }

/// Documentation for structs/enums
pub struct Item { }

//! Documentation for module
```

### Documentation Sections

```rust
/// Description
///
/// # Arguments
/// * `param` - description
///
/// # Returns
/// Description of return
///
/// # Errors
/// Description of errors
///
/// # Panics
/// When it panics
///
/// # Examples
/// ```
/// example code
/// ```
pub fn function() { }
```

### Doc Tests

```rust
/// # Examples
/// ```
/// assert_eq!(function(), expected);
/// ```
pub fn function() { }
```

## Common Patterns

### Pattern 1: Basic function documentation

```rust
/// Calculates the sum of two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Pattern 2: Comprehensive documentation

```rust
/// Validates an email address.
///
/// Checks if the email contains an @ symbol and a domain.
///
/// # Arguments
/// * `email` - The email address to validate
///
/// # Returns
/// `true` if the email is valid, `false` otherwise
///
/// # Examples
/// ```
/// assert!(validate_email("user@example.com"));
/// assert!(!validate_email("invalid"));
/// ```
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}
```

### Pattern 3: Struct documentation

```rust
/// Represents a configuration.
///
/// Configuration can be loaded from files or created manually.
pub struct Config {
    /// Configuration name
    pub name: String,
    /// Configuration version
    pub version: u32,
}
```

### Pattern 4: Module documentation

```rust
//! Configuration management utilities.
//!
//! This module handles loading and parsing configuration files.
//!
//! # Examples
//!
//! Loading configuration:
//! ```ignore
//! let config = load_config("config.toml")?;
//! ```

/// Load configuration from file.
pub fn load_config(path: &str) -> Result<Config, Error> {
    // implementation
}
```

### Pattern 5: Error documentation

```rust
/// Parse a number from a string.
///
/// # Errors
/// Returns `ParseError` if:
/// * The string is empty
/// * The string contains non-numeric characters
///
/// # Examples
/// ```
/// assert_eq!(parse_number("42"), Ok(42));
/// assert!(parse_number("abc").is_err());
/// ```
pub fn parse_number(s: &str) -> Result<i32, ParseError> {
    s.parse()
}
```

### Pattern 6: Doc test with setup

```rust
/// # Examples
/// ```
/// let data = vec![1, 2, 3];
/// let result = process(&data);
/// assert_eq!(result.len(), 3);
/// ```
pub fn process(data: &[i32]) -> Vec<i32> {
    data.to_vec()
}
```

### Pattern 7: Enum documentation

```rust
/// Result of a query operation.
pub enum QueryResult {
    /// Query succeeded with data
    Success(Vec<String>),
    /// Query failed with error message
    Error(String),
    /// Query is still executing
    Pending,
}
```

### Pattern 8: Panic documentation

```rust
/// Gets element at index without bounds checking.
///
/// # Panics
/// Panics if index is >= array length
pub fn get_unchecked(arr: &[i32], index: usize) -> i32 {
    arr[index]
}
```

## Common Mistakes

### Mistake 1: No documentation

```rust
// ❌ Bad: no documentation
pub fn process(input: &str) -> String {
    input.to_uppercase()
}

// ✅ Good: clear documentation
/// Converts text to uppercase.
pub fn process(input: &str) -> String {
    input.to_uppercase()
}
```

### Mistake 2: Incomplete documentation

```rust
// ❌ Incomplete: missing examples and details
/// A user struct.
pub struct User {
    pub name: String,
    pub age: u32,
}

// ✅ Complete: fully documented
/// Represents a user in the system.
///
/// # Fields
/// * `name` - User's full name
/// * `age` - User's age in years
pub struct User {
    pub name: String,
    pub age: u32,
}
```

### Mistake 3: Broken doc tests

```rust
// ❌ Doc test that doesn't compile
/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 6);  // Wrong expected value!
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// ✅ Correct doc test
/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Mistake 4: Unhelpful examples

```rust
// ❌ Example doesn't show real usage
/// # Examples
/// ```
/// add(1, 1);
/// ```

// ✅ Example shows expected usage
/// # Examples
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
```

### Mistake 5: Missing error documentation

```rust
// ❌ No error documentation
/// Parses a number.
pub fn parse(s: &str) -> Result<i32, Error> {
    s.parse()
}

// ✅ Error documentation
/// Parses a number.
///
/// # Errors
/// Returns error if string is not a valid number.
pub fn parse(s: &str) -> Result<i32, Error> {
    s.parse()
}
```

## Real-World Examples

### Example 1: Well-documented function

```rust
/// Calculates the greatest common divisor of two numbers.
///
/// Uses the Euclidean algorithm for efficient computation.
///
/// # Arguments
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
/// The GCD of a and b
///
/// # Examples
/// ```
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(13, 7), 1);
/// ```
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
```

### Example 2: Documented struct

```rust
/// Represents a rectangular area.
///
/// Rectangle is defined by width and height.
#[derive(Debug, Clone)]
pub struct Rectangle {
    /// Width in units
    pub width: f64,
    /// Height in units
    pub height: f64,
}

impl Rectangle {
    /// Creates a new rectangle.
    pub fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }
    }

    /// Calculates the area.
    ///
    /// # Examples
    /// ```
    /// let r = Rectangle::new(10.0, 5.0);
    /// assert_eq!(r.area(), 50.0);
    /// ```
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

### Example 3: Module documentation

```rust
//! Math utilities for common calculations.
//!
//! This module provides functions for GCD, LCM, and other
//! mathematical operations.
//!
//! # Examples
//!
//! Computing GCD:
//! ```
//! use math_utils::gcd;
//! assert_eq!(gcd(48, 18), 6);
//! ```

/// Greatest common divisor
pub fn gcd(a: u32, b: u32) -> u32 { /* ... */ }

/// Least common multiple
pub fn lcm(a: u32, b: u32) -> u32 { /* ... */ }
```

## Related Concepts

### Prerequisites
- **Functions** - Documenting functions
- **Structs/Enums** - Documenting types
- **Modules** - Module-level documentation

### What comes next
- **Testing** - Doc tests as executable documentation
- **Code Quality** - Documentation as quality metric
- **Publishing** - Documentation on docs.rs

### Cross-references
- Module 02: Trait documentation
- Module 03: Testing with doc tests
- cargo doc tool

## Best Practices

### Document public API

```rust
// ✅ Good: documents public items
/// Public function that should be documented
pub fn public_function() { }

// Internal implementation, less critical
fn internal_helper() { }
```

### Write from user perspective

```rust
// ✅ User-focused
/// Removes leading and trailing whitespace from a string.
pub fn trim_string(s: &str) -> &str { }

// ❌ Implementation-focused
/// This function calls the trim method
pub fn trim_string(s: &str) -> &str { }
```

### Include examples for complex functions

```rust
// ✅ Example helps understanding
/// # Examples
/// ```
/// let result = complex_calculation(5);
/// ```
pub fn complex_calculation(n: i32) -> i32 { }

// ⚠️ No example for complex function
pub fn complex_calculation(n: i32) -> i32 { }
```

### Keep documentation current

Update documentation when code changes:

```rust
// When function behavior changes, update docs
/// Calculates X using algorithm Y
/// (Previously used algorithm Z)
pub fn calculate() { }
```

## Summary

- **Doc comments** use `///` for item documentation
- **Doc tests** are executable examples that get tested
- **Sections** like Arguments, Returns, Errors improve clarity
- **Markdown** support enables rich formatting
- **cargo doc** generates HTML documentation
- **Examples** help users understand usage
- **Module docs** use `//!` for crate/module-level
- **Documentation** is part of the API contract

## Key Takeaways

1. Use `///` for item documentation
2. Include examples with `# Examples` section
3. Document errors with `# Errors` section
4. Doc tests keep examples current
5. cargo doc generates browsable HTML
6. Documentation is part of public API
7. Well-documented code is more maintainable
8. Examples should be runnable

## Practice Exercise Ideas

1. Write comprehensive documentation for a function
2. Add doc tests that verify behavior
3. Document a struct with fields
4. Create module-level documentation
5. Write error documentation
6. Use cargo doc to view generated docs
7. Fix broken doc tests
8. Document edge cases and panics

---

**Time to complete this concept**: 1.5-2 hours
**Difficulty**: Beginner-Intermediate
**Prerequisite**: Functions, Structs, Modules
**Next concept**: Code Quality Tools

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
