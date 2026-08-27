# Concept: Error Handling Basics

## Overview

Error handling is a critical aspect of robust systems. Unlike many languages that use exceptions, Rust uses types to represent success and failure: `Option<T>` for optional values and `Result<T, E>` for operations that might fail. This approach makes errors explicit in the type system and ensures they're handled appropriately. This concept covers the fundamentals of error handling in Rust.

## Learning Objectives

By the end of this concept, you will understand:
- The difference between `Option<T>` and `Result<T, E>`
- How to create and use Result types
- Pattern matching on Results
- The `?` operator for error propagation
- Common error handling patterns
- When to use panic vs Result
- Creating custom error types

## Theory

### Why Rust's Approach?

Traditional exception handling (try/catch) has issues:
- Exceptions are implicit in function signatures
- Easy to ignore errors accidentally
- Control flow based on exceptions is unclear
- Performance overhead

Rust's approach with `Result<T, E>` is better because:
- Errors are explicit in the type signature
- Compiler forces you to handle errors
- Zero-cost abstraction
- Clear control flow

### Option<T>

`Option<T>` represents a value that may or may not exist:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

**Creating Options:**

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

// Vec methods return Option
let v = vec![1, 2, 3];
let first = v.get(0);  // Some(&1)
let out_of_bounds = v.get(10);  // None
```

**Matching Options:**

```rust
let x: Option<i32> = Some(5);

match x {
    Some(value) => println!("Value: {}", value),
    None => println!("No value"),
}
```

**Option Methods:**

```rust
let x = Some(5);

x.is_some();           // true
x.is_none();           // false
x.unwrap();            // 5 (panics if None)
x.unwrap_or(0);        // 5 (default to 0 if None)
x.unwrap_or_else(|| calculate_default());  // Custom default
x.map(|v| v * 2);      // Transform: Some(10)
x.and_then(|v| Some(v * 2));  // Chain operations

// Extracting value safely
if let Some(value) = x {
    println!("Value: {}", value);
}
```

### Result<T, E>

`Result<T, E>` represents either success (Ok) or failure (Err):

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**Creating Results:**

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

let result = divide(10, 2);  // Ok(5)
let error = divide(10, 0);   // Err("Division by zero")
```

**Matching Results:**

```rust
match divide(10, 2) {
    Ok(value) => println!("Result: {}", value),
    Err(error) => println!("Error: {}", error),
}
```

**Result Methods:**

```rust
let result: Result<i32, String> = Ok(5);

result.is_ok();              // true
result.is_err();             // false
result.ok();                 // Some(5)
result.err();                // None
result.unwrap();             // 5 (panics if Err)
result.unwrap_or(0);         // 5 (default if Err)
result.unwrap_err();         // Err value
result.map(|v| v * 2);       // Transform: Ok(10)
result.map_err(|e| format!("Error: {}", e));  // Transform error
result.and_then(|v| Ok(v * 2));  // Chain operations
```

### Pattern Matching with if let

Simpler than match for single pattern:

```rust
// With match
match get_value() {
    Some(v) => println!("Value: {}", v),
    None => {}
}

// With if let
if let Some(v) = get_value() {
    println!("Value: {}", v);
}

// if let with Result
if let Ok(value) = divide(10, 2) {
    println!("Result: {}", value);
} else {
    println!("Division failed");
}
```

### The ? Operator (Error Propagation)

The `?` operator automatically propagates errors up the call stack:

```rust
// Without ?
fn read_file(path: &str) -> Result<String, std::io::Error> {
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

// With ? (much cleaner)
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)?
}

// Chaining multiple operations
fn process_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    let uppercase = content.to_uppercase();
    Ok(uppercase)
}
```

**How ? works:**
- If `Result` is `Ok`, extracts the value
- If `Result` is `Err`, returns error immediately
- Can only be used in functions returning `Result` or `Option`

### panic! vs Result

**panic!** - Use only for unrecoverable errors

```rust
// Reasonable: programming error
assert!(x > 0, "x must be positive");

// Reasonable: precondition violation
if config.is_empty() {
    panic!("Configuration required");
}

// Bad: user input should use Result
if user_input.parse::<i32>().is_err() {
    panic!("Invalid number");  // ❌ Don't do this
}
```

**Result** - Use for recoverable errors

```rust
// Good: parse can fail
fn parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse()
}

// Good: file might not exist
fn read_config(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
```

### Custom Error Types

**Simple string error:**

