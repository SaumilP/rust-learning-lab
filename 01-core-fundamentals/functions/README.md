# Concept: Functions

## Overview

Functions are reusable blocks of code that perform a specific task. Rust code uses functions extensively - you've already seen the most important one: the `main` function. Functions help organize code, make it more readable, and allow you to reuse logic without duplication.

## Learning Objectives

By the end of this concept, you will understand:
- How to declare functions with `fn`
- Function parameters and return types
- Expressions vs statements
- Early returns
- Function best practices

## Theory

### Function Basics

Functions in Rust start with the `fn` keyword. The `main` function is special - it's the entry point of every executable Rust program:

```rust
fn main() {
    println!("Hello, world!");
}
```

### Function Declaration

A function declaration specifies:
1. The `fn` keyword
2. Function name
3. Parameters (in parentheses)
4. Return type (if not void)
5. Body (in curly braces)

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Parameters

Parameters are declared with their types:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn multiply(x: f64, y: f64) -> f64 {
    x * y
}
```

**Key points**:
- Each parameter must have a type
- Multiple parameters separated by commas
- Parameters are immutable by default
- Parameters are NOT automatically mutable references

### Return Types

Functions can return values using `->`:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn double(x: i32) -> i32 {
    x * 2
}
```

Functions that don't return a value have implied return type `()` (unit type):

```rust
fn say_hello() {
    println!("Hello!");
}

fn say_hello() -> () {  // Explicit, equivalent above
    println!("Hello!");
}
```

### Expressions vs Statements

This is important in Rust!

**Statements** are instructions that perform an action but don't return a value. They end with a semicolon:

```rust
let x = 5;
let y = (let z = 6);  // ❌ ERROR: statements don't return
```

**Expressions** evaluate to a resulting value that can be returned. They do NOT end with a semicolon:

```rust
{
    let x = 3;
    x + 1  // Expression - returns 4
}

{
    let x = 3;
    x + 1;  // Statement - returns nothing!
}
```

**Critical difference**:
```rust
fn example() -> i32 {
    5 + 6    // Expression - returns 11
}

fn example() -> i32 {
    5 + 6;   // Statement - ERROR: returns () not i32
}
```

### Return Values

There are two ways to return from a function:

**Using expressions (implicit return)**:
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon - returns the value
}
```

**Using the `return` keyword (explicit return)**:
```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;  // With semicolon and return keyword
}
```

**Early return**:
```rust
fn validate(x: i32) -> bool {
    if x < 0 {
        return false;  // Early return
    }
    if x > 100 {
        return false;  // Another early return
    }
    true  // Final return
}
```

### Function Scope

Variables declared in a function are local to that function:

```rust
fn example() {
    let x = 5;
    println!("{}", x);  // ✅ OK - x in scope
}

println!("{}", x);  // ❌ ERROR - x out of scope
```

## Syntax

### Basic Function Declaration

```rust
fn function_name() {
    // body
}

fn function_name(param1: Type1) {
    // body
}

fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // body
    value_to_return
}
```

### Function Examples

```rust
// No parameters, no return
fn say_hello() {
    println!("Hello!");
}

// Parameters, no return
fn print_numbers(start: i32, end: i32) {
    for i in start..=end {
        println!("{}", i);
    }
}

// No parameters, with return
fn get_five() -> i32 {
    5
}

// Parameters and return
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Multiple parameters, complex return
fn describe_person(name: &str, age: u32) -> String {
    format!("{} is {} years old", name, age)
}
```

## Common Patterns

### Pattern 1: Simple calculation

```rust
fn calculate_area(width: f64, height: f64) -> f64 {
    width * height
}

let area = calculate_area(10.0, 5.0);
```

### Pattern 2: Validation with early return

```rust
fn validate_age(age: u32) -> bool {
    if age < 0 {
        return false;
    }
    if age > 150 {
        return false;
    }
    true
}
```

### Pattern 3: Processing input

```rust
fn process_name(name: &str) -> String {
    let trimmed = name.trim();
    let uppercase = trimmed.to_uppercase();
    uppercase
}

let result = process_name("  john  ");
```

### Pattern 4: Optional returns (covered later)

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}
```

