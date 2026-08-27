# Enums and Pattern Matching

## Overview

Enums (enumerations) are types that can have multiple distinct variants, each potentially carrying different data. Pattern matching is a powerful control flow construct that destructures enums and extracts their contained values. Together, they provide an elegant way to handle complex data structures and branching logic. This combination is central to idiomatic Rust and enables expressive, safe code that handles all cases.

## Theory

### What are Enums?

Enums represent a value that can be one of several variants. Unlike most languages where enums are just named constants, Rust enums can carry associated data.

```rust
enum Message {
    Quit,                      // No data
    Move { x: i32, y: i32 },  // Struct variant
    Write(String),             // Tuple variant
    ChangeColor(i32, i32, i32) // Multiple data
}
```

### Pattern Matching Basics

Pattern matching allows destructuring values and handling each case:

```rust
match value {
    Pattern1 => { /* handle pattern1 */ },
    Pattern2 => { /* handle pattern2 */ },
    _ => { /* handle default */ }
}
```

### Pattern Matching Rules

1. **Exhaustiveness** - Must handle all cases (or use `_`)
2. **Type Safety** - Compiler ensures patterns are valid
3. **Binding** - Extract values from patterns
4. **Destructuring** - Unwrap nested structures

### Common Patterns

1. **Literal Patterns** - Match exact values
2. **Variable Patterns** - Capture values
3. **Underscore Pattern** - Ignore values
4. **Range Patterns** - Match ranges
5. **Struct/Tuple Patterns** - Destructure
6. **Enum Patterns** - Match enum variants
7. **Guard Patterns** - Add conditions

## Syntax

### Defining Enums

```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

enum Shape {
    Circle(f64),              // Tuple variant
    Rectangle { width: f64, height: f64 },  // Struct variant
    Triangle(f64, f64, f64),
}
```

### Pattern Matching on Enums

```rust
fn describe_color(color: Color) -> String {
    match color {
        Color::Red => "Red".to_string(),
        Color::Green => "Green".to_string(),
        Color::Blue => "Blue".to_string(),
    }
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle(a, b, c) => {
            // Heron's formula
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}
```

### if let Expression

```rust
let opt = Some(5);
if let Some(x) = opt {
    println!("Value: {}", x);
} else {
    println!("None");
}
```

### while let Loop

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### Destructuring in Patterns

```rust
let (x, y) = (5, 6);  // Tuple destructuring
let Point { x, y } = point;  // Struct destructuring

match result {
    Ok((x, y)) => println!("Got: {}, {}", x, y),
    Err(e) => println!("Error: {}", e),
}
```

### Range Patterns

```rust
match value {
    1..=5 => println!("One to five"),
    6..=10 => println!("Six to ten"),
    _ => println!("Other"),
}
```

### Guard Patterns

```rust
match value {
    x if x < 5 => println!("Less than five"),
    x if x >= 5 && x < 10 => println!("Five to ten"),
    _ => println!("Ten or more"),
}
```

### Binding with `@`

```rust
match value {
    0..=50 @ x => println!("Small: {}", x),
    51..=100 @ x => println!("Medium: {}", x),
    _ => println!("Large"),
}
```

## Common Patterns

### Pattern 1: Result Handling

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(10.0, 2.0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}
```

### Pattern 2: Option Matching

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn main() {
    match find_first_even(&[1, 3, 5, 6, 7]) {
        Some(n) => println!("Found: {}", n),
        None => println!("No even number"),
    }
}
```

### Pattern 3: Enum with Associated Data

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    Message::Quit.call();
    Message::Move { x: 5, y: 10 }.call();
}
```

### Pattern 4: State Machine with Enums

```rust
#[derive(Debug)]
enum State {
    Loading,
    Ready(String),
    Error(String),
}

struct App {
    state: State,
}

