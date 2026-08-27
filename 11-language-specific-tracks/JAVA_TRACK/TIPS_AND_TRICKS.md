# Tips & Tricks for Java Developers

Productivity shortcuts and best practices I wish someone had told me on day one.

## Development Environment

### 1. Use `cargo check` for fast feedback

```bash
# Instead of `cargo build` during development
cargo check  # Just type-checks, no codegen (10x faster)
```

**Why**: When you're iterating quickly, you just want to know if it compiles. Codegen is slow and unnecessary until you run it.

**Workflow**:
- Write code
- Run `cargo check` (fast feedback)
- Once it compiles, run `cargo build` or `cargo run`

### 2. `cargo clippy` is your friend

```bash
cargo clippy  # Like a super-powered linter
```

**Why**: Clippy catches un-idiomatic code and suggests improvements. It's like having a Rust expert review your code.

**Example**:
```rust
// You write:
if x == true { }

// Clippy suggests:
if x { }
```

Run it often—it teaches you Rust idioms faster than any tutorial.

### 3. `cargo fmt` for consistent formatting

```bash
cargo fmt  # Formats code like `black` for Python or Prettier for JS
```

**Why**: Never argue about formatting again. Just run `cargo fmt` and move on.

**Pro tip**: Set up your editor to run it on save.

### 4. Use `rust-analyzer` in your IDE

- **VS Code**: Install "rust-analyzer" extension
- **IntelliJ**: Install "Rust" plugin

**Why**: Inline error messages, autocomplete, go-to-definition—all the things you're used to from IntelliJ IDEA.

**Better than**: The old `RLS` (Rust Language Server). Rust-analyzer is faster and more accurate.

---

## Code Organization

### 5. Keep modules in separate files early

```rust
// lib.rs
mod database;  // Looks for database.rs or database/mod.rs
mod handlers;

// database.rs
pub struct Database { }
impl Database { }
```

**Why**: Easier to navigate than one huge file. Java developers are used to this anyway (one class per file).

### 6. Use `impl` blocks to organize methods

```rust
struct User {
    name: String,
    age: u32,
}

// Constructors
impl User {
    pub fn new(name: String, age: u32) -> Self {
        User { name, age }
    }
}

// Business logic
impl User {
    pub fn can_vote(&self) -> bool {
        self.age >= 18
    }
}

// Trait implementations
impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}
```

**Why**: Multiple `impl` blocks help organize code logically, like how you'd group methods in Java.

---

## Error Handling

### 7. Use the `?` operator liberally

```rust
// Instead of this:
fn read_file(path: &str) -> Result<String, std::io::Error> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}

// Write this:
fn read_file(path: &str) -> Result<String, std::io::Error> {
    Ok(fs::read_to_string(path)?)  // So much cleaner!
}

// Or even simpler:
fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)  // Last expression is returned
}
```

**Think of `?` as**:
- If `Ok(value)`, unwrap and continue
- If `Err(e)`, return early with that error

**Like Java's**: Letting checked exceptions propagate up the call stack, but explicit.

### 8. Use `anyhow` for application errors

```rust
use anyhow::{Result, Context};

fn load_config() -> Result<Config> {
    let content = fs::read_to_string("config.json")
        .context("Failed to read config file")?;

    let config: Config = serde_json::from_str(&content)
        .context("Failed to parse config")?;

    Ok(config)
}
```

**Why**: `anyhow::Result<T>` works with any error type. Perfect for applications (not libraries).

**For libraries**: Use `thiserror` to define custom error types.

---

## Working with Strings

### 9. Use `&str` for function parameters

```rust
// ❌ Don't do this:
fn greet(name: String) { }

// ✅ Do this:
fn greet(name: &str) { }
```

**Why**: Accepts both `String` and `&str`. More flexible.

**Calling**:
```rust
greet("Alice");           // &str literal
greet(&my_string);        // String reference
```

### 10. Format strings with `format!` macro

```rust
// Like Java's String.format()
let name = "Alice";
let age = 30;
let message = format!("{} is {} years old", name, age);

// Or with named arguments
let message = format!("{name} is {age} years old", name = name, age = age);

// Debug formatting (like toString())
let debug = format!("{:?}", some_struct);
```

