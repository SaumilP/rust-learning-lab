# Key Takeaways: Documentation

## Quick Reference

### Doc Comment Syntax

```rust
/// Documentation for item
pub fn function() { }

//! Documentation for module/crate
```

### Sections

```rust
/// Brief description.
///
/// Longer description here.
///
/// # Arguments
/// * `param` - Description
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
/// assert_eq!(function(), expected);
/// ```
pub fn function() { }
```

### Doc Test

```rust
/// # Examples
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

### Generate Docs

```bash
cargo doc --open
cargo test --doc
```

## Essential Concepts

### 1. Doc Comments

```rust
// Three slashes: documentation
/// This documents the item below
pub fn function() { }

// Exclamation: documents container
//! This documents the module/crate
```

### 2. Common Sections

| Section | Use | Example |
|---------|-----|---------|
| Brief | Summary | "Adds two numbers" |
| Description | Details | Explain algorithm, edge cases |
| `# Arguments` | Parameters | What each param is |
| `# Returns` | Return value | What's returned |
| `# Errors` | Error cases | When/what errors |
| `# Panics` | Panic cases | When panics |
| `# Examples` | Usage code | Runnable examples |
| `# Safety` | Unsafe | Safety requirements |

### 3. Doc Tests

```rust
/// # Examples
/// ```
/// let result = function();
/// assert_eq!(result, expected);
/// ```
```

**Running:**
```bash
cargo test --doc
```

**Ignoring:**
```rust
/// ```ignore
/// // Not tested
/// ```
```

**Hiding setup:**
```rust
/// ```
/// # fn setup() {}
/// # setup();
/// // Only shown
/// assert_eq!(2 + 2, 4);
/// ```
```

### 4. Markdown Support

```rust
/// **Bold text**
/// *Italic text*
/// `code inline`
/// [link](url)
///
/// ```rust
/// code block
/// ```
///
/// # Heading
/// ## Subheading
///
/// * List item 1
/// * List item 2
pub fn function() { }
```

### 5. Referencing Items

```rust
/// See [`add`] for addition.
/// See [`User`] struct for details.
pub fn multiply() { }
```

### 6. Struct Documentation

```rust
/// Represents a user.
pub struct User {
    /// User's ID
    pub id: u32,
    /// User's name
    pub name: String,
}
```

### 7. Module Documentation

```rust
//! Utility functions for string handling.
//!
//! This module provides common string operations.

/// Reverses a string
pub fn reverse(s: &str) -> String { }
```

## Common Patterns

### Pattern 1: Simple function
```rust
/// Doubles a number.
pub fn double(x: i32) -> i32 { x * 2 }
```

### Pattern 2: With arguments and return
```rust
/// Adds two numbers.
///
/// # Arguments
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
/// The sum of a and b
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

### Pattern 3: With example
```rust
/// # Examples
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

### Pattern 4: With errors
```rust
/// # Errors
/// Returns error if file doesn't exist
pub fn read_file(path: &str) -> Result<String> { }
```

### Pattern 5: With panics
```rust
/// # Panics
/// Panics if index >= length
pub fn get_unchecked(arr: &[i32], i: usize) -> i32 { }
```

### Pattern 6: Struct fields
```rust
/// A point in space.
pub struct Point {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}
```

### Pattern 7: Module docs
```rust
//! Math utilities.
//!
//! Provides GCD, LCM, and related functions.