impl App {
    fn process(&mut self, input: &str) {
        self.state = match &self.state {
            State::Loading => {
                if !input.is_empty() {
                    State::Ready(input.to_string())
                } else {
                    State::Loading
                }
            }
            State::Ready(_) => State::Ready(input.to_string()),
            State::Error(_) => {
                if input == "retry" {
                    State::Loading
                } else {
                    State::Error("Invalid command".to_string())
                }
            }
        };
    }
}
```

### Pattern 5: Conditional Matching with Guards

```rust
#[derive(Debug)]
enum HttpStatus {
    Success(u32),
    Redirect(u32),
    ClientError(u32),
    ServerError(u32),
}

fn describe_status(status: HttpStatus) -> String {
    match status {
        HttpStatus::Success(code) if code >= 200 && code < 300 => {
            format!("Success: {}", code)
        }
        HttpStatus::Redirect(code) if code >= 300 && code < 400 => {
            format!("Redirect: {}", code)
        }
        HttpStatus::ClientError(code) if code >= 400 && code < 500 => {
            format!("Client Error: {}", code)
        }
        HttpStatus::ServerError(code) if code >= 500 && code < 600 => {
            format!("Server Error: {}", code)
        }
        _ => "Unknown".to_string(),
    }
}
```

## Common Mistakes

### Mistake 1: Non-Exhaustive Pattern Match

```rust
// ❌ WRONG - Doesn't handle all enum variants
match color {
    Color::Red => println!("Red"),
    Color::Green => println!("Green"),
    // Missing Color::Blue
}

// ✅ CORRECT - Handle all or use wildcard
match color {
    Color::Red => println!("Red"),
    Color::Green => println!("Green"),
    Color::Blue => println!("Blue"),
}

// ✅ ALSO CORRECT - Wildcard for remaining
match color {
    Color::Red => println!("Red"),
    Color::Green => println!("Green"),
    _ => println!("Other"),
}
```

### Mistake 2: Forgetting to Destructure

```rust
// ❌ WRONG - Not extracting data from variant
match shape {
    Shape::Circle => { /* can't use radius */ }
}

// ✅ CORRECT - Destructure to get data
match shape {
    Shape::Circle(r) => println!("Radius: {}", r),
}
```

### Mistake 3: Incorrect Pattern Syntax

```rust
// ❌ WRONG - Wrong destructuring syntax
match point {
    Point(x, y) => println!("{}, {}", x, y),  // If Point is struct
}

// ✅ CORRECT - Use proper struct destructuring
match point {
    Point { x, y } => println!("{}, {}", x, y),
}
```

### Mistake 4: Missing Binding in Guard

```rust
// ❌ WRONG - Variable not bound before guard
match value {
    x if x > 5 => {  // x is bound here
        println!("{}", x);
    }
    _ => {}
}

// More common mistake:
match value {
    Pattern1 => {},
    Pattern2 if value > 5 => {},  // value not destructured
}
```

### Mistake 5: Unreachable Patterns

```rust
// ❌ WRONG - First pattern catches everything
match value {
    _ => println!("All"),
    x if x > 5 => println!("Greater"),  // Unreachable
}

// ✅ CORRECT - More specific patterns first
match value {
    x if x > 5 => println!("Greater"),
    _ => println!("All"),
}
```

## Real-World Examples

### Example 1: Command Parser

```rust
#[derive(Debug)]
enum Command {
    Quit,
    Help(Option<String>),
    Execute(String, Vec<String>),
    Config { key: String, value: String },
}

fn parse_command(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    match parts.get(0) {
        Some(&"quit") => Some(Command::Quit),
        Some(&"help") => Some(Command::Help(parts.get(1).map(|s| s.to_string()))),
        Some(&cmd) => {
            let args = parts[1..].iter().map(|s| s.to_string()).collect();
            Some(Command::Execute(cmd.to_string(), args))
        }
        None => None,
    }
}

fn execute(cmd: Command) {
    match cmd {
        Command::Quit => println!("Quitting..."),
        Command::Help(topic) => {
            match topic {
                Some(t) => println!("Help for: {}", t),
                None => println!("General help"),
            }
        }
        Command::Execute(prog, args) => {
            println!("Executing {} with args: {:?}", prog, args);
        }
        Command::Config { key, value } => {
            println!("Set {} = {}", key, value);
        }
    }
}
```

### Example 2: Event Handling

```rust
#[derive(Debug)]
enum Event {
    Click { x: i32, y: i32 },
    KeyPress(char),
    WindowClose,
    Resize { width: u32, height: u32 },
}

