//! # Documentation Example Module
//!
//! This is a module-level documentation comment using `//!`.
//! It documents the entire file/module.
//!
//! ## Purpose
//!
//! This module demonstrates how to write documentation in Rust.
//!
//! ## Usage
//!
//! Run `rustdoc doc_comments.rs` to generate the documentation.

// Example: Documentation Comments with ///
//
// Demonstrates:
// - Doc comment syntax (///, //!)
// - Common doc sections (Examples, Panics, Errors, Safety)
// - Markdown formatting in docs
// - Generating documentation with cargo doc

fn main() {
    println!("This file demonstrates documentation comments.");
    println!("Generate docs with: rustdoc doc_comments.rs");
    println!("Or in a cargo project: cargo doc --open");
    println!();

    // Demo the documented functions
    let result = add(2, 3);
    println!("add(2, 3) = {}", result);

    let greeting = greet("World");
    println!("greet(\"World\") = \"{}\"", greeting);

    let user = User::new("Alice", "alice@example.com");
    println!("Created user: {} <{}>", user.name(), user.email());
}

/// Adds two numbers together.
///
/// This function takes two `i32` values and returns their sum.
/// It performs standard integer addition.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// ```
/// // Works with negative numbers too
/// let result = add(-5, 10);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts the second number from the first.
///
/// # Arguments
///
/// * `a` - The number to subtract from (minuend)
/// * `b` - The number to subtract (subtrahend)
///
/// # Returns
///
/// The difference between `a` and `b`.
///
/// # Examples
///
/// ```
/// let result = subtract(10, 3);
/// assert_eq!(result, 7);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Divides two numbers.
///
/// This function performs integer division, truncating any remainder.
///
/// # Arguments
///
/// * `dividend` - The number being divided
/// * `divisor` - The number to divide by
///
/// # Panics
///
/// Panics if `divisor` is zero.
///
/// # Examples
///
/// ```
/// let result = divide(10, 3);
/// assert_eq!(result, 3);
/// ```
///
/// ```should_panic
/// // This will panic
/// divide(10, 0);
/// ```
pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("Cannot divide by zero!");
    }
    dividend / divisor
}

/// Safely divides two numbers.
///
/// Unlike [`divide`], this function returns an error instead of panicking
/// when division by zero is attempted.
///
/// # Arguments
///
/// * `dividend` - The number being divided
/// * `divisor` - The number to divide by
///
/// # Errors
///
/// Returns `Err` with an error message if `divisor` is zero.
///
/// # Examples
///
/// Successful division:
///
/// ```
/// let result = safe_divide(10, 2);
/// assert_eq!(result, Ok(5));
/// ```
///
/// Division by zero:
///
/// ```
/// let result = safe_divide(10, 0);
/// assert!(result.is_err());
/// ```
pub fn safe_divide(dividend: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(dividend / divisor)
    }
}

/// Creates a greeting message.
///
/// Formats a friendly greeting for the given name.
///
/// # Arguments
///
/// * `name` - The name of the person to greet
///
/// # Returns
///
/// A `String` containing the greeting message.
///
/// # Examples
///
/// ```
/// let greeting = greet("Alice");
/// assert_eq!(greeting, "Hello, Alice!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Finds the maximum of two values.
///
/// Uses comparison to determine which value is larger.
///
/// # Type Parameters
///
/// * `T` - Any type that implements [`PartialOrd`]
///
/// # Examples
///
/// ```
/// assert_eq!(max(5, 10), 10);
/// assert_eq!(max('a', 'z'), 'z');
/// ```
pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

/// Represents a user in the system.
///
/// Users have a name and email address that are set at creation
/// and can be accessed but not modified.
///
/// # Examples
///
/// Creating a new user:
///
/// ```
/// let user = User::new("Alice", "alice@example.com");
/// assert_eq!(user.name(), "Alice");
/// assert_eq!(user.email(), "alice@example.com");
/// ```
#[derive(Debug, Clone)]
pub struct User {
    /// The user's display name.
    name: String,
    /// The user's email address.
    email: String,
}

impl User {
    /// Creates a new user with the given name and email.
    ///
    /// # Arguments
    ///
    /// * `name` - The user's display name
    /// * `email` - The user's email address
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User::new("Bob", "bob@example.com");
    /// ```
    pub fn new(name: &str, email: &str) -> Self {
        User {
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    /// Returns the user's name.
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User::new("Alice", "alice@example.com");
    /// assert_eq!(user.name(), "Alice");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the user's email address.
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User::new("Alice", "alice@example.com");
    /// assert_eq!(user.email(), "alice@example.com");
    /// ```
    pub fn email(&self) -> &str {
        &self.email
    }
}

/// Represents possible errors in the application.
///
/// This enum covers all error cases that can occur during
/// normal operation.
///
/// # Variants
///
/// * `NotFound` - The requested resource was not found
/// * `InvalidInput` - The provided input was invalid
/// * `NetworkError` - A network operation failed
#[derive(Debug)]
pub enum AppError {
    /// The requested resource could not be found.
    ///
    /// Contains a description of what was not found.
    NotFound(String),

    /// The provided input was invalid.
    ///
    /// Contains details about why the input was rejected.
    InvalidInput(String),

    /// A network operation failed.
    ///
    /// Contains the underlying error message.
    NetworkError(String),
}

/// Configuration options for the application.
///
/// Use the builder pattern to construct instances:
///
/// # Examples
///
/// ```
/// let config = Config::default()
///     .with_timeout(60)
///     .with_retries(3);
/// ```
#[derive(Debug, Default)]
pub struct Config {
    /// Connection timeout in seconds.
    timeout: u32,
    /// Number of retry attempts.
    retries: u32,
}

impl Config {
    /// Sets the connection timeout.
    ///
    /// # Arguments
    ///
    /// * `seconds` - Timeout duration in seconds
    pub fn with_timeout(mut self, seconds: u32) -> Self {
        self.timeout = seconds;
        self
    }

    /// Sets the number of retry attempts.
    ///
    /// # Arguments
    ///
    /// * `count` - Number of retries (0 means no retries)
    pub fn with_retries(mut self, count: u32) -> Self {
        self.retries = count;
        self
    }
}
