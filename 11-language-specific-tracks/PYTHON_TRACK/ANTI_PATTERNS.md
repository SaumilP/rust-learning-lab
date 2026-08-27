# Anti-Patterns: Common Mistakes Python Developers Make

This guide covers mistakes I made (and every Python developer I know made) when learning Rust. Save yourself time and frustration!

## 1. Cloning Everything Because It "Just Works"

### The mistake

```rust
// ❌ ANTI-PATTERN: Clone to make the borrow checker happy
fn process_data(items: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for item in items {
        let cloned = item.clone();  // Unnecessary!
        let processed = cloned.to_uppercase();
        result.push(processed.clone());  // Also unnecessary!
    }
    result
}
```

### Why Python developers do this

In Python, you never think about copying vs referencing. When the borrow checker complains, your first instinct is to clone until it compiles.

### The correct approach

```rust
// ✅ CORRECT: Use iterators and avoid cloning
fn process_data(items: &[String]) -> Vec<String> {
    items.iter()
        .map(|item| item.to_uppercase())
        .collect()
}

// Or if you need to consume the Vec
fn process_data_consuming(items: Vec<String>) -> Vec<String> {
    items.into_iter()
        .map(|item| item.to_uppercase())
        .collect()
}
```

### When to clone

- When you actually need two independent copies
- When cloning is cheap (integers, small structs)
- When borrowing would make code significantly more complex

### When NOT to clone

- Just to satisfy the borrow checker (understand borrowing instead)
- For large data structures passed around locally
- Inside loops (performance killer)

---

## 2. Using `.unwrap()` Everywhere Like `except: pass`

### The mistake

```rust
// ❌ ANTI-PATTERN: Unwrap and hope for the best
fn load_config() -> Config {
    let file = std::fs::read_to_string("config.json")
        .unwrap();  // Crashes if file doesn't exist!

    let config: Config = serde_json::from_str(&file)
        .unwrap();  // Crashes if JSON is invalid!

    config
}
```

### Why Python developers do this

In Python, you might use bare `except:` or just let exceptions bubble up. `.unwrap()` feels similar, but it's actually a panic (program crash).

### The correct approach

```rust
// ✅ CORRECT: Proper error handling
use std::error::Error;

fn load_config() -> Result<Config, Box<dyn Error>> {
    let file = std::fs::read_to_string("config.json")?;
    let config: Config = serde_json::from_str(&file)?;
    Ok(config)
}

// Usage
match load_config() {
    Ok(config) => println!("Loaded: {:?}", config),
    Err(e) => eprintln!("Failed to load config: {}", e),
}
```

### When `.unwrap()` is OK

- In `main()` for quick prototypes
- In tests (test failures are expected)
- When you've proven something can't fail (with a comment explaining why)

```rust
// OK: We just inserted this, so it must exist
map.insert("key", value);
let value = map.get("key").unwrap();  // Can't fail
```

### Alternatives to `.unwrap()`

- `.expect("meaningful message")` - Better for debugging
- `?` operator - Propagate to caller
- `.unwrap_or(default)` - Provide fallback
- `match` - Handle each case explicitly

---

## 3. Writing Python-Style Loops Instead of Using Iterators

### The mistake

```rust
// ❌ ANTI-PATTERN: Python-style for loops
fn sum_of_squares(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..numbers.len() {
        sum += numbers[i] * numbers[i];
    }
    sum
}

fn filter_and_double(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for num in numbers {
        if *num > 0 {
            result.push(num * 2);
        }
    }
    result
}
```

### Why Python developers do this

This looks like Python's for loops. It works, but it's not idiomatic Rust.

### The correct approach

```rust
// ✅ CORRECT: Use iterator chains
fn sum_of_squares(numbers: &[i32]) -> i32 {
    numbers.iter()
        .map(|&n| n * n)
        .sum()
}

fn filter_and_double(numbers: &[i32]) -> Vec<i32> {
    numbers.iter()
        .filter(|&&n| n > 0)
        .map(|&n| n * 2)
        .collect()
}
```

### Benefits

- More concise and readable
- Composable (chain operations)
- Compiler optimizes better
- Harder to make off-by-one errors
- More functional, less imperative

---

