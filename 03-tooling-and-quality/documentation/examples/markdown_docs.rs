//! # Markdown Documentation Guide
//!
//! This module demonstrates Markdown formatting available in Rust documentation.
//!
//! ## Topics
//!
//! - Headers and text formatting
//! - Lists and tables
//! - Code blocks and links
//!
//! ## Quick Start
//!
//! ```rust
//! let result = 2 + 3;
//! assert_eq!(result, 5);
//! ```

// Example: Using Markdown in Documentation
//
// Demonstrates:
// - Markdown formatting in Rust docs
// - Headers, lists, code blocks
// - Links and cross-references
// - Tables and emphasis

fn main() {
    println!("This file demonstrates Markdown formatting in docs.");
    println!("Generate docs with: rustdoc markdown_docs.rs --output docs");
    println!("Or in cargo: cargo doc --open");
}

/// # Headers in Documentation
///
/// Headers help organize documentation. Use `#` for headers:
///
/// ## Level 2 Header
///
/// ### Level 3 Header
///
/// #### Level 4 Header
///
/// # Usage
///
/// ```text
/// // Your code here
/// ```
///
/// # See Also
///
/// - [`subtract`] - For subtraction
/// - [`multiply`] - For multiplication
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// # Text Formatting Examples
///
/// Rust documentation supports various text formatting options:
///
/// ## Emphasis
///
/// - *Italic text* using single asterisks
/// - **Bold text** using double asterisks
/// - ***Bold and italic*** using triple asterisks
/// - `Inline code` using backticks
///
/// ## Special Text
///
/// > This is a blockquote.
/// > It can span multiple lines.
/// >
/// > And have multiple paragraphs.
///
/// ## Horizontal Rules
///
/// Use three or more dashes for a horizontal rule:
///
/// ---
///
/// Content continues after the rule.
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// # Lists in Documentation
///
/// ## Unordered Lists
///
/// - First item
/// - Second item
///   - Nested item
///   - Another nested item
/// - Third item
///
/// Or using asterisks:
///
/// * Item one
/// * Item two
/// * Item three
///
/// ## Ordered Lists
///
/// 1. First step
/// 2. Second step
/// 3. Third step
///    1. Sub-step A
///    2. Sub-step B
///
/// ## Task Lists
///
/// - [x] Completed task
/// - [ ] Incomplete task
/// - [ ] Another task
///
/// ## Definition-Style Lists
///
/// **Term 1**
/// : Definition for term 1
///
/// **Term 2**
/// : Definition for term 2
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// # Code Blocks in Documentation
///
/// ## Inline Code
///
/// Use `backticks` for inline code like `let x = 5;` or type names like `Option<T>`.
///
/// ## Fenced Code Blocks
///
/// Rust code (with syntax highlighting):
///
/// ```rust
/// fn example() {
///     let x = 5;
///     let y = 10;
///     println!("Sum: {}", x + y);
/// }
/// ```
///
/// Without language specifier:
///
/// ```
/// This is a plain code block
/// without syntax highlighting
/// ```
///
/// ## Other Languages
///
/// JSON:
///
/// ```json
/// {
///     "name": "example",
///     "version": "1.0.0"
/// }
/// ```
///
/// Shell commands:
///
/// ```bash
/// $ cargo build
/// $ cargo test
/// $ cargo doc --open
/// ```
///
/// ## Text Output
///
/// Use `text` for non-code output:
///
/// ```text
/// Output:
///   Line 1
///   Line 2
///   Line 3
/// ```
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

