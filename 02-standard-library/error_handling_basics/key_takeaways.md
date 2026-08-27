# Key Takeaways: Error Handling Basics

## Quick Reference

### Option<T>

```rust
let x: Option<i32> = Some(5);
let y: Option<i32> = None;

// Methods
x.is_some();               // bool
x.is_none();               // bool
x.unwrap();                // T (panics if None)
x.unwrap_or(default);      // T
x.map(|v| v * 2);          // Option<i32>
x.and_then(|v| Some(v*2)); // Option<i32>

// Matching
match x { Some(v) => { }, None => { } }

// if let
if let Some(v) = x { }
```

### Result<T, E>

```rust
let r: Result<i32, String> = Ok(5);
let e: Result<i32, String> = Err("error".to_string());

// Methods
r.is_ok();                 // bool
r.is_err();                // bool
r.ok();                    // Option<T>
r.err();                   // Option<E>
r.unwrap();                // T (panics if Err)
r.unwrap_or(default);      // T
r.map(|v| v * 2);          // Result<i32, E>
r.map_err(|e| format!("{}",e));  // Result<T, String>
r?                         // Propagate error

// Matching
match r { Ok(v) => { }, Err(e) => { } }

// if let
if let Ok(v) = r { }
```

## Essential Concepts

### 1. Option<T> vs Result<T, E>

| Type | Use Case | Values |
|------|----------|--------|
| `Option<T>` | Value may not exist | Some(T) or None |
| `Result<T, E>` | Operation may fail | Ok(T) or Err(E) |

```rust
// Option: present or absent
let first = vec![1, 2, 3].get(0);  // Some(&1)

// Result: success or failure
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("div by zero".into()) }
    else { Ok(a / b) }
}
```

### 2. The ? Operator

Automatically propagates errors up the call stack:

```rust
// Without ?
fn read_file(path: &str) -> Result<String, std::io::Error> {
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

// With ? (cleaner)
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)?
}

// Key: ? can only be used in functions returning Result or Option
```

### 3. When to Panic vs Result

