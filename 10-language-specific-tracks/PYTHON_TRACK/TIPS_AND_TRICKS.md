# Tips & Tricks for Python Developers

Productivity shortcuts and best practices I wish someone had told me when I started Rust.

## Development Environment

### 1. Use `cargo check` for fast feedback

```bash
# Instead of `cargo build` during development
cargo check  # Just type-checks, no codegen (10x faster!)
```

**Why**: Python has instant feedback with `python script.py`. Rust compilation is slower, but `cargo check` gives you quick type-checking without generating binaries.

**Workflow**:
- Write code
- Run `cargo check` (fast feedback)
- Once it type-checks, run `cargo run`

### 2. `cargo clippy` is your linter

```bash
cargo clippy  # Like `pylint` or `flake8` but better
```

**Why**: Clippy catches un-idiomatic Rust and suggests improvements. It's like having a Rust expert review your code.

**Example**:
```rust
// You write:
if x == true { }

// Clippy suggests:
if x { }
```

Run it often—it teaches Rust idioms faster than any tutorial!

### 3. `cargo fmt` for code formatting

```bash
cargo fmt  # Like `black` for Python
```

**Why**: Never argue about formatting. Just run `cargo fmt` before committing.

**Setup auto-format on save** in VS Code:
```json
{
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

### 4. Use `rust-analyzer` (not RLS)

**VS Code**:
```bash
code --install-extension rust-lang.rust-analyzer
```

**Why**: Inline errors, autocomplete, go-to-definition. Like PyCharm for Python.

**Better than**: The old RLS (Rust Language Server). Rust-analyzer is faster and more accurate.

### 5. Install `cargo-watch` for auto-rebuild

```bash
cargo install cargo-watch

# Auto-check on file changes
cargo watch -x check

# Auto-run tests
cargo watch -x test

# Auto-run
cargo watch -x run
```

**Like**: `nodemon` for Node.js or Python's auto-reload in Flask/Django.

---

## Cargo Tricks

### 6. Useful cargo commands

```bash
# Development
cargo check          # Fast type-checking
cargo build          # Debug build
cargo run            # Build and run
cargo test           # Run tests
cargo bench          # Run benchmarks

# Production
cargo build --release    # Optimized build (10-100x faster)
cargo run --release      # Run optimized binary

# Maintenance
cargo clean          # Remove build artifacts
cargo update         # Update dependencies
cargo tree           # Show dependency tree
```

### 7. Speed up compile times

Add to `~/.cargo/config.toml`:

```toml
[build]
# Use all CPU cores
jobs = 8

[profile.dev]
# Slight optimization in debug builds
opt-level = 1

[profile.dev.package."*"]
# Optimize dependencies but not your code
opt-level = 3
```

### 8. Use `cargo add` for dependencies

```bash
cargo install cargo-edit

# Add dependency
cargo add serde --features derive
cargo add tokio --features full

# Add dev dependency
cargo add --dev criterion
```

**Like**: `pip install` but for `Cargo.toml`.

---

## Code Patterns

### 9. Iterator shortcuts

```rust
// Instead of this:
let mut result = Vec::new();
for item in items {
    if item > 0 {
        result.push(item * 2);
    }
}

// Write this:
let result: Vec<_> = items.iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * 2)
    .collect();
```

**Common iterator methods**:
- `.map(|x| ...)` - Transform
- `.filter(|x| ...)` - Keep matching
- `.collect()` - Build collection
- `.sum()`, `.product()` - Aggregate
- `.find(|x| ...)` - Find first
- `.any(|x| ...)`, `.all(|x| ...)` - Check conditions
- `.take(n)`, `.skip(n)` - Limit/skip
- `.enumerate()` - Add indices
- `.zip(other)` - Pair with another iterator

### 10. The `?` operator is your friend

```rust
// Instead of this:
fn read_file() -> Result<String, std::io::Error> {
    match std::fs::read_to_string("file.txt") {
        Ok(contents) => Ok(contents),
        Err(e) => Err(e),
    }
}

// Write this:
fn read_file() -> Result<String, std::io::Error> {
    Ok(std::fs::read_to_string("file.txt")?)
}

// Even simpler:
fn read_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string("file.txt")
}
```

### 11. Use `anyhow` for application errors

```toml
[dependencies]
anyhow = "1.0"
```

```rust
use anyhow::{Result, Context};