## Common Mistakes

### Mistake 1: Forgetting parameter types

```rust
// ❌ ERROR: parameter types required
fn add(a, b) {
    a + b
}

// ✅ CORRECT: specify types
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Mistake 2: Semicolon breaks return

```rust
// ❌ ERROR: semicolon makes it a statement
fn add(a: i32, b: i32) -> i32 {
    a + b;  // Returns () not i32!
}

// ✅ CORRECT: no semicolon for return
fn add(a: i32, b: i32) -> i32 {
    a + b  // Returns the value
}
```

### Mistake 3: Modifying parameters

```rust
// ❌ ERROR: parameters are immutable
fn increment(x: i32) {
    x = x + 1;  // Can't modify x
}

// ✅ CORRECT: create local variable
fn increment(x: i32) -> i32 {
    x + 1
}
```

### Mistake 4: Missing return type

```rust
// ❌ Unclear if it returns
fn process(data: &str) {
    let result = data.len();
    // Does this return result or nothing?
}

// ✅ Clear: returns usize
fn process(data: &str) -> usize {
    data.len()
}
```

## Real-World Examples

### Example 1: Temperature conversion

```rust
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

let boiling = celsius_to_fahrenheit(100.0);
println!("{} C = {} F", 100, boiling);
```

### Example 2: Validation and processing

```rust
fn validate_email(email: &str) -> bool {
    if email.is_empty() {
        return false;
    }
    if !email.contains('@') {
        return false;
    }
    if !email.contains('.') {
        return false;
    }
    true
}

if validate_email("user@example.com") {
    println!("Valid email!");
}
```

### Example 3: String processing

```rust
fn format_name(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}

fn format_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

let full_name = format_name("John", "Doe");
let greeting = format_greeting(&full_name);
println!("{}", greeting);  // Hello, John Doe!
```

### Example 4: Accumulation

```rust
fn sum_up_to(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

println!("Sum 1-10: {}", sum_up_to(10));  // 55
```

## Related Concepts

### Prerequisites
- **Variables & Mutability** - Function parameters are variables
- **Data Types** - Parameter and return types

### What comes next
- **Control Flow** - Conditionals and loops within functions
- **Ownership** - How functions interact with data ownership

### Cross-references
- Module 01: All concepts use functions
- Module 02: Function types and closures
- Module 06: Generic functions with trait bounds

## Best Practices

### Use descriptive function names

```rust
// ✅ Good: clear what it does
fn calculate_total_price(items: &[f64], tax_rate: f64) -> f64 { }

// ❌ Avoid: unclear
fn calc(x: &[f64], t: f64) -> f64 { }
```

### Keep functions focused (Single Responsibility)

```rust
// ✅ Good: each function does one thing
fn validate_input(input: &str) -> bool { }
fn process_input(input: &str) -> String { }

// ❌ Avoid: doing too much
fn handle_input(input: &str) -> (bool, String) { }
```

### Use explicit return types for clarity

```rust
// ✅ Good: clear what's returned
fn get_count(items: &[i32]) -> usize {
    items.len()
}

// ⚠️ Less clear without return type
fn get_count(items: &[i32]) {
    items.len()  // Does this return or not?
}
```

### Prefer expressions over early returns when possible

```rust
// ✅ Good: expression-oriented
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// ⚠️ Okay but more verbose
fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    return b;
}
```

## Summary

- **Functions** organize code into reusable units
- **Parameters** have explicit types
- **Return types** specified with `->`
- **Expressions** return values, statements don't
- **Scope** - local variables only exist in function
- **Early returns** for validation logic

## Key Takeaways

1. Functions require parameter types
2. Return values use expressions (no semicolon)
3. `return` keyword is optional for last expression
4. Function names should describe what they do
5. Keep functions small and focused

## Practice Exercise Ideas

1. Write a function that validates age
2. Create a function to format data
3. Implement a simple calculation function
4. Practice using early returns
5. Combine multiple functions

---

**Time to complete this concept**: 1-1.5 hours
**Difficulty**: Beginner
**Prerequisite**: Variables & Mutability, Data Types
**Next concept**: Control Flow

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
