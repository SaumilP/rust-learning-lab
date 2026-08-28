// Example: Running Code in Documentation (Doc Tests)
//
// Demonstrates:
// - Writing testable code examples in docs
// - Doc test attributes (ignore, should_panic, no_run, compile_fail)
// - Hiding boilerplate in doc tests
// - Testing async code in docs

fn main() {
    println!("This file demonstrates doc tests - code examples that are tested.");
    println!("Run doc tests with: rustdoc --test doc_tests.rs");
    println!("Or in cargo: cargo test --doc");
    println!();

    // Demo the functions
    println!("=== Demo ===");
    println!("factorial(5) = {}", factorial(5));
    println!("is_palindrome(\"racecar\") = {}", is_palindrome("racecar"));
}

/// Calculates the factorial of a number.
///
/// The factorial of n (written as n!) is the product of all positive
/// integers less than or equal to n.
///
/// # Examples
///
/// Basic usage - this code is automatically tested:
///
/// ```
/// let result = factorial(5);
/// assert_eq!(result, 120);  // 5! = 5 * 4 * 3 * 2 * 1 = 120
/// ```
///
/// Edge cases:
///
/// ```
/// assert_eq!(factorial(0), 1);  // 0! = 1 by definition
/// assert_eq!(factorial(1), 1);  // 1! = 1
/// ```
///
/// # Panics
///
/// Panics if the result would overflow a `u64`.
///
/// ```should_panic
/// factorial(100);  // This will overflow
/// ```
pub fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n.checked_mul(factorial(n - 1)).expect("Factorial overflow"),
    }
}

/// Checks if a string is a palindrome.
///
/// A palindrome reads the same forwards and backwards.
/// This function is case-insensitive and ignores non-alphanumeric characters.
///
/// # Examples
///
/// Simple palindrome:
///
/// ```
/// assert!(is_palindrome("racecar"));
/// assert!(is_palindrome("A"));
/// assert!(is_palindrome(""));
/// ```
///
/// Case insensitivity:
///
/// ```
/// assert!(is_palindrome("RaceCar"));
/// assert!(is_palindrome("Madam"));
/// ```
///
/// Ignoring punctuation and spaces:
///
/// ```
/// assert!(is_palindrome("A man, a plan, a canal: Panama"));
/// assert!(is_palindrome("Was it a car or a cat I saw?"));
/// ```
///
/// Non-palindromes:
///
/// ```
/// assert!(!is_palindrome("hello"));
/// assert!(!is_palindrome("world"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    cleaned == cleaned.chars().rev().collect::<String>()
}

/// Parses a positive integer from a string.
///
/// # Examples
///
/// Successful parsing:
///
/// ```
/// let result = parse_positive("42");
/// assert_eq!(result, Ok(42));
/// ```
///
/// Error cases:
///
/// ```
/// let result = parse_positive("-5");
/// assert!(result.is_err());
///
/// let result = parse_positive("not a number");
/// assert!(result.is_err());
/// ```
///
/// Using with error handling:
///
/// ```
/// fn process(input: &str) -> Result<u32, String> {
///     let num = parse_positive(input)?;
///     Ok(num * 2)
/// }
///
/// assert_eq!(process("21"), Ok(42));
/// ```
pub fn parse_positive(s: &str) -> Result<u32, String> {
    let num: i32 = s.parse().map_err(|_| format!("Cannot parse: {}", s))?;
    if num < 0 {
        return Err(format!("Negative number: {}", num));
    }
    Ok(num as u32)
}

/// A simple counter that can be incremented and decremented.
///
/// # Examples
///
/// Creating and using a counter:
///
/// ```
/// let mut counter = Counter::new();
/// assert_eq!(counter.value(), 0);
///
/// counter.increment();
/// counter.increment();
/// assert_eq!(counter.value(), 2);
///
/// counter.decrement();
/// assert_eq!(counter.value(), 1);
/// ```
///
/// Starting with an initial value:
///
/// ```
/// let counter = Counter::with_value(10);
/// assert_eq!(counter.value(), 10);
/// ```
#[derive(Debug)]
pub struct Counter {
    value: i32,
}