fn load_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.json")
        .context("Failed to read config file")?;

    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config")?;

    Ok(config)
}
```

**Why**: Like Python exceptions with context. Perfect for applications.

**For libraries**: Use `thiserror` to define custom error types.

---

## String Handling

### 12. String conversion shortcuts

```rust
// &str to String
let owned = "hello".to_string();
let owned = String::from("hello");
let owned = "hello".to_owned();

// String to &str
let slice: &str = &owned;
let slice = owned.as_str();

// Format string (like f-strings)
let name = "Alice";
let greeting = format!("Hello, {}!", name);

// Into String (generic)
fn accepts_string(s: impl Into<String>) {
    let s: String = s.into();
}

accepts_string("hello");  // &str works
accepts_string(String::from("hello"));  // String works
```

### 13. Common string operations

```rust
let s = "  hello world  ";

s.trim()                    // "hello world"
s.to_uppercase()            // "  HELLO WORLD  "
s.to_lowercase()            // "  hello world  "
s.replace("world", "Rust")  // "  hello Rust  "
s.split_whitespace()        // Iterator: ["hello", "world"]
s.starts_with("  h")        // true
s.contains("world")         // true

// Join (like Python's str.join)
let words = vec!["hello", "world"];
let sentence = words.join(" ");  // "hello world"
```

---

## Debugging

### 14. Use `dbg!()` macro

```rust
// Prints value and location, returns value
let x = 5;
dbg!(x);  // [src/main.rs:2] x = 5

// Chain in expressions
let result = dbg!(expensive_computation());

// Multiple values
dbg!(x, y, z);
```

**Better than**: `println!("{:?}", x)` because it shows location and variable name.

### 15. Derive `Debug` for your types

```rust
#[derive(Debug)]  // Automatic Debug implementation
struct User {
    name: String,
    age: u32,
}

let user = User { name: "Alice".to_string(), age: 30 };
println!("{:?}", user);   // User { name: "Alice", age: 30 }
println!("{:#?}", user);  // Pretty-printed
```

**Always derive `Debug`** unless you have a reason not to!

### 16. Use REPL for experimentation

```bash
# Install Rust REPL
cargo install evcxr_repl

# Run it
evcxr
```

**Like**: IPython for Rust. Great for trying small snippets.

---

## Error Handling

### 17. Match for explicit handling

```rust
match result {
    Ok(value) => {
        // Use value
    }
    Err(e) => {
        eprintln!("Error: {}", e);
        // Handle error
    }
}
```

### 18. `if let` for single case

```rust
// Instead of:
match maybe_value {
    Some(v) => println!("{}", v),
    None => {},
}

// Write:
if let Some(v) = maybe_value {
    println!("{}", v);
}
```

### 19. Unwrap variants

```rust
let maybe: Option<i32> = Some(5);

maybe.unwrap()              // Panics if None
maybe.expect("no value!")   // Panics with message
maybe.unwrap_or(0)          // Default value
maybe.unwrap_or_else(|| compute_default())  // Lazy default
maybe.unwrap_or_default()   // Type's default (0 for numbers)
```

---

## Testing

### 20. Tests in same file

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Expected panic");
    }
}
```

**Run tests**:
```bash
cargo test                      # All tests
cargo test test_add             # Specific test
cargo test -- --nocapture       # Show println! output
cargo test -- --test-threads=1  # Run sequentially
```

### 21. Use `assert_eq!` and `assert_ne!`

```rust
assert_eq!(actual, expected);     // Better error messages
assert_ne!(actual, not_expected);
assert!(condition);               // Boolean assertion
```

---

## Performance

### 22. Always benchmark with `--release`

```bash
# Debug build (slow, good for debugging)
cargo run

# Release build (10-100x faster!)
cargo run --release
```

**Never** benchmark debug builds. They're intentionally slow with debug info.

### 23. Use `criterion` for benchmarks

```toml
[dev-dependencies]
criterion = "0.5"
```

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### 24. Avoid allocations in hot loops

```rust
// ❌ Slow: Allocates every iteration
for i in 0..1000000 {
    let s = format!("Item {}", i);  // Allocation!
}

// ✅ Fast: Reuse buffer
let mut buffer = String::new();
for i in 0..1000000 {
    buffer.clear();
    use std::fmt::Write;
    write!(&mut buffer, "Item {}", i).unwrap();
}
```