## 4. Trying to Write Dynamic/Generic Code Like Python

### The mistake

```rust
// ❌ ANTI-PATTERN: Trying to make everything "flexible"
use std::any::Any;

fn process_anything(value: Box<dyn Any>) -> Box<dyn Any> {
    // Lost all type safety!
    if let Some(num) = value.downcast_ref::<i32>() {
        Box::new(num * 2)
    } else if let Some(s) = value.downcast_ref::<String>() {
        Box::new(s.to_uppercase())
    } else {
        value
    }
}
```

### Why Python developers do this

Python's dynamic typing lets you pass anything anywhere. In Rust, this feels restrictive.

### The correct approach

**Option 1: Use enums for known variants**

```rust
// ✅ CORRECT: Enum for known types
enum Value {
    Number(i32),
    Text(String),
}

fn process_value(value: Value) -> Value {
    match value {
        Value::Number(n) => Value::Number(n * 2),
        Value::Text(s) => Value::Text(s.to_uppercase()),
    }
}
```

**Option 2: Use generics for type-agnostic code**

```rust
// ✅ CORRECT: Generics with trait bounds
fn process<T: Clone>(value: T) -> T {
    value.clone()
}

fn print_anything<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}
```

**Option 3: Trait objects when you need runtime polymorphism**

```rust
// ✅ CORRECT: Trait objects for runtime dispatch
trait Processable {
    fn process(&self) -> String;
}

fn handle(item: &dyn Processable) {
    println!("{}", item.process());
}
```

---

## 5. Not Reading Compiler Errors

### The mistake

```
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
```

**Your reaction**: "Ugh, compiler is so annoying!" *Adds random `&`, `mut`, or `.clone()` until it compiles*

### The correct approach

**Read the error**. Rust errors explain the problem:

```
error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
  --> src/main.rs:5:5
   |
4  |     let y = &x;
   |             -- immutable borrow occurs here
5  |     x.push(1);
   |     ^^^^^^^^^ mutable borrow occurs here
6  |     println!("{:?}", y);
   |                      - immutable borrow later used here
```

The compiler tells you:
1. **What** the problem is
2. **Where** it occurred
3. **Why** it's a problem
4. Sometimes **how** to fix it

**Listen to it!** Don't fight it.

---

## 6. Avoiding Lifetimes Instead of Learning Them

### The mistake

```rust
// ❌ ANTI-PATTERN: Cloning to avoid lifetime errors
struct User {
    name: String,  // Owned, forces cloning
}

fn get_first_name(user: &User) -> String {
    user.name.split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()  // Clone to avoid lifetime!
}
```

### Why Python developers do this

Lifetimes don't exist in Python. When you see lifetime errors, you clone to make them go away.

### The correct approach

```rust
// ✅ CORRECT: Use lifetimes to borrow
fn get_first_name(user: &User) -> &str {
    user.name.split_whitespace()
        .next()
        .unwrap_or("")
}

// Or with explicit lifetime (when needed)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### When cloning is actually better

- When you need to store the value beyond the original's lifetime
- When the data is small
- When cloning simplifies the API significantly

But learn lifetimes first, then decide!

---

## 7. String Allocation Everywhere

### The mistake

```rust
// ❌ ANTI-PATTERN: Allocating strings unnecessarily
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

fn process_names(names: Vec<String>) -> Vec<String> {
    names.iter()
        .map(|name| greet(name.clone()))  // Clone + allocation
        .collect()
}
```

### Why Python developers do this

Python has one string type. You don't think about allocation.

### The correct approach

```rust
// ✅ CORRECT: Use &str where possible
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn process_names(names: &[String]) -> Vec<String> {
    names.iter()
        .map(|name| greet(name))  // No clone needed
        .collect()
}

// Even better: return &str when you can
fn greet_static(name: &str) -> String {
    // Still need String here because format! creates new string
    format!("Hello, {}!", name)
}
```

### General rule

- **Parameters**: Use `&str` (accepts both `String` and `&str`)
- **Struct fields**: Use `String` (owns the data)
- **Return values**: Use `String` if owned, `&str` if borrowed

---

## 8. Ignoring Ownership to Write "Flexible" Code

### The mistake

```rust
// ❌ ANTI-PATTERN: Trying to return references to locals
fn create_user() -> &str {
    let name = String::from("Alice");
    &name  // ERROR: returns reference to local variable!
}