impl Counter {
    /// Creates a new counter starting at zero.
    ///
    /// # Examples
    ///
    /// ```
    /// let counter = Counter::new();
    /// assert_eq!(counter.value(), 0);
    /// ```
    pub fn new() -> Self {
        Counter { value: 0 }
    }

    /// Creates a counter with the specified initial value.
    ///
    /// # Examples
    ///
    /// ```
    /// let counter = Counter::with_value(42);
    /// assert_eq!(counter.value(), 42);
    /// ```
    pub fn with_value(value: i32) -> Self {
        Counter { value }
    }

    /// Returns the current value.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Increments the counter by one.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut c = Counter::new();
    /// c.increment();
    /// assert_eq!(c.value(), 1);
    /// ```
    pub fn increment(&mut self) {
        self.value += 1;
    }

    /// Decrements the counter by one.
    pub fn decrement(&mut self) {
        self.value -= 1;
    }
}

/// Reads a file and returns its contents.
///
/// # Examples
///
/// This example uses `no_run` because it requires a real file:
///
/// ```no_run
/// let contents = read_file("config.txt");
/// println!("File contents: {:?}", contents);
/// ```
///
/// # Errors
///
/// Returns an error if the file cannot be read.
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// Connects to a server (simulated).
///
/// # Examples
///
/// This example is marked `ignore` because it requires network access:
///
/// ```ignore
/// let connection = connect("localhost:8080")?;
/// connection.send("Hello");
/// ```
pub fn connect(_addr: &str) -> Result<(), String> {
    // Simulated connection
    Ok(())
}

/// Demonstrates compile_fail attribute.
///
/// The following code would not compile because you can't add
/// incompatible types:
///
/// ```compile_fail
/// let x: i32 = 5;
/// let y: &str = "hello";
/// let z = x + y;  // Error: cannot add i32 and &str
/// ```
///
/// This is useful for documenting what NOT to do.
pub fn type_safety_demo() {
    // This function exists just for documentation
}

/// Demonstrates hiding lines in doc tests.
///
/// Sometimes you need setup code that isn't relevant to the example.
/// Lines starting with `#` are hidden but still compiled.
///
/// # Examples
///
/// ```
/// # // This line is hidden
/// # fn setup() -> Vec<i32> {
/// #     vec![1, 2, 3, 4, 5]
/// # }
/// # let data = setup();
/// // The user sees this:
/// let sum: i32 = data.iter().sum();
/// assert_eq!(sum, 15);
/// ```
///
/// Another example with hidden main:
///
/// ```
/// # fn main() -> Result<(), String> {
/// let result = parse_positive("42")?;
/// assert_eq!(result, 42);
/// # Ok(())
/// # }
/// ```
pub fn hidden_lines_demo() {
    // This function exists for documentation
}

/// A configuration builder with multiple examples.
///
/// # Examples
///
/// ## Basic Configuration
///
/// ```
/// let config = ConfigBuilder::new()
///     .name("MyApp")
///     .build();
///
/// assert_eq!(config.name, "MyApp");
/// assert_eq!(config.timeout, 30);  // default
/// ```
///
/// ## Custom Timeout
///
/// ```
/// let config = ConfigBuilder::new()
///     .name("Server")
///     .timeout(60)
///     .build();
///
/// assert_eq!(config.timeout, 60);
/// ```
///
/// ## Full Configuration
///
/// ```
/// let config = ConfigBuilder::new()
///     .name("Production")
///     .timeout(120)
///     .retries(5)
///     .build();
///
/// assert_eq!(config.name, "Production");
/// assert_eq!(config.timeout, 120);
/// assert_eq!(config.retries, 5);
/// ```
pub struct ConfigBuilder {
    name: String,
    timeout: u32,
    retries: u32,
}

pub struct BuiltConfig {
    pub name: String,
    pub timeout: u32,
    pub retries: u32,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder {
            name: String::new(),
            timeout: 30,
            retries: 3,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn timeout(mut self, seconds: u32) -> Self {
        self.timeout = seconds;
        self
    }

    pub fn retries(mut self, count: u32) -> Self {
        self.retries = count;
        self
    }

    pub fn build(self) -> BuiltConfig {
        BuiltConfig {
            name: self.name,
            timeout: self.timeout,
            retries: self.retries,
        }
    }
}
