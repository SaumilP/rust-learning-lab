# Key Takeaways: Functions

## Quick Reference

### Function Declaration
```rust
fn greet() {
    println!("Hello!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn process(name: &str) -> String {
    format!("Hello, {}", name)
}
```

### Calling Functions
```rust
greet();
let sum = add(5, 3);
let result = process("Alice");
```

## Essential Concepts

### 1. Function Declaration
```
fn name(param: Type) -> ReturnType {
    body
}
```

### 2. Parameters MUST have types
```rust
// ❌ ERROR
fn add(a, b) { }

// ✅ CORRECT
fn add(a: i32, b: i32) -> i32 { }
```

### 3. Expressions vs Statements

**Statements** (end with `;`): Don't return value
```rust
let x = 5;
let y = (let z = 6);  // ❌ Error
```

**Expressions** (no `;`): Return value
```rust
5 + 6              // Returns 11
{
    let x = 3;
    x + 1          // Returns 4
}
```

### 4. Return Values
```rust
// Implicit return (expression)
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon!
}

// Explicit return
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
```

### 5. Critical: Semicolon breaks return!
```rust
fn broken() -> i32 {
    5;  // ❌ Returns nothing (statement)
}

fn works() -> i32 {
    5   // ✅ Returns 5 (expression)
}
```

## Common Patterns

### Pattern 1: Simple function
```rust
fn double(x: i32) -> i32 {
    x * 2
}
```

### Pattern 2: No return value
```rust
fn print_twice(x: i32) {
    println!("{}", x);
    println!("{}", x);
}
```

### Pattern 3: Multiple parameters
```rust
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}
```

### Pattern 4: String processing
```rust
fn uppercase(s: &str) -> String {
    s.to_uppercase()
}
```

### Pattern 5: Validation with early return
```rust
fn is_valid(age: u32) -> bool {
    if age < 18 { return false; }
    if age > 120 { return false; }
    true
}
```

## Checklist

When writing a function:

- [ ] Does it have a descriptive name?
- [ ] Do all parameters have types?
- [ ] Is the return type specified (if returning)?
- [ ] No semicolon on last expression?
- [ ] Does it do one thing?

## Function Structure

```
fn function_name(param: Type) -> ReturnType {
    // Do something
    return_value  // No semicolon for expression
}
```

## Return Types

| Return | Means |
|--------|-------|
| `-> i32` | Returns i32 |
| `-> String` | Returns String |
| `-> ()` | Returns nothing (unit type) |
| (omitted) | Same as `-> ()` |

## Common Errors

### ❌ DON'T: Forget parameter types
```rust
fn add(a, b) -> i32 {
    a + b
}
```

### ✅ DO: Include parameter types
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### ❌ DON'T: Semicolon on return
```rust
fn get_five() -> i32 {
    5;  // Oops! Returns () not i32
}
```

### ✅ DO: No semicolon on expression
```rust
fn get_five() -> i32 {
    5   // Correct!
}
```

### ❌ DON'T: Modify parameters
```rust
fn increment(x: i32) {
    x = x + 1;  // Can't modify
}
```

### ✅ DO: Return modified value
```rust
fn increment(x: i32) -> i32 {
    x + 1
}
```

## Practice Questions

1. What type must all parameters have?
2. Why does a semicolon break the return?
3. When would you use early return?
4. What's the difference between expression and statement?
5. How do you return nothing from a function?

## Related Concepts

- **Before**: Variables & Mutability, Data Types
- **After**: Control Flow, Ownership

## Time Estimates

- Reading this concept: 20-30 minutes
- Working through examples: 30-40 minutes
- Practice exercises: 30-60 minutes
- Total: 1-1.5 hours

## Resources

- [Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust by Example: Functions](https://doc.rust-lang.org/rust-by-example/fn.html)

---

**Status**: Core concept
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Beginner