// Or trying to store borrowed data incorrectly
struct Cache {
    data: &str,  // ERROR: missing lifetime specifier
}
```

### Why Python developers do this

In Python, objects live as long as there are references. In Rust, local variables are dropped when the function returns.

### The correct approach

```rust
// ✅ CORRECT: Return owned data
fn create_user() -> String {
    String::from("Alice")
}

// Or use 'static lifetime for constants
fn get_default_name() -> &'static str {
    "Guest"  // String literal has 'static lifetime
}

// For structs, add lifetime parameter
struct Cache<'a> {
    data: &'a str,
}
```

---

## 9. Premature Optimization

### The mistake

```rust
// ❌ ANTI-PATTERN: Using unsafe for "performance"
fn process(data: &[u8]) -> Vec<u8> {
    unsafe {
        // Complex pointer manipulation
        // Barely faster, much more dangerous
        std::slice::from_raw_parts(data.as_ptr(), data.len())
            .to_vec()
    }
}
```

### Why Python developers do this

Coming from Python, you're excited about Rust's performance. But safe Rust is already 50-100x faster than Python!

### The correct approach

1. **Write clear, safe code first**
2. **Measure** with benchmarks (`criterion` crate)
3. **Optimize** only bottlenecks
4. **Use `cargo clippy`** for free optimizations

```rust
// ✅ CORRECT: Clean, safe, still fast
fn process(data: &[u8]) -> Vec<u8> {
    data.to_vec()  // Simple, safe, fast enough
}

// Benchmark first, then optimize if needed
```

---

## 10. Not Using Type Inference

### The mistake

```rust
// ❌ ANTI-PATTERN: Over-annotating types
fn calculate() -> i32 {
    let x: i32 = 5;
    let y: i32 = 10;
    let sum: i32 = x + y;
    let doubled: i32 = sum * 2;
    return doubled;
}
```

### Why Python developers do this

After learning that Rust needs types, you over-annotate everything.

### The correct approach

```rust
// ✅ CORRECT: Let Rust infer types
fn calculate() -> i32 {
    let x = 5;      // Inferred as i32
    let y = 10;     // Inferred as i32
    let sum = x + y;
    sum * 2         // Last expression is returned
}
```

### When to annotate

- Function signatures (always)
- When inference would be ambiguous
- When you want to be explicit for clarity
- When converting types

```rust
// Need annotation here - ambiguous
let numbers: Vec<i32> = (0..10).collect();

// Or use turbofish
let numbers = (0..10).collect::<Vec<i32>>();
```

---

## Summary: How to Avoid These Mistakes

1. **Learn borrowing** instead of cloning everywhere
2. **Handle errors properly** instead of unwrapping
3. **Use iterators** instead of Python-style loops
4. **Embrace static typing** instead of fighting it
5. **Read compiler errors** instead of randomly trying things
6. **Learn lifetimes** instead of cloning to avoid them
7. **Use `&str` parameters** instead of allocating Strings
8. **Return owned data** instead of trying to return references
9. **Measure before optimizing** instead of premature unsafe
10. **Let Rust infer** instead of over-annotating types

## The Mindset Shift

| Python thinking | Rust thinking |
|-----------------|---------------|
| "Make it work, optimize later" | "Make it correct first" |
| "Clone to fix error" | "Understand ownership" |
| "Ignore type hints" | "Types prevent bugs" |
| "Try/except somewhere" | "Handle errors here" |
| "Dynamic is flexible" | "Static is safe" |
| "GC handles memory" | "Ownership handles memory" |

## Next Steps

1. **Build projects** - Theory only goes so far
2. **Do Rustlings** - Practice exercises
3. **Use `cargo clippy`** - Catches un-idiomatic code
4. **Ask for help** - Rust Discord #beginners
5. **Embrace the struggle** - Everyone fought the borrow checker

Remember: These anti-patterns aren't character flaws—they're natural instincts from Python. With practice, you'll develop new instincts that embrace Rust's strengths.

Next: Check out TIPS_AND_TRICKS.md for productivity shortcuts!