fn handle_event(event: Event) {
    match event {
        Event::Click { x, y } => {
            println!("Clicked at ({}, {})", x, y);
        }
        Event::KeyPress(ch) => {
            println!("Key pressed: {}", ch);
        }
        Event::WindowClose => {
            println!("Window closing");
        }
        Event::Resize { width, height } => {
            println!("Resized to {}x{}", width, height);
        }
    }
}
```

### Example 3: JSON-Like Data

```rust
#[derive(Debug)]
enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(std::collections::HashMap<String, JsonValue>),
}

fn json_to_string(value: &JsonValue) -> String {
    match value {
        JsonValue::Null => "null".to_string(),
        JsonValue::Boolean(b) => b.to_string(),
        JsonValue::Number(n) => n.to_string(),
        JsonValue::String(s) => format!("\"{}\"", s),
        JsonValue::Array(arr) => {
            let items = arr.iter().map(json_to_string).collect::<Vec<_>>();
            format!("[{}]", items.join(", "))
        }
        JsonValue::Object(obj) => {
            let items = obj
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, json_to_string(v)))
                .collect::<Vec<_>>();
            format!("{{{}}}", items.join(", "))
        }
    }
}
```

### Example 4: Result Chain

```rust
#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    OutOfRange,
    Empty,
}

fn parse_positive_integer(s: &str) -> Result<u32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }

    match s.parse::<i32>() {
        Ok(n) if n >= 0 => Ok(n as u32),
        Ok(_) => Err(ParseError::OutOfRange),
        Err(_) => Err(ParseError::InvalidFormat),
    }
}

fn main() {
    let inputs = vec!["42", "-5", "abc", ""];

    for input in inputs {
        match parse_positive_integer(input) {
            Ok(n) => println!("Parsed: {}", n),
            Err(ParseError::InvalidFormat) => println!("Invalid format"),
            Err(ParseError::OutOfRange) => println!("Out of range"),
            Err(ParseError::Empty) => println!("Empty string"),
        }
    }
}
```

### Example 5: Tree Traversal

```rust
#[derive(Debug)]
enum Tree<T> {
    Empty,
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
}

impl<T: std::fmt::Display> Tree<T> {
    fn print_inorder(&self) {
        match self {
            Tree::Empty => {}
            Tree::Node { value, left, right } => {
                left.print_inorder();
                print!("{} ", value);
                right.print_inorder();
            }
        }
    }
}
```

## Related Concepts

### Prerequisites
- Module 01: Data types, Pattern matching basics
- Module 02: Collections
- Module 05: Ownership and error handling

### Follow-ups
- Advanced Pattern Matching (Slice patterns, OR patterns)
- Derive Macros (Custom derives for enums)
- State Machines (Complex enum-based states)

## Best Practices

1. **Exhaustive Matching** - Handle all cases or use `_`
2. **Use `if let` for Single Cases** - More readable than full match
3. **Use `while let` for Loops** - Cleaner iteration over Options/Results
4. **Guard Patterns** - Add conditions when needed
5. **Meaningful Variant Names** - Make intent clear
6. **Document Complex Enums** - Explain variants and their meanings
7. **Avoid Over-Nesting** - Flatten patterns where possible
8. **Use Enums for States** - Better than boolean flags

## Summary

Enums and pattern matching form the backbone of expressive Rust code. Enums with associated data replace complex nested structures in other languages, while pattern matching provides elegant control flow. Together they enable writing code that is simultaneously safe, readable, and efficient.

## Practice Exercise Ideas

1. Create a calculator enum with operations and pattern match on them
2. Build a state machine for a traffic light system
3. Implement a binary tree enum with traversal functions
4. Create a command-line parser using enums
5. Build a simple protocol handler with message enums

