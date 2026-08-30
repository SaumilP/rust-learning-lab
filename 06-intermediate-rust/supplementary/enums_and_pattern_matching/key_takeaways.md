# Enums and Pattern Matching - Key Takeaways

## Defining Enums

```rust
enum Color {
    Red,
    Green,
    Blue,
}

enum Shape {
    Circle(f64),                      // Tuple variant
    Rectangle { width: f64, height: f64 },  // Struct variant
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

## Basic Pattern Matching

```rust
match color {
    Color::Red => println!("Red"),
    Color::Green => println!("Green"),
    Color::Blue => println!("Blue"),
}

match shape {
    Shape::Circle(r) => println!("Radius: {}", r),
    Shape::Rectangle { width, height } => println!("{}x{}", width, height),
}
```

## if let (Single Case)

```rust
let opt = Some(5);
if let Some(x) = opt {
    println!("Value: {}", x);
} else {
    println!("None");
}

// Equivalent to:
match opt {
    Some(x) => println!("Value: {}", x),
    _ => println!("None"),
}
```

## while let (Loop)

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}

// Equivalent to:
loop {
    match stack.pop() {
        Some(top) => println!("{}", top),
        None => break,
    }
}
```

## Destructuring Patterns

```rust
// Tuple destructuring
let (x, y) = (5, 6);

// Struct destructuring
let Point { x, y } = point;

// Enum with tuple data
match message {
    Message::Write(text) => println!("{}", text),
    _ => {}
}

// Enum with struct data
match message {
    Message::Move { x, y } => println!("{}, {}", x, y),
    _ => {}
}
```

## Range Patterns

```rust
match value {
    1..=5 => println!("One to five"),
    6..=10 => println!("Six to ten"),
    _ => println!("Other"),
}

// Also works with chars
match ch {
    'a'..='z' => println!("Lowercase"),
    'A'..='Z' => println!("Uppercase"),
    _ => println!("Other"),
}
```

## Guard Patterns

```rust
match value {
    x if x < 5 => println!("Less than five"),
    x if x >= 5 && x < 10 => println!("Five to ten"),
    _ => println!("Ten or more"),
}

match (x, y) {
    (x, y) if x == y => println!("Equal"),
    _ => println!("Different"),
}
```

## Binding with @

```rust
match value {
    0..=50 @ n => println!("Small: {}", n),
    51..=100 @ n => println!("Medium: {}", n),
    _ => println!("Large"),
}
```

## Wildcard Pattern

```rust
match value {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),  // Matches all remaining
}

// Ignore specific value
match (Some(x), y) {
    (Some(3), _) => println!("Got 3, ignore y"),
    _ => {}
}
```

## Multiple Patterns

```rust
match value {
    1 | 2 | 3 => println!("One, two, or three"),
    4 | 5 => println!("Four or five"),
    _ => println!("Other"),
}
```

## Important Patterns

| Pattern | Use |
|---------|-----|
| `match` | Pattern match entire expression |
| `if let` | Single pattern, ignore others |
| `while let` | Loop while pattern matches |
| `_` | Ignore value |
| `x` | Bind to variable |
| `1..=5` | Range pattern |
| `x if x > 5` | Guard condition |
| `Color::Red` | Enum variant |
| `Point { x, y }` | Struct fields |

## Result Handling

```rust
match result {
    Ok(value) => println!("Success: {}", value),
    Err(e) => println!("Error: {}", e),
}

// With if let
if let Ok(value) = result {
    println!("Got: {}", value);
}
```

## Option Handling

```rust
match opt {
    Some(x) => println!("Value: {}", x),
    None => println!("No value"),
}

// With if let
if let Some(x) = opt {
    println!("Value: {}", x);
}
```

## Exhaustiveness Requirement

✓ Must handle all enum variants
✓ Or use `_` wildcard for remaining
✓ Compiler catches non-exhaustive matches

```rust
// ERROR: missing Blue
match color {
    Color::Red => {},
    Color::Green => {},
}

// OK: handled all
match color {
    Color::Red => {},
    Color::Green => {},
    Color::Blue => {},
}

// OK: wildcard catches rest
match color {
    Color::Red => {},
    _ => {},
}
```

## Important Notes

✓ Enum variants can have associated data
✓ Destructuring extracts that data
✓ Pattern matching is exhaustive (checked by compiler)
✓ `if let` is shorthand for single pattern match
✓ `while let` loops while pattern matches
✓ Guard conditions (`if`) add logic to patterns
✓ Multiple patterns joined with `|`
✓ `@` binds matched value to variable

## Common Mistakes

1. ❌ Non-exhaustive match
   ```rust
   match color {
       Color::Red => {},
       // Missing other variants
   }
   ```

2. ❌ Forgetting to destructure
   ```rust
   match shape {
       Shape::Circle => {},  // Can't use radius
   }
   ```

3. ❌ Unreachable patterns
   ```rust
   match value {
       _ => {},           // Matches everything
       x if x > 5 => {},  // Never reached
   }
   ```

4. ❌ Wrong destructuring syntax
   ```rust
   match point {
       (x, y) => {}  // If Point is struct, use { x, y }
   }
   ```

## Quick Reference

```rust
// Define enum
enum Status {
    Pending,
    Active(String),
    Complete { result: String },
}

// Pattern match
match status {
    Status::Pending => println!("Waiting"),
    Status::Active(name) => println!("Active: {}", name),
    Status::Complete { result } => println!("Done: {}", result),
}

// if let
if let Status::Active(name) = status {
    println!("Name: {}", name);
}

// while let
while let Some(item) = queue.pop() {
    process(item);
}

// Guard
match value {
    x if x > 0 => println!("Positive"),
    x if x < 0 => println!("Negative"),
    _ => println!("Zero"),
}

// Multiple patterns
match value {
    1 | 2 | 3 => println!("Small"),
    _ => println!("Large"),
}
```

## Pattern Types

| Type | Example |
|------|---------|
| Literal | `5`, `"hello"` |
| Variable | `x`, `Some(x)` |
| Wildcard | `_` |
| Range | `1..=5` |
| Struct | `Point { x, y }` |
| Enum | `Color::Red` |
| Guard | `x if x > 5` |
| Or | `1 \| 2 \| 3` |
| Binding | `0..=50 @ n` |

## Related Concepts

- Result/Option types (use pattern matching heavily)
- State machines (built with enums)
- Derive macros (custom derives for enums)
- Advanced patterns (slice patterns, nested guards)