/// # Links and References
///
/// ## Inline Links
///
/// Visit the [Rust website](https://www.rust-lang.org/) for more information.
///
/// ## Reference-Style Links
///
/// Check out the [Rust Book][book] for tutorials.
///
/// [book]: https://doc.rust-lang.org/book/
///
/// ## Internal Links (Cross-References)
///
/// Link to other items in your crate:
///
/// - See [`add`] for addition
/// - See [`Calculator`] struct
/// - See [`Calculator::new`] method
/// - See [`AppError`] enum
///
/// ## Full Path Links
///
/// - [`std::vec::Vec`]
/// - [`std::collections::HashMap`]
/// - [`std::io::Result`]
///
/// ## Automatic Links
///
/// Use angle brackets for automatic links: <https://docs.rs>
pub fn modulo(a: i32, b: i32) -> i32 {
    a % b
}

/// # Tables in Documentation
///
/// Tables use pipes and dashes:
///
/// | Operation | Symbol | Example |
/// |-----------|--------|---------|
/// | Add       | `+`    | `a + b` |
/// | Subtract  | `-`    | `a - b` |
/// | Multiply  | `*`    | `a * b` |
/// | Divide    | `/`    | `a / b` |
///
/// ## Alignment
///
/// | Left | Center | Right |
/// |:-----|:------:|------:|
/// | L1   | C1     | R1    |
/// | L2   | C2     | R2    |
/// | L3   | C3     | R3    |
///
/// ## Complex Table
///
/// | Type | Size | Range |
/// |------|------|-------|
/// | `i8` | 1 byte | -128 to 127 |
/// | `i16` | 2 bytes | -32,768 to 32,767 |
/// | `i32` | 4 bytes | -2^31 to 2^31-1 |
/// | `i64` | 8 bytes | -2^63 to 2^63-1 |
pub fn power(base: i32, exp: u32) -> i32 {
    base.pow(exp)
}

/// A calculator with documented methods.
///
/// # Overview
///
/// `Calculator` provides basic arithmetic operations with a clean API.
///
/// # Architecture
///
/// ```text
///  +------------+
///  | Calculator |
///  +------------+
///        |
///        +-- add()
///        +-- subtract()
///        +-- multiply()
///        +-- divide()
/// ```
///
/// # Examples
///
/// ## Basic Usage
///
/// ```
/// let calc = Calculator::new();
/// let result = calc.add(5, 3);
/// assert_eq!(result, 8);
/// ```
///
/// ## Chaining Operations
///
/// ```
/// let calc = Calculator::new();
/// let a = calc.add(10, 5);      // 15
/// let b = calc.multiply(a, 2);   // 30
/// let c = calc.subtract(b, 10);  // 20
/// assert_eq!(c, 20);
/// ```
///
/// # Performance
///
/// All operations are O(1) constant time.
///
/// # Thread Safety
///
/// `Calculator` is `Send` and `Sync`, safe for use across threads.
pub struct Calculator;

impl Calculator {
    /// Creates a new calculator instance.
    pub fn new() -> Self {
        Calculator
    }

    /// Adds two numbers.
    pub fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    /// Subtracts b from a.
    pub fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    /// Multiplies two numbers.
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    /// Divides a by b.
    pub fn divide(&self, a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
}

/// # Error Types
///
/// This enum represents all possible errors.
///
/// ## Variants
///
/// | Variant | Description | Recovery |
/// |---------|-------------|----------|
/// | `NotFound` | Item missing | Retry or create |
/// | `InvalidInput` | Bad data | Validate input |
/// | `NetworkError` | Connection issue | Retry later |
///
/// ## Error Handling
///
/// ```rust
/// fn process() -> Result<(), AppError> {
///     // ... processing ...
///     Ok(())
/// }
///
/// match process() {
///     Ok(()) => println!("Success!"),
///     Err(AppError::NotFound(msg)) => eprintln!("Not found: {}", msg),
///     Err(AppError::InvalidInput(msg)) => eprintln!("Invalid: {}", msg),
///     Err(AppError::NetworkError(msg)) => eprintln!("Network: {}", msg),
/// }
/// ```
#[derive(Debug)]
pub enum AppError {
    /// Resource not found.
    NotFound(String),
    /// Invalid input provided.
    InvalidInput(String),
    /// Network operation failed.
    NetworkError(String),
}
