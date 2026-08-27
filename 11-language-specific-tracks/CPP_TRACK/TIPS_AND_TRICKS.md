# Tips and Tricks for C/C++ Developers

Productivity tips, debugging techniques, and hidden gems for C/C++ developers learning Rust.

## Development Environment

### Essential Tools

```bash
# Install Rust (if you haven't already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Update Rust
rustup update

# Install useful components
rustup component add rustfmt      # Code formatter
rustup component add clippy       # Linter
rustup component add rust-analyzer # LSP for IDEs

# Install cargo tools
cargo install cargo-watch         # Auto-rebuild on file changes
cargo install cargo-expand        # See macro expansions
cargo install cargo-edit          # Add/remove dependencies easily
cargo install cargo-audit         # Security vulnerability scanner
cargo install cargo-outdated      # Check for outdated dependencies
```

### IDE Setup

**VS Code** (most popular):
```json
// .vscode/settings.json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.loadOutDirsFromCheck": true,
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

**CLion** (JetBrains):
- Install Rust plugin
- Works great, similar to CLion for C++

**Vim/Neovim**:
```vim
" Using coc.nvim
Plug 'neoclide/coc.nvim', {'branch': 'release'}
:CocInstall coc-rust-analyzer
```

### Compiler vs GCC/Clang Flags

| GCC/Clang | Rust (cargo) | Notes |
|-----------|--------------|-------|
| `-O0` | `cargo build` | Debug mode (default) |
| `-O2` / `-O3` | `cargo build --release` | Release mode |
| `-g` | Enabled by default | Debug symbols |
| `-Wall -Wextra` | `clippy` | More thorough linting |
| `-fsanitize=address` | `RUSTFLAGS="-Z sanitizer=address"` | Address sanitizer (nightly) |
| `-fsanitize=thread` | `RUSTFLAGS="-Z sanitizer=thread"` | Thread sanitizer (nightly) |

## Cargo Shortcuts

### Everyday Commands

```bash
# New project
cargo new my_project         # Binary (main.rs)
cargo new --lib my_lib       # Library (lib.rs)

# Build and run
cargo build                  # Debug build
cargo build --release        # Release build (optimized)
cargo run                    # Build + run
cargo run --release          # Build release + run

# Quick compile check (faster than build)
cargo check                  # Just check if it compiles

# Testing
cargo test                   # Run all tests
cargo test test_name         # Run specific test
cargo test --release         # Test release build

# Documentation
cargo doc --open             # Build + open docs in browser

# Linting and formatting
cargo fmt                    # Format code
cargo clippy                 # Lint with helpful suggestions
cargo clippy -- -W clippy::pedantic  # Extra pedantic lints

# Dependencies
cargo add serde              # Add dependency (needs cargo-edit)
cargo rm serde               # Remove dependency
cargo update                 # Update dependencies
```

### Hidden Gems

```bash
# Watch for changes and auto-rebuild
cargo watch -x run
cargo watch -x test
cargo watch -x "run --release"

# See expanded macros
cargo expand
cargo expand module::function

# Benchmark (nightly only)
cargo bench

# Tree of dependencies
cargo tree

# Why is this dependency included?
cargo tree -i dependency_name

# Build for specific target
cargo build --target x86_64-pc-windows-gnu

# Check binary size
cargo bloat --release

# Time compilation
cargo build --timings
```

## Debugging Techniques

### Print Debugging

```rust
// C++ style
std::cout << "x = " << x << std::endl;

// Rust equivalents
println!("x = {}", x);           // Display trait
println!("x = {:?}", x);         // Debug trait (auto-derived)
println!("x = {:#?}", x);        // Pretty-print Debug
dbg!(x);                         // Prints value + file:line

// dbg! macro returns the value, so you can insert it anywhere
let y = dbg!(x * 2);

// Example
let point = Point { x: 3, y: 4 };
dbg!(&point);  // Prints: [src/main.rs:10] &point = Point { x: 3, y: 4 }
```

### Using GDB/LLDB

```bash
# Install rust-gdb wrapper
rustup component add rust-src

# Debug with gdb
rust-gdb target/debug/my_program

# Debug with lldb (Mac)
rust-lldb target/debug/my_program

# Common gdb commands (same as C++)
(gdb) break main
(gdb) run
(gdb) next
(gdb) step
(gdb) print variable
(gdb) backtrace
```

### Compiler Error Messages

Rust has amazing error messages. Read them carefully!

```rust
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:5:20
  |