| Situation | Use |
|-----------|-----|
| Programming error (precondition violated) | panic! |
| User input invalid | Result |
| File not found | Result |
| Logic bug (shouldn't happen) | panic! |
| Expected failure | Result |

```rust
// panic! - developer's fault
assert!(x > 0, "x must be positive");

// Result - user's input
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}
```

### 4. Pattern Matching

```rust
// Match on Option
match opt {
    Some(value) => println!("Value: {}", value),
    None => println!("No value"),
}

// Match on Result
match result {
    Ok(value) => println!("Success: {}", value),
    Err(e) => println!("Error: {}", e),
}

// if let - simpler pattern
if let Some(v) = opt { println!("{}", v); }
if let Ok(v) = result { println!("{}", v); }

// if let...else
if let Ok(v) = result {
    println!("Success: {}", v);
} else {
    println!("Failed");
}
```

### 5. Extracting Values Safely

```rust
// unwrap() - panics if None/Err
let v = opt.unwrap();

// unwrap_or() - provides default
let v = opt.unwrap_or(default_value);

// unwrap_or_else() - computes default
let v = opt.unwrap_or_else(|| compute_default());

// map() - transform value
let doubled = opt.map(|x| x * 2);

// and_then() - chain operations
let result = opt.and_then(|x| Some(x * 2));
```

### 6. Error Propagation

```rust
// ? returns error early
fn process() -> Result<String, std::io::Error> {
    let file = std::fs::read_to_string("input.txt")?;
    // ^ Returns Err immediately if file not found
    let data = parse(&file)?;
    // ^ Returns Err immediately if parse fails
    Ok(data)
}

// Equivalent to:
fn process() -> Result<String, std::io::Error> {
    match std::fs::read_to_string("input.txt") {
        Ok(file) => match parse(&file) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
```

## Common Patterns

### Pattern 1: Simple unwrap
```rust
let v = risky_op().unwrap_or(default);
```

### Pattern 2: Match on Result
```rust
match risky_op() {
    Ok(v) => println!("Success: {}", v),
    Err(e) => println!("Error: {}", e),
}
```

### Pattern 3: Error propagation
```rust
fn complex() -> Result<i32, Error> {
    let x = risky_op()?;
    let y = another_risky()?;
    Ok(x + y)
}
```

### Pattern 4: if let binding
```rust
if let Some(v) = maybe_value {
    println!("Got: {}", v);
}
```

### Pattern 5: Transform error
```rust
risky_op()
    .map_err(|e| format!("Operation failed: {}", e))
```

### Pattern 6: Default on error
```rust
config = load_config()
    .unwrap_or_else(|_| Config::default())
```

### Pattern 7: Chain operations
```rust
let result = vec![1,2,3]
    .iter()
    .find(|&&x| x > 5)
    .ok_or("Not found")?;
```

### Pattern 8: Convert types
```rust
let num: i32 = "42".parse()
    .map_err(|_| "Invalid number")?;
```

## Checklist: Error Handling

**For Option:**
- [ ] Is value definitely present?
- [ ] Can I use unwrap safely?
- [ ] Should I provide a default with unwrap_or?
- [ ] Can I use if let for cleaner code?

**For Result:**
- [ ] Should this operation fail?
- [ ] Can caller recover from error?
- [ ] Should I propagate with ??
- [ ] Do I need custom error type?

**For panic!:**
- [ ] Is this a programming error?
- [ ] Is this unrecoverable?
- [ ] Should user handle this differently?

## Error Prevention

### ❌ DON'T: Ignore Result
```rust
std::fs::read_to_string("file.txt");  // Error not handled!
```

### ✅ DO: Handle Result
```rust
let content = std::fs::read_to_string("file.txt")?;
// or
match std::fs::read_to_string("file.txt") {
    Ok(c) => { /* use c */ },
    Err(e) => { /* handle error */ },
}
```

### ❌ DON'T: Unwrap without checking
```rust
let v = vec![1, 2, 3];
let item = v.get(10).unwrap();  // Panics!
```

### ✅ DO: Handle safely
```rust
let item = v.get(10).unwrap_or(&0);  // Safe
// or
if let Some(item) = v.get(10) { /* use item */ }
```

### ❌ DON'T: Use ? outside Result/Option function
```rust
fn process() {  // Returns nothing
    let file = std::fs::read_to_string("file.txt")?;  // ERROR
}
```

### ✅ DO: Return Result
```rust
fn process() -> Result<String, std::io::Error> {
    std::fs::read_to_string("file.txt")?
}
```

### ❌ DON'T: Panic on user input
```rust
fn parse_age(input: &str) -> i32 {
    input.parse().unwrap()  // Bad: input can fail
}
```

### ✅ DO: Return Result
```rust
fn parse_age(input: &str) -> Result<i32, ParseIntError> {
    input.parse()
}
```

### ❌ DON'T: Lose error information
```rust
match operation() {
    Ok(v) => println!("{}", v),
    Err(_) => println!("Failed"),  // Lost error details
}
```

### ✅ DO: Preserve errors
```rust
match operation() {
    Ok(v) => println!("{}", v),
    Err(e) => println!("Failed: {}", e),  // Keep details
}
```

## Quick Method Reference

### Option Methods

| Method | Purpose | Returns |
|--------|---------|---------|
| `is_some()` | Check if Some? | bool |
| `is_none()` | Check if None? | bool |
| `unwrap()` | Extract or panic | T |
| `unwrap_or(d)` | Extract or use default | T |
| `unwrap_or_else(f)` | Extract or compute default | T |
| `map(f)` | Transform value | Option |
| `and_then(f)` | Chain operations | Option |
| `ok_or(e)` | Convert to Result | Result |

### Result Methods

| Method | Purpose | Returns |
|--------|---------|---------|
| `is_ok()` | Check if Ok? | bool |
| `is_err()` | Check if Err? | bool |
| `ok()` | Convert to Option | Option<T> |
| `err()` | Extract error | Option<E> |
| `unwrap()` | Extract or panic | T |
| `unwrap_or(d)` | Extract or use default | T |
| `map(f)` | Transform value | Result |
| `map_err(f)` | Transform error | Result |
| `and_then(f)` | Chain operations | Result |

## Performance Tips

### Option/Result are zero-cost
- No overhead compared to manual checks
- Optimizations eliminate all indirection
- Pattern matching is as efficient as if statements

### Use ? for efficiency
- Avoids nesting of match statements
- Single exit path
- LLVM optimizes well

### Avoid unnecessary allocations
```rust
// ✅ Good: borrows error
match op() {
    Err(e) => eprintln!("{}", e),
}

// ❌ Unnecessary: clones error
match op() {
    Err(e) => {
        let e_copy = e.clone();
        eprintln!("{}", e_copy);
    }
}
```

## Related Concepts

- **Pattern Matching** - Fundamental to error handling
- **Traits** - Error trait for custom types
- **Generics** - Option and Result are generic
- **Type System** - Errors captured in types

## Time Estimates

- Reading this takeaway: 10-15 minutes
- Reviewing patterns: 10 minutes
- Practice drills: 20-30 minutes
- Total: 40-55 minutes

## Practice Questions

1. What's the difference between Option and Result?
2. When would you use unwrap vs unwrap_or?
3. How does the ? operator work?
4. When should you panic vs return Result?
5. How do you chain multiple Result operations?
6. What does map do to an Option/Result?
7. How do you handle multiple error types?
8. When is if let appropriate vs match?

## Common Conversions

| From | To | Method |
|------|----|----|
| Result | Option | `.ok()` or `.err()` |
| Option | Result | `.ok_or(err)` |
| Value | Option | `Some(value)` |
| Value | Result | `Ok(value)` |
| Error | Result | `Err(error)` |

## Real-World Scenarios

| Problem | Solution | Pattern |
|---------|----------|---------|
| Value might not exist | Use Option | `if let Some(v)` |
| Operation might fail | Use Result | Return Result |
| Parse user input | Return Result | `.parse()?` |
| File might not exist | Return Result | `.read_to_string()?` |
| Provide default value | Use unwrap_or | `.unwrap_or(default)` |
| Multiple fallible ops | Chain with ? | `op1()? op2()?` |
| Ignore error silently | Use map | `.map(f)` |
| Log error, continue | Use unwrap_or_else | `.unwrap_or_else(\|e\| { log })` |

## Cheat Sheet

**Quick Result pattern:**
```rust
fn risky() -> Result<T, E> { Ok(value)? Err(e) }

fn caller() -> Result<T, E> {
    let result = risky()?;  // Propagate
    Ok(process(result)?)    // Chain
}
```

**Quick Option pattern:**
```rust
if let Some(value) = maybe {
    println!("{}", value);
} else {
    println!("None");
}
```

**Match both types:**
```rust
match operation() {
    Ok(success) => println!("Got: {}", success),
    Err(err) => println!("Error: {}", err),
}
```

---

**Status**: Quick reference guide
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Intermediate
**Part of**: Module 02 - Standard Library