### 25. Pre-allocate collections

```rust
// ❌ Slow: Reallocates as it grows
let mut vec = Vec::new();
for i in 0..1000 {
    vec.push(i);
}

// ✅ Fast: Pre-allocate
let mut vec = Vec::with_capacity(1000);
for i in 0..1000 {
    vec.push(i);  // No reallocation
}
```

---

## Common Derive Macros

### 26. Useful derives

```rust
#[derive(Debug)]           // Print with {:?}
#[derive(Clone)]           // .clone() method
#[derive(Copy)]            // Implicit copy (only for stack types)
#[derive(PartialEq, Eq)]   // == and != operators
#[derive(PartialOrd, Ord)] // <, >, <=, >= operators
#[derive(Hash)]            // Use in HashMap/HashSet
#[derive(Default)]         // Default::default()

// Serde (JSON, YAML, etc.)
#[derive(Serialize, Deserialize)]

// All at once
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MyType {
    field: String,
}
```

---

## Python to Rust Common Operations

### 27. File I/O

```rust
use std::fs;

// Read entire file
let content = fs::read_to_string("file.txt")?;

// Write entire file
fs::write("file.txt", "content")?;

// Read lines
use std::io::{BufRead, BufReader};
let file = fs::File::open("file.txt")?;
for line in BufReader::new(file).lines() {
    println!("{}", line?);
}
```

### 28. JSON handling

```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

// Serialize
let person = Person { name: "Alice".to_string(), age: 30 };
let json = serde_json::to_string(&person)?;

// Deserialize
let person: Person = serde_json::from_str(&json)?;

// Pretty print
let json = serde_json::to_string_pretty(&person)?;
```

### 29. HTTP requests

```rust
use reqwest;

// Synchronous
let response = reqwest::blocking::get("https://api.example.com")?;
let body = response.text()?;

// Asynchronous
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.example.com").await?;
    let body = response.text().await?;
    Ok(())
}
```

---

## IDE Shortcuts (VS Code)

### 30. Essential keyboard shortcuts

- `Ctrl+Space` - Autocomplete
- `F12` - Go to definition
- `Shift+F12` - Find all references
- `F2` - Rename symbol
- `Ctrl+.` - Quick fix (use this!)
- `Ctrl+Shift+P` - Command palette

### 31. Rust-specific tricks

- Hover over variables to see types
- `Ctrl+Click` on types to see definitions
- Use "Expand macro" command to see what macros generate
- "Rust Analyzer: View Hir" to see compiler's view

---

## Quick Wins

### 32. Use `?` instead of `unwrap()` in functions

```rust
// ❌ Before
fn process() {
    let data = read_file().unwrap();
}

// ✅ After
fn process() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_file()?;
    Ok(())
}
```

### 33. Use pattern matching in function parameters

```rust
// Instead of:
fn print_point(point: (i32, i32)) {
    let x = point.0;
    let y = point.1;
    println!("({}, {})", x, y);
}

// Write:
fn print_point((x, y): (i32, i32)) {
    println!("({}, {})", x, y);
}
```

### 34. Use `_` to ignore unused variables

```rust
// Warning: unused variable
let result = expensive_computation();

// No warning
let _result = expensive_computation();
let _ = expensive_computation();  // Also drops immediately
```

---

## Python Developer Mental Model

When you think... | Think in Rust...
|------------------|------------------|
| `if x:` | `if x != 0 {` or `if !x.is_empty() {` |
| `for item in items:` | `for item in &items {` |
| `list comprehension` | `items.iter().map().filter().collect()` |
| `try/except` | `match result { Ok/Err }` or `?` |
| `with open() as f:` | RAII (automatic cleanup) |
| `x = None` | `let x: Option<T> = None;` |
| `def func(x=5):` | No default params, use `Option` |

---

## Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings**: https://github.com/rust-lang/rustlings
- **Rust Playground**: https://play.rust-lang.org/
- **Crates.io**: https://crates.io/ (like PyPI)
- **Docs.rs**: https://docs.rs/ (crate documentation)

---

**Remember**: Rust has a steeper learning curve than Python, but the productivity gains come from fewer bugs and better performance. These tips will help you get there faster!

Good luck! 🦀