pub fn gcd(a: u32, b: u32) -> u32 { }
```

### Pattern 8: Complex example
```rust
/// # Examples
/// ```
/// let mut v = vec![1, 2, 3];
/// v.push(4);
/// assert_eq!(v.len(), 4);
/// ```
pub fn process(v: &mut Vec<i32>) { }
```

## Checklist: Good Documentation

- [ ] Brief description present
- [ ] Arguments documented
- [ ] Return value documented
- [ ] Errors documented (if applicable)
- [ ] Panics documented (if applicable)
- [ ] Examples included
- [ ] Examples are runnable (doc tests)
- [ ] Public API fully documented

## Error Prevention

### ❌ DON'T: No documentation
```rust
pub fn process(input: &str) -> String { }
```

### ✅ DO: Comprehensive documentation
```rust
/// Processes input text.
///
/// # Examples
/// ```
/// assert_eq!(process("hello"), "HELLO");
/// ```
pub fn process(input: &str) -> String { }
```

### ❌ DON'T: Broken doc test
```rust
/// # Examples
/// ```
/// assert_eq!(add(2, 3), 6);  // Wrong!
/// ```
```

### ✅ DO: Correct doc test
```rust
/// # Examples
/// ```
/// assert_eq!(add(2, 3), 5);
/// ```
```

### ❌ DON'T: Incomplete sections
```rust
/// Does something
///
/// # Arguments
/// * `x` - Something
/// * `y` - Something
pub fn func(x: i32, y: i32) -> Result<i32> { }
```

### ✅ DO: Complete sections
```rust
/// Does something
///
/// # Arguments
/// * `x` - First number
/// * `y` - Second number
///
/// # Returns
/// The result
///
/// # Errors
/// If calculation fails
pub fn func(x: i32, y: i32) -> Result<i32> { }
```

### ❌ DON'T: Unhelpful example
```rust
/// # Examples
/// ```
/// process(input);
/// ```
```

### ✅ DO: Clear, complete example
```rust
/// # Examples
/// ```
/// let result = process("test");
/// assert_eq!(result, "TEST");
/// ```
```

### ❌ DON'T: No error documentation
```rust
/// Parses a number.
pub fn parse(s: &str) -> Result<i32, Error> { }
```

### ✅ DO: Document errors
```rust
/// Parses a number.
///
/// # Errors
/// Returns error if string is not a valid integer.
pub fn parse(s: &str) -> Result<i32, Error> { }
```

## Documentation Commands

| Command | Purpose |
|---------|---------|
| `cargo doc` | Generate docs (HTML) |
| `cargo doc --open` | Generate and open browser |
| `cargo doc --document-private-items` | Include private items |
| `cargo test --doc` | Run doc tests |
| `cargo test --doc test_name` | Run specific doc test |

## Sections Quick Reference

```rust
/// One-line summary.
///
/// Longer description explaining behavior,
/// algorithm, use cases, etc.
///
/// # Arguments
/// * `param1` - What it does
/// * `param2` - What it does
///
/// # Returns
/// What is returned and when
///
/// # Errors
/// When errors occur and why
///
/// # Panics
/// When panics occur (rare)
///
/// # Safety
/// Safety requirements (if unsafe)
///
/// # Examples
/// ```
/// // Example code
/// assert!(verify());
/// ```
pub fn function() { }
```

## Markdown Formatting

```rust
/// **Bold text**
/// *Italic text*
/// `inline code`
/// [Link text](https://example.com)
///
/// ```rust
/// fn code_block() {
///     println!("Example");
/// }
/// ```
///
/// # Heading
/// ## Subheading
///
/// - List item 1
/// - List item 2
/// - List item 3
pub fn function() { }
```

## Related Concepts

- **Functions** - Most documented items
- **Testing** - Doc tests verify examples
- **Modules** - Module-level documentation
- **Public API** - What gets documented

## Time Estimates

- Reading this takeaway: 10-15 minutes
- Reviewing patterns: 10 minutes
- Practice drills: 20-30 minutes
- Total: 40-55 minutes

## Practice Questions

1. What's the syntax for doc comments?
2. What are the main documentation sections?
3. How do doc tests work?
4. How do you run doc tests?
5. What's the difference between /// and //!?
6. How do you hide code in doc tests?
7. How do you reference other items?
8. What Markdown features are supported?

## Documentation Best Practices

**Public API must be documented:**
```rust
/// Document all public items
pub fn public_func() { }

// Private items are optional
fn private_helper() { }
```

**User-focused language:**
```rust
/// Removes whitespace from start and end
pub fn trim_string(s: &str) -> &str { }
```

**Examples should be complete:**
```rust
/// # Examples
/// ```
/// use my_crate::function;
/// let result = function("input");
/// assert_eq!(result, "expected");
/// ```
pub fn function(s: &str) -> String { }
```

**Keep docs current:**
- Update docs when code changes
- Run doc tests regularly
- Use `cargo doc` to preview

## Tools

| Tool | Purpose |
|------|---------|
| `cargo doc` | Generate HTML documentation |
| `cargo test --doc` | Test examples in docs |
| `cargo clippy` | Lint checks including docs |
| `docs.rs` | Host on-line documentation |

---

**Status**: Quick reference guide
**Importance**: ⭐⭐⭐⭐ (Important)
**Difficulty**: Beginner-Intermediate
**Part of**: Module 03 - Tooling and Quality