**Common formats**:
- `{}` - Display
- `{:?}` - Debug
- `{:#?}` - Pretty Debug
- `{:x}` - Hex
- `{:b}` - Binary

### 11. String concatenation with `format!` or `push_str`

```rust
// ❌ Don't use + for multiple strings (slow):
let s = s1 + &s2 + &s3;

// ✅ Use format! for readability:
let s = format!("{}{}{}", s1, s2, s3);

// ✅ Or push_str for performance:
let mut s = String::new();
s.push_str(&s1);
s.push_str(&s2);
s.push_str(&s3);
```

---

## Collections

### 12. Collect into Vec with type annotation

```rust
// Type annotation tells collect() what to build
let numbers: Vec<i32> = (0..10).collect();

// Or turbofish syntax
let numbers = (0..10).collect::<Vec<i32>>();
```

### 13. Use `entry` API for HashMap updates

```rust
use std::collections::HashMap;

let mut map = HashMap::new();

// ❌ Clunky:
if let Some(count) = map.get_mut("key") {
    *count += 1;
} else {
    map.insert("key", 1);
}

// ✅ Clean:
*map.entry("key").or_insert(0) += 1;
```

**Common entry API patterns**:
```rust
map.entry(key).or_insert(default);
map.entry(key).or_insert_with(|| expensive_default());
map.entry(key).and_modify(|v| *v += 1).or_insert(0);
```

---

## Iterators

### 14. Chain iterator methods instead of loops

```rust
// ❌ Java/C-style loop:
let mut result = Vec::new();
for item in items {
    if item > 10 {
        result.push(item * 2);
    }
}

// ✅ Iterator chain:
let result: Vec<_> = items.iter()
    .filter(|&&item| item > 10)
    .map(|item| item * 2)
    .collect();
```

**Common iterator methods** (like Java streams):
- `.map()` - Transform each element
- `.filter()` - Keep elements matching predicate
- `.collect()` - Build a collection
- `.fold()` - Reduce (like Java's reduce)
- `.find()` - Find first match
- `.any()` / `.all()` - Check if any/all match
- `.take(n)` - First n elements
- `.skip(n)` - Skip first n elements

### 15. Use `.iter()`, `.iter_mut()`, or `.into_iter()` correctly

```rust
let vec = vec![1, 2, 3];

// Borrow (read-only)
for item in vec.iter() {  // item is &i32
    println!("{}", item);
}

// Borrow mutably
for item in vec.iter_mut() {  // item is &mut i32
    *item *= 2;
}

// Consume (take ownership)
for item in vec.into_iter() {  // item is i32, vec is consumed
    println!("{}", item);
}
// vec is no longer available here
```

**Shortcut**: `for item in &vec` is same as `vec.iter()`

---

## Pattern Matching

### 16. Use `if let` for single-case matches

```rust
// ❌ Verbose:
match option {
    Some(value) => println!("{}", value),
    None => {},
}

// ✅ Concise:
if let Some(value) = option {
    println!("{}", value);
}
```

### 17. Match with guards

```rust
match number {
    n if n < 0 => println!("negative"),
    0 => println!("zero"),
    n if n < 10 => println!("small"),
    _ => println!("large"),
}
```

### 18. Destructure in function parameters

```rust
// Instead of this:
fn process_point(point: (i32, i32)) {
    let x = point.0;
    let y = point.1;
}

// Do this:
fn process_point((x, y): (i32, i32)) {
    // x and y are already extracted
}
```

---

## Debugging

### 19. Use `dbg!` macro for quick debugging

```rust
// Prints value and returns it
let x = 5;
dbg!(x);  // Prints: [src/main.rs:2] x = 5

// Chain in expressions
let result = dbg!(expensive_computation());

// Multiple values
dbg!(x, y, z);
```

**Better than** `println!("{:?}", x)` because it shows location and variable name.

### 20. Derive `Debug` for your types

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

let user = User { name: "Alice".to_string(), age: 30 };
println!("{:?}", user);  // User { name: "Alice", age: 30 }
println!("{:#?}", user);  // Pretty-printed
```

**Pro tip**: Always derive `Debug` unless you have a reason not to.

---

## Performance

### 21. Use `cargo build --release` for benchmarking

```bash
# Debug build (default, slow):
cargo build
cargo run

# Release build (optimized):
cargo build --release
cargo run --release
```

**Why**: Debug builds are 10-100x slower. Always benchmark with `--release`.

### 22. Avoid allocations in hot loops

```rust
// ❌ Allocates every iteration:
for i in 0..1000000 {
    let s = format!("Item {}", i);  // Allocation!
}

// ✅ Reuse buffer:
let mut buffer = String::new();
for i in 0..1000000 {
    buffer.clear();
    use std::fmt::Write;
    write!(&mut buffer, "Item {}", i).unwrap();
}
```

### 23. Use `Vec::with_capacity` if you know the size

```rust
// ❌ Reallocates as it grows:
let mut vec = Vec::new();
for i in 0..1000 {
    vec.push(i);
}

// ✅ Pre-allocate:
let mut vec = Vec::with_capacity(1000);
for i in 0..1000 {
    vec.push(i);  // No reallocation needed
}
```

---

## Testing

### 24. Write tests in the same file

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
        panic!("expected panic");
    }
}
```

**Run tests**:
```bash
cargo test
cargo test test_name  # Run specific test
cargo test -- --nocapture  # Show println! output
```

### 25. Use `assert_eq!` and `assert_ne!`

```rust
assert_eq!(actual, expected);  // Better error messages than assert!
assert_ne!(actual, not_this);
```

---

## Derive Macros

### 26. Derive common traits

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    name: String,
    age: u32,
}
```

**Common derives**:
- `Debug` - Print with `{:?}`
- `Clone` - Deep copy with `.clone()`
- `Copy` - Implicit copy (only for types with no heap data)
- `PartialEq` / `Eq` - Equality comparison
- `PartialOrd` / `Ord` - Ordering comparison
- `Hash` - Use in HashMap/HashSet
- `Default` - Default values

**Serde derives** (for JSON/YAML/etc):
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
}
```