3 |     let s = String::from("hello");
  |         - move occurs because `s` has type `String`,
  |           which does not implement the `Copy` trait
4 |     let t = s;
  |             - value moved here
5 |     println!("{}", s);
  |                    ^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
4 |     let t = s.clone();
  |              ++++++++
```

**Tip:** The compiler often suggests fixes. Try them!

### Debugging Ownership Issues

```rust
// Use clippy to catch common mistakes
cargo clippy

// Enable more warnings
#![warn(clippy::all, clippy::pedantic)]

// See what type the compiler inferred
let x = vec![1, 2, 3];
let _: () = x;  // ERROR: expected (), found Vec<i32>
// Compiler tells you the type!

// Check lifetimes
cargo rustc -- -Z unpretty=hir  // Nightly only
```

## Performance Tips

### Profile Before Optimizing

```bash
# Install flamegraph
cargo install flamegraph

# Profile with perf (Linux)
cargo flamegraph

# Or use perf directly
cargo build --release
perf record -g target/release/my_program
perf report

# Valgrind (works with Rust!)
valgrind --tool=callgrind target/release/my_program
kcachegrind callgrind.out.*
```

### Common Optimizations

```rust
// 1. Avoid unnecessary allocations
// ❌ BAD
fn process(s: String) -> String {
    s.to_uppercase()
}

// ✅ GOOD
fn process(s: &str) -> String {
    s.to_uppercase()
}

// 2. Use iterator chains (zero-cost)
// ❌ BAD
let mut result = Vec::new();
for i in 0..1000 {
    if i % 2 == 0 {
        result.push(i * 2);
    }
}

// ✅ GOOD (compiles to same code, more readable)
let result: Vec<i32> = (0..1000)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();

// 3. Preallocate vectors
// ❌ BAD
let mut v = Vec::new();
for i in 0..1000 {
    v.push(i);  // Reallocates multiple times
}

// ✅ GOOD
let mut v = Vec::with_capacity(1000);
for i in 0..1000 {
    v.push(i);  // No reallocations
}

// 4. Use &str instead of String when possible
// ❌ BAD
fn get_extension(path: String) -> String {
    // ...allocations everywhere...
}

// ✅ GOOD
fn get_extension(path: &str) -> &str {
    // No allocations
}

// 5. Avoid cloning in hot loops
// ❌ BAD
for item in &items {
    process(item.clone());  // Heap allocation every iteration
}

// ✅ GOOD
for item in &items {
    process(item);  // Just borrow
}
```

### Release Mode Optimizations

```toml
# Cargo.toml
[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization, slower compile
opt-level = 3           # Maximum optimization
```

### Inline Hints

```rust
// Force inline (like __attribute__((always_inline)))
#[inline(always)]
fn hot_function() {
    // ...
}

// Suggest inline (like inline keyword)
#[inline]
fn might_inline() {
    // ...
}

// Never inline
#[inline(never)]
fn cold_function() {
    // ...
}
```

## Testing Tricks

### Writing Tests

```rust
// Unit tests (same file as code)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        panic!("This should panic");
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn specific_panic() {
        let _ = 1 / 0;
    }

    #[test]
    fn returns_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Math is broken"))
        }
    }
}

// Integration tests (tests/ directory)
// tests/integration_test.rs
use my_crate;

#[test]
fn test_public_api() {
    // Test public interface
}
```

### Test Organization

```bash
# Run specific test
cargo test test_name

# Run all tests in a module
cargo test module_name::

# Run tests matching pattern
cargo test add_

# Show println! output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=4

# Run ignored tests
cargo test -- --ignored

# Run doc tests only
cargo test --doc
```

### Benchmarking

```rust
// Nightly only
#![feature(test)]
extern crate test;

#[cfg(test)]
mod benches {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_function(b: &mut Bencher) {
        b.iter(|| {
            // Code to benchmark
            expensive_operation()
        });
    }
}
```

Or use `criterion` crate (stable Rust):

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "my_benchmark"
harness = false
```

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

## Working with C/C++

### FFI Basics

```rust
// Calling C from Rust
extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe {
    let x = abs(-42);
}

// Exposing Rust to C
#[no_mangle]
pub extern "C" fn rust_function(x: i32) -> i32 {
    x * 2
}
```

### Using bindgen

```bash
# Install bindgen
cargo install bindgen

# Generate bindings
bindgen header.h -o bindings.rs
```

```rust
// build.rs
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=mylib");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
```

### Calling C++

```toml
[build-dependencies]
cc = "1.0"
```

