# Anti-Patterns: Common Mistakes for Go Developers

Coming from Go, you'll naturally try to apply Go patterns to Rust. Some work fine, but others fight against Rust's design. This guide shows common mistakes Go developers make when learning Rust—and how to fix them.

## Table of Contents

1. [Fighting the Borrow Checker](#fighting-the-borrow-checker)
2. [Overusing Arc<Mutex<T>>](#overusing-arcmutext)
3. [Cloning Everything](#cloning-everything)
4. [Ignoring Iterators](#ignoring-iterators)
5. [Using unwrap() in Production](#using-unwrap-in-production)
6. [Not Using Enums](#not-using-enums)
7. [Treating Strings Like Go](#treating-strings-like-go)
8. [Reimplementing Go's defer](#reimplementing-gos-defer)
9. [Avoiding Lifetimes with 'static](#avoiding-lifetimes-with-static)
10. [Not Leveraging Zero-Cost Abstractions](#not-leveraging-zero-cost-abstractions)

---

## Fighting the Borrow Checker

### The Anti-Pattern

```rust
// ❌ Trying to write Go-style code
fn process_data(data: &mut Vec<i32>) {
    let first = &data[0];  // Immutable borrow
    data.push(10);         // ERROR! Can't mutate while borrowed
    println!("{}", first);
}
```

**Why Go developers do this**: In Go, you can read and write to slices freely. The borrow checker feels restrictive.

### The Fix

```rust
// ✅ Work with the borrow checker
fn process_data(data: &mut Vec<i32>) {
    let first = data[0];   // Copy the value
    data.push(10);         // Now we can mutate
    println!("{}", first);
}

// Or restructure to avoid the conflict
fn process_data_better(data: &mut Vec<i32>) {
    data.push(10);
    if let Some(first) = data.first() {
        println!("{}", first);
    }
}
```

**The lesson**: Don't fight the borrow checker—redesign your approach. The compiler is preventing real bugs.

---

## Overusing Arc<Mutex<T>>

### The Anti-Pattern

```rust
// ❌ Treating Rust like Go with shared pointers everywhere
use std::sync::{Arc, Mutex};

struct AppState {
    counter: Arc<Mutex<i32>>,
    data: Arc<Mutex<Vec<String>>>,
    config: Arc<Mutex<Config>>,
}

fn increment(state: &AppState) {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
}
```

**Why Go developers do this**: In Go, you pass pointers around and use mutexes for synchronization. This feels natural.

### The Fix

```rust
// ✅ Use ownership and borrowing first
struct AppState {
    counter: AtomicI32,      // Atomic for simple counters
    data: RwLock<Vec<String>>,  // RwLock for read-heavy data
    config: Config,          // Immutable config doesn't need locking
}

fn increment(state: &AppState) {
    state.counter.fetch_add(1, Ordering::SeqCst);
}

// Or use message passing instead of shared state
use std::sync::mpsc;

fn message_passing_approach() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(Message::Increment).unwrap();
    });

    // Single thread owns the data
    let mut counter = 0;
    for msg in rx {
        match msg {
            Message::Increment => counter += 1,
        }
    }
}
```

**The lesson**: Arc<Mutex<T>> should be a last resort. Try atomics, RwLock, or message passing first.

---

## Cloning Everything

### The Anti-Pattern

```rust
// ❌ Cloning to avoid borrow checker errors
fn process_items(items: Vec<String>) {
    for item in &items {
        let item_copy = item.clone();  // Unnecessary clone
        expensive_operation(item_copy);
    }

    let items_copy = items.clone();  // Another unnecessary clone
    save_to_db(items_copy);
}

fn expensive_operation(s: String) { /* ... */ }
fn save_to_db(items: Vec<String>) { /* ... */ }
```

**Why Go developers do this**: Cloning makes borrow checker errors go away. It feels like copying pointers in Go.

### The Fix

```rust
// ✅ Use references appropriately
fn process_items(items: Vec<String>) {
    for item in &items {
        expensive_operation(item);  // Pass reference
    }

    save_to_db(items);  // Move ownership here (last use)
}

fn expensive_operation(s: &str) { /* ... */ }
fn save_to_db(items: Vec<String>) { /* ... */ }

// Or if you need multiple ownership, design for it
fn process_items_shared(items: &[String]) {
    for item in items {
        expensive_operation(item);
    }
    // items still valid, caller owns it
}
```

**The lesson**: Cloning is expensive. Use references for reading, move for transferring ownership.

---

## Ignoring Iterators

### The Anti-Pattern

```rust
// ❌ Writing Go-style loops
fn sum_evens(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            sum += numbers[i];
        }
    }
    sum
}

fn to_uppercase(words: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for word in words {
        result.push(word.to_uppercase());
    }
    result
}
```

**Why Go developers do this**: Go doesn't have powerful iterators, so explicit loops are the norm.

### The Fix

```rust
// ✅ Use iterator chains (zero-cost!)
fn sum_evens(numbers: &[i32]) -> i32 {
    numbers.iter()
        .filter(|n| *n % 2 == 0)
        .sum()
}

fn to_uppercase(words: &[String]) -> Vec<String> {
    words.iter()
        .map(|w| w.to_uppercase())
        .collect()
}

// More complex example
fn process_data(items: &[Item]) -> Vec<ProcessedItem> {
    items.iter()
        .filter(|item| item.is_valid())
        .map(|item| item.process())
        .filter_map(|result| result.ok())
        .collect()
}
```

**The lesson**: Iterators are zero-cost abstractions. They're often faster than manual loops and more readable.

---

## Using unwrap() in Production

### The Anti-Pattern

```rust
// ❌ Using unwrap() like you'd ignore errors in Go with _
fn load_config(path: &str) -> Config {
    let content = fs::read_to_string(path).unwrap();  // ❌ Panic!
    serde_json::from_str(&content).unwrap()           // ❌ Panic!
}

fn main() {
    let config = load_config("config.json");
    // If file doesn't exist or JSON is invalid, entire program panics
}
```

**Why Go developers do this**: It's quick, like using `_` to ignore errors in Go during prototyping.

### The Fix

```rust
// ✅ Proper error handling
use anyhow::{Context, Result};

fn load_config(path: &str) -> Result<Config> {
    let content = fs::read_to_string(path)
        .context("Failed to read config file")?;

    let config = serde_json::from_str(&content)
        .context("Failed to parse config JSON")?;

    Ok(config)
}

fn main() -> Result<()> {
    let config = load_config("config.json")?;
    // Error propagates with context
    Ok(())
}

// Or use match for specific error handling
fn load_config_with_default(path: &str) -> Config {
    match fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}
```

**The lesson**: `unwrap()` is for prototyping. Production code should use `?`, `match`, or `unwrap_or`.

---

## Not Using Enums

### The Anti-Pattern

```rust
// ❌ Using multiple Option fields like Go's nil pointers
struct PaymentResult {
    success: bool,
    transaction_id: Option<String>,
    error: Option<String>,
}

fn process_payment(amount: f64) -> PaymentResult {
    if amount > 0.0 {
        PaymentResult {
            success: true,
            transaction_id: Some("TX123".to_string()),
            error: None,
        }
    } else {
        PaymentResult {
            success: false,
            transaction_id: None,
            error: Some("Invalid amount".to_string()),
        }
    }
}

// Usage requires checking multiple fields
fn main() {
    let result = process_payment(100.0);
    if result.success {
        println!("ID: {}", result.transaction_id.unwrap());
    } else {
        println!("Error: {}", result.error.unwrap());
    }
}
```

**Why Go developers do this**: Go uses booleans and nil pointers for state. This pattern feels familiar.

### The Fix

```rust
// ✅ Use enums for mutually exclusive states
enum PaymentResult {
    Success { transaction_id: String },
    Failure { error: String },
}

fn process_payment(amount: f64) -> PaymentResult {
    if amount > 0.0 {
        PaymentResult::Success {
            transaction_id: "TX123".to_string(),
        }
    } else {
        PaymentResult::Failure {
            error: "Invalid amount".to_string(),
        }
    }
}

// Usage with exhaustive matching
fn main() {
    match process_payment(100.0) {
        PaymentResult::Success { transaction_id } => {
            println!("ID: {}", transaction_id);
        }
        PaymentResult::Failure { error } => {
            println!("Error: {}", error);
        }
    }
    // Compiler ensures all cases are handled
}
```

**The lesson**: Rust's enums are powerful. Use them for states, not booleans and Options.

---

## Treating Strings Like Go

### The Anti-Pattern

```rust
// ❌ Treating String and &str the same
fn concat_strings(a: String, b: String) -> String {
    format!("{}{}", a, b)  // Takes ownership, can't reuse a or b
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let result = concat_strings(s1, s2);
    // s1 and s2 are now invalid!
}

// ❌ Converting unnecessarily
fn print_message(msg: String) {
    println!("{}", msg);
}

fn main() {
    print_message("Hello".to_string());  // Unnecessary allocation
}
```

**Why Go developers do this**: Go has one string type. String vs &str confusion is frustrating.

### The Fix

```rust
// ✅ Use &str for parameters, String for ownership
fn concat_strings(a: &str, b: &str) -> String {
    format!("{}{}", a, b)
}

fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let result = concat_strings(&s1, &s2);
    // s1 and s2 still valid!
}

// ✅ Accept &str for functions that just read
fn print_message(msg: &str) {
    println!("{}", msg);
}

fn main() {
    print_message("Hello");  // No allocation needed

    let owned = String::from("World");
    print_message(&owned);   // Works with String too
}

// Rule of thumb:
// - Function parameters: use &str
// - Return values that are new: use String
// - Struct fields for owned data: use String
```

**The lesson**: Default to `&str` for function parameters. Use `String` only when you need ownership.

---

## Reimplementing Go's defer

### The Anti-Pattern

```rust
// ❌ Manually implementing defer-like cleanup
fn process_file(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // ... do work ...

    // Manually close? (File has Drop, but this pattern shows the thinking)
    drop(reader);

    Ok(())
}

// ❌ Using a Vec<Box<dyn Fn()>> to collect cleanup functions
struct DeferStack {
    cleanups: Vec<Box<dyn FnOnce()>>,
}

impl DeferStack {
    fn defer<F: FnOnce() + 'static>(&mut self, f: F) {
        self.cleanups.push(Box::new(f));
    }
}

impl Drop for DeferStack {
    fn drop(&mut self) {
        while let Some(cleanup) = self.cleanups.pop() {
            cleanup();
        }
    }
}
```

**Why Go developers do this**: Go's `defer` is explicit and flexible. Rust's Drop feels hidden.

### The Fix

```rust
// ✅ Trust RAII and Drop
fn process_file(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // ... do work ...

    // File closed automatically when reader goes out of scope
    Ok(())
}

// ✅ For custom cleanup, implement Drop
struct Resource {
    id: u32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("Cleaning up resource {}", self.id);
    }
}

// ✅ For early cleanup, use explicit drop
fn use_resource() {
    let resource = Resource { id: 1 };
    // ... use resource ...
    drop(resource);  // Explicit early cleanup
    // Continue without resource
}

// ✅ For multiple cleanups in order, use scope
fn multiple_resources() {
    let r1 = Resource { id: 1 };
    {
        let r2 = Resource { id: 2 };
        // r2 dropped here
    }
    // r1 dropped at end of function
}
```

**The lesson**: RAII is better than defer. Drop is automatic, predictable, and zero-cost.

---

## Avoiding Lifetimes with 'static

### The Anti-Pattern

```rust
// ❌ Using 'static to avoid lifetime errors
fn get_name() -> &'static str {
    let name = String::from("Alice");
    // Can't return &name because it doesn't live long enough
    // So we leak it to make it 'static
    Box::leak(name.into_boxed_str())  // ❌ Memory leak!
}

// ❌ Requiring 'static when not needed
fn store_callback<F: Fn() + 'static>(callback: F) {
    // Unnecessarily restrictive
}
```

**Why Go developers do this**: Lifetimes are confusing. 'static makes the errors go away.

### The Fix

```rust
// ✅ Return owned data instead
fn get_name() -> String {
    String::from("Alice")
}

// ✅ Use proper lifetimes
fn get_first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// ✅ Lifetime elision works in most cases
fn combine<'a>(a: &'a str, b: &'a str) -> String {
    format!("{} {}", a, b)
}

// Often you can just write:
fn combine_simple(a: &str, b: &str) -> String {
    format!("{} {}", a, b)
}

// ✅ Use generic lifetime when appropriate
fn store_callback<F: Fn()>(callback: F) {
    // No 'static needed if we don't store it beyond the function
    callback();
}
```

**The lesson**: Don't use 'static to avoid learning lifetimes. Most lifetime annotations are simple.

---

## Not Leveraging Zero-Cost Abstractions

### The Anti-Pattern

```rust
// ❌ Writing low-level code when abstractions exist
fn find_max(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let mut max = numbers[0];
    for i in 1..numbers.len() {
        if numbers[i] > max {
            max = numbers[i];
        }
    }
    Some(max)
}

// ❌ Manual error handling when ? exists
fn read_and_parse(path: &str) -> Result<Config, Error> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(Error::from(e)),
    };

    match serde_json::from_str(&content) {
        Ok(config) => Ok(config),
        Err(e) => Err(Error::from(e)),
    }
}
```

**Why Go developers do this**: Go favors explicit code. Abstractions feel like they have overhead.

### The Fix

```rust
// ✅ Use iterator methods (compiles to same code!)
fn find_max(numbers: &[i32]) -> Option<i32> {
    numbers.iter().max().copied()
}

// ✅ Use ? operator for cleaner error handling
fn read_and_parse(path: &str) -> Result<Config, Error> {
    let content = fs::read_to_string(path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

// ✅ Use type system features
fn process_optional(value: Option<i32>) -> Option<i32> {
    value.map(|v| v * 2).filter(|v| *v > 10)
}

// Instead of:
fn process_optional_manual(value: Option<i32>) -> Option<i32> {
    if let Some(v) = value {
        let doubled = v * 2;
        if doubled > 10 {
            Some(doubled)
        } else {
            None
        }
    } else {
        None
    }
}
```

**The lesson**: Rust's abstractions compile to the same code as manual implementations. Use them.

---

## Summary: Breaking Go Habits

### Habits to Break

| Go Habit | Why It's Wrong in Rust | Better Rust Approach |
|----------|------------------------|----------------------|
| Pass pointers everywhere | Rust tracks ownership | Use references (&T) for borrowing |
| Use mutexes for everything | Unnecessary overhead | Try atomics, RwLock, or channels |
| Clone to fix borrow errors | Expensive and wrong | Redesign your data flow |
| Ignore iterators | Missing free optimizations | Use iterator chains |
| `_ = err` (ignore errors) | unwrap() panics in production | Use ?, match, or unwrap_or |
| Use booleans for state | Unclear and error-prone | Use enums with data |
| One string type | Confusion and allocations | &str for parameters, String for ownership |
| Explicit defer calls | Manual and error-prone | Trust RAII and Drop |
| Avoid complex types | Fighting the type system | Embrace enums and traits |
| Write low-level code | Missing optimizations | Use zero-cost abstractions |

### The Meta-Lesson

Go optimizes for simplicity. Rust optimizes for correctness and performance. When you fight Rust's patterns, you're usually fighting safety guarantees that prevent real bugs.

**The Rust way**:
1. Let the compiler guide you
2. Use the type system to encode invariants
3. Trust zero-cost abstractions
4. Embrace ownership instead of fighting it

Most "antipatterns" come from trying to write Go code in Rust. Instead, learn to think in Rust—and you'll write safer, faster code than you could in Go.