```rust
fn validate_age(age: u32) -> Result<(), String> {
    if age < 0 {
        Err("Age cannot be negative".to_string())
    } else if age > 150 {
        Err("Age seems unrealistic".to_string())
    } else {
        Ok(())
    }
}
```

**Custom error struct:**

```rust
#[derive(Debug)]
struct ValidationError {
    field: String,
    message: String,
}

fn validate_age(age: u32) -> Result<(), ValidationError> {
    if age > 150 {
        Err(ValidationError {
            field: "age".to_string(),
            message: "Age seems unrealistic".to_string(),
        })
    } else {
        Ok(())
    }
}
```

**Custom error enum:**

```rust
#[derive(Debug)]
enum FileError {
    NotFound(String),
    PermissionDenied(String),
    IoError(String),
}

fn read_config(path: &str) -> Result<String, FileError> {
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(FileError::IoError(e.to_string())),
    }
}
```

## Syntax

### Option<T>

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

match x {
    Some(value) => { },
    None => { },
}

if let Some(value) = x { }

x.unwrap()
x.unwrap_or(default)
x.map(|v| f(v))
```

### Result<T, E>

```rust
let r: Result<i32, String> = Ok(5);
let e: Result<i32, String> = Err("error".to_string());

match r {
    Ok(value) => { },
    Err(error) => { },
}

if let Ok(value) = r { }

r.unwrap()
r.unwrap_or(default)
r.map(|v| f(v))
r.map_err(|e| f(e))
r?  // Propagate error
```

## Common Patterns

### Pattern 1: Simple unwrap with default

```rust
let value = risky_operation().unwrap_or(default_value);
```

### Pattern 2: Match on Result

```rust
match risky_operation() {
    Ok(value) => println!("Success: {}", value),
    Err(e) => println!("Error: {}", e),
}
```

### Pattern 3: Error propagation with ?

```rust
fn complex_operation() -> Result<String, std::io::Error> {
    let file_content = std::fs::read_to_string("file.txt")?;
    let processed = file_content.to_uppercase();
    Ok(processed)
}
```

### Pattern 4: if let for optional handling

```rust
if let Some(value) = maybe_value {
    println!("Got: {}", value);
} else {
    println!("No value");
}
```

### Pattern 5: Chain operations

```rust
let result = vec![1, 2, 3]
    .iter()
    .find(|&&x| x > 5)
    .ok_or("No matching element")?;
```

### Pattern 6: Transform errors

```rust
fn operation() -> Result<i32, String> {
    risky_operation()
        .map_err(|e| format!("Operation failed: {}", e))
}
```

### Pattern 7: Default behavior on error

```rust
let config = load_config()
    .unwrap_or_else(|e| {
        eprintln!("Config error: {}", e);
        Config::default()
    });
```

### Pattern 8: Multiple operations

```rust
fn process() -> Result<String, Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("input.txt")?;
    let number: i32 = file.trim().parse()?;
    let result = format!("Number: {}", number);
    Ok(result)
}
```

## Common Mistakes

### Mistake 1: Ignoring Result

```rust
// ❌ Silently ignores error
std::fs::read_to_string("file.txt");

// ✅ Handle the error
let content = std::fs::read_to_string("file.txt")?;
// or
let content = match std::fs::read_to_string("file.txt") {
    Ok(c) => c,
    Err(e) => panic!("Cannot read: {}", e),
};
```

### Mistake 2: Unwrapping without checking

```rust
// ❌ Panics if None
let v = vec![1, 2, 3];
let value = v.get(10).unwrap();

// ✅ Safe handling
let value = v.get(10).unwrap_or(&0);
```

### Mistake 3: ? without Result return

```rust
// ❌ Error: can't use ? in non-Result function
fn process(file: &str) {
    let content = std::fs::read_to_string(file)?;  // Error!
}

// ✅ Return Result
fn process(file: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file)?
}
```

### Mistake 4: Losing error information

```rust
// ❌ Loses original error
match risky_op() {
    Ok(v) => println!("{}", v),
    Err(_) => println!("Failed"),  // No error details
}

// ✅ Preserves error info
match risky_op() {
    Ok(v) => println!("{}", v),
    Err(e) => println!("Failed: {}", e),
}
```

### Mistake 5: Panic when error is expected

```rust
// ❌ Wrong: user input can fail
fn parse_age(input: &str) -> i32 {
    input.parse().unwrap()  // Panics on invalid input
}

// ✅ Right: return Result
fn parse_age(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.parse()
}
```

## Real-World Examples

### Example 1: Configuration loading

```rust
#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
}

fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    // Parse JSON/YAML...
    Ok(Config {
        host: "localhost".to_string(),
        port: 8080,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("config.toml")?;
    println!("Config loaded: {:?}", config);
    Ok(())
}
```

### Example 2: User input validation

```rust
fn validate_email(email: &str) -> Result<(), String> {
    if email.is_empty() {
        return Err("Email cannot be empty".to_string());
    }
    if !email.contains('@') {
        return Err("Email must contain @".to_string());
    }
    if !email.contains('.') {
        return Err("Email must contain domain".to_string());
    }
    Ok(())
}

fn register_user(email: &str) -> Result<(), String> {
    validate_email(email)?;
    // Register user...
    Ok(())
}
```

### Example 3: Chaining operations

```rust
fn read_and_parse(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let number = content.trim().parse()?;
    Ok(number)
}
```

### Example 4: Custom error handling

```rust
#[derive(Debug)]
enum DataError {
    FileNotFound(String),
    ParseError(String),
    InvalidData(String),
}

fn load_data(path: &str) -> Result<Vec<i32>, DataError> {
    let content = std::fs::read_to_string(path)
        .map_err(|_| DataError::FileNotFound(path.to_string()))?;

    let numbers: Vec<i32> = content
        .lines()
        .map(|line| line.parse::<i32>()
            .map_err(|_| DataError::ParseError(line.to_string())))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(numbers)
}
```

### Example 5: Graceful degradation

```rust
fn get_user_preference(key: &str) -> String {
    std::env::var(key)
        .unwrap_or_else(|_| {
            eprintln!("Warning: {} not set, using default", key);
            "default".to_string()
        })
}
```

## Related Concepts

### Prerequisites
- **Pattern Matching** - Used in error handling
- **Types** - Understanding Option and Result types
- **Functions** - Function return types

### What comes next
- **Custom Error Types** - Implementing error traits
- **Logging** - Recording errors for debugging
- **Testing** - Testing error paths
- **Advanced Error Handling** - anyhow, thiserror crates

### Cross-references
- Module 01: Pattern matching fundamentals
- Module 02: Collections return Options
- Module 06: Advanced error handling patterns

## Best Practices

### Use Result for fallible operations

```rust
// ✅ Good: function can fail
fn parse_config(file: &str) -> Result<Config, ConfigError> { }

// ❌ Bad: ignores possibility of failure
fn parse_config(file: &str) -> Config { }
```

### Propagate errors with ?

```rust
// ✅ Clean: errors propagate naturally
fn process_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.to_uppercase())
}

// ❌ Verbose: manual error handling
fn process_file(path: &str) -> Result<String, std::io::Error> {
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(content.to_uppercase()),
        Err(e) => Err(e),
    }
}
```

### Don't panic in libraries

```rust
// ✅ Good: return Result
pub fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}

// ❌ Bad: panics on invalid input
pub fn parse_int(s: &str) -> i32 {
    s.parse().unwrap()
}
```

### Use descriptive error types

```rust
// ✅ Good: error type provides context
enum ValidationError {
    InvalidEmail(String),
    InvalidAge(u32),
}

// ❌ Vague: loses information
fn validate(email: &str, age: u32) -> Result<(), String> {
    Err("Invalid".to_string())
}
```

## Summary

- **Option<T>** - Represents optional value (Some/None)
- **Result<T, E>** - Represents success/failure (Ok/Err)
- **? operator** - Automatically propagates errors
- **Pattern matching** - Handle Results and Options explicitly
- **if let** - Simplified pattern matching
- **panic!** - Only for unrecoverable errors
- **Custom errors** - Create meaningful error types

## Key Takeaways

1. Rust makes errors explicit through the type system
2. Option for optional values, Result for fallible operations
3. The `?` operator makes error propagation clean and idiomatic
4. Always handle Results and Options (compiler enforces this)
5. Panics are for programming errors, Results for runtime errors
6. Custom error types provide better error context
7. Error handling is zero-cost at compile time

## Practice Exercise Ideas

1. Write function returning Result with custom error
2. Chain multiple operations with ? operator
3. Create custom error enum
4. Handle multiple error types
5. Implement error conversion
6. Practice pattern matching on errors
7. Write graceful error fallback

---

**Time to complete this concept**: 2-2.5 hours
**Difficulty**: Intermediate
**Prerequisite**: Pattern Matching, Type System
**Next concept**: End of Module 02

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