```rust
// build.rs
fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/wrapper.cpp")
        .compile("wrapper");
}
```

## Documentation

### Writing Good Docs

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// This function panics if the sum would overflow.
///
/// # Errors
///
/// Returns `Err` if...
///
/// # Safety
///
/// This function is unsafe because...
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Module-level docs
//! This module contains utilities for...

// Private item doc
/// Internal helper function.
fn helper() {}
```

### Doc Tests

```rust
/// ```
/// use my_crate::*;
/// let x = add(2, 3);
/// assert_eq!(x, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Run doc tests
// cargo test --doc
```

## Dependency Management

### Cargo.toml Tips

```toml
[dependencies]
# Specific version
serde = "1.0.192"

# Any compatible version
serde = "1.0"

# Minimum version
serde = ">=1.0"

# From git
my-lib = { git = "https://github.com/user/repo" }

# From local path
my-lib = { path = "../my-lib" }

# Optional dependencies
serde = { version = "1.0", optional = true }

# Features
serde = { version = "1.0", features = ["derive"] }

# Platform-specific
[target.'cfg(windows)'.dependencies]
winapi = "0.3"

# Dev dependencies (tests/benches only)
[dev-dependencies]
criterion = "0.5"
```

### Version Selection

```bash
# Update minor/patch versions
cargo update

# Update to latest (even major versions)
cargo update --aggressive

# Update specific dependency
cargo update -p serde

# Check for outdated deps
cargo outdated
```

## Miscellaneous Tips

### Error Handling Shortcuts

```rust
use anyhow::Result;  // Instead of Result<T, Box<dyn Error>>

fn main() -> Result<()> {
    let data = std::fs::read_to_string("file.txt")?;
    Ok(())
}
```

### Iterating Idioms

```rust
// Iterate with index
for (i, item) in items.iter().enumerate() {
    println!("{}: {}", i, item);
}

// Windows (sliding window)
for window in items.windows(2) {
    println!("{:?}", window);
}

// Chunks
for chunk in items.chunks(3) {
    println!("{:?}", chunk);
}

// Zip two iterators
for (a, b) in iter1.zip(iter2) {
    println!("{} {}", a, b);
}
```

### Type Aliases

```rust
// Like typedef in C++
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn read_file() -> Result<String> {
    // ...
}
```

### Turbofish Syntax

```rust
// When the compiler can't infer type
let numbers: Vec<i32> = vec!["1", "2", "3"]
    .iter()
    .map(|s| s.parse::<i32>().unwrap())
    .collect();

// Or use turbofish on collect
let numbers = vec!["1", "2", "3"]
    .iter()
    .map(|s| s.parse().unwrap())
    .collect::<Vec<i32>>();
```

### Custom Derive

```rust
// Auto-implement traits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
```

## Learning Resources

### Official Documentation

- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Rustlings** (exercises): https://github.com/rust-lang/rustlings
- **Standard Library**: https://doc.rust-lang.org/std/

### C/C++ to Rust Guides

- **Rust for C++ Programmers**: https://github.com/nrc/r4cppp
- **From C to Rust**: https://github.com/immunant/c2rust
- **Nomicon** (unsafe Rust): https://doc.rust-lang.org/nomicon/

### Community

- **r/rust** on Reddit
- **Rust Discord**: https://discord.gg/rust-lang
- **This Week in Rust**: https://this-week-in-rust.org/
- **Rust Users Forum**: https://users.rust-lang.org/

## Quick Reference Card

Print this out and keep it nearby:

```
COMMON PATTERNS:

Error handling:    result?
Optional value:    if let Some(x) = opt { }
Pattern matching:  match value { pattern => expr }
Iterate:          for item in &collection { }
Mutate:           for item in &mut collection { }
Consume:          for item in collection { }

LIFETIMES:

'a                Lifetime parameter
&'a T             Reference with lifetime 'a
&'static          Lives for entire program

SMART POINTERS:

Box<T>            Heap allocation, unique ownership
Rc<T>             Reference counted (single-thread)
Arc<T>            Atomic reference counted (multi-thread)
RefCell<T>        Interior mutability (runtime checks)
Mutex<T>          Thread-safe interior mutability

TRAITS:

Clone             Explicit copy (.clone())
Copy              Implicit copy (only simple types)
Debug             Debug formatting ({:?})
Display           User-facing formatting ({})
Default           Default value
Drop              Destructor
```

---

**Remember:** The Rust compiler is your friend. When it rejects your code, it's trying to help you avoid bugs. Read the error messages, they're incredibly helpful!