---

## Cargo Tricks

### 27. Use `cargo watch` for auto-recompile

```bash
cargo install cargo-watch
cargo watch -x check  # Runs cargo check on file changes
cargo watch -x test   # Runs tests on file changes
```

**Like**: Hot reload in Node.js or Spring Boot DevTools.

### 28. Use `cargo tree` to debug dependencies

```bash
cargo tree  # Shows dependency tree
cargo tree -i crate_name  # Inverse: who depends on this?
```

### 29. Speed up compile times

```rust
// Cargo.toml
[profile.dev]
opt-level = 1  # Slight optimization in debug builds

[profile.dev.package."*"]
opt-level = 3  # Optimize dependencies but not your code
```

---

## Quick Wins

### 30. Learn these keyboard shortcuts (VS Code)

- `Ctrl+Space` - Autocomplete
- `F12` - Go to definition
- `Shift+F12` - Find references
- `F2` - Rename symbol
- `Ctrl+.` - Quick fix (trust me, use this!)

### 31. Use Rust Playground for quick experiments

https://play.rust-lang.org/

**Why**: No local setup needed. Great for testing snippets or sharing code.

### 32. Read the compiler errors carefully

Rust error messages are genuinely helpful. They often suggest the fix:

```
error[E0382]: borrow of moved value: `s`
  --> src/main.rs:4:20
   |
2  |     let s = String::from("hello");
   |         - move occurs because `s` has type `String`
3  |     let t = s;
   |             - value moved here
4  |     println!("{}", s);
   |                    ^ value borrowed here after move
   |
help: consider cloning the value
   |
3  |     let t = s.clone();
   |              ++++++++
```

The compiler literally told you to use `.clone()`!

---

## Bonus: Java → Rust Mental Model

| When you think... | Think in Rust... |
|-------------------|------------------|
| "Create an object" | "Who owns this data?" |
| "Pass a reference" | "Am I borrowing or moving?" |
| "Null check" | "Pattern match on Option" |
| "Try-catch" | "Match on Result or use ?" |
| "Synchronized" | "Use Mutex or channels" |
| "Interface" | "Trait" |
| "ArrayList" | "Vec" |
| "Stream API" | "Iterator chains" |

---

**Remember**: Rust rewards you for thinking about ownership and lifetimes upfront. The time you "waste" making the compiler happy is time saved debugging production issues.

Good luck, and welcome to Rust! 🦀
