# Tips & Tricks: Productivity Guide for Go Developers

Make your Rust journey smoother with these tips. Coming from Go, you'll appreciate some of these workflows.

## Table of Contents

1. [Development Environment](#development-environment)
2. [Cargo Shortcuts](#cargo-shortcuts)
3. [Debugging Techniques](#debugging-techniques)
4. [IDE Tips](#ide-tips)
5. [Common Patterns](#common-patterns)
6. [Performance Tips](#performance-tips)
7. [Testing Tricks](#testing-tricks)
8. [Documentation](#documentation)
9. [Learning Resources](#learning-resources)

---

## Development Environment

### Essential Tools

```bash
# Install Rust (if you haven't)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Essential tools
rustup component add rustfmt  # Like gofmt
rustup component add clippy   # Like go vet, but better

# Update Rust
rustup update
```

### Editor Setup

**VS Code** (most popular):
```bash
# Install rust-analyzer extension
code --install-extension rust-lang.rust-analyzer
```

**Neovim**:
```lua
-- With LSP config
require'lspconfig'.rust_analyzer.setup{}
```

**IntelliJ/CLion**:
- Install Rust plugin from JetBrains marketplace

---

## Cargo Shortcuts

### Quick Commands (Like Go's toolchain)

| Task | Go | Rust |
|------|-----|------|
| Build | `go build` | `cargo build` |
| Run | `go run main.go` | `cargo run` |
| Test | `go test` | `cargo test` |
| Format | `gofmt -w .` | `cargo fmt` |
| Lint | `go vet` | `cargo clippy` |
| Docs | `godoc` | `cargo doc --open` |
| Add dependency | Edit `go.mod` | `cargo add <crate>` |
| Update deps | `go get -u` | `cargo update` |

### Faster Development Workflow

```bash
# Fast check without codegen (MUCH faster than cargo build)
cargo check

# Check + run tests + clippy in one command
cargo test && cargo clippy

# Auto-run on file changes (like using air/realize in Go)
cargo install cargo-watch
cargo watch -x check -x test

# Faster builds for development
cargo build --release  # Optimized builds (slower compile, fast runtime)
```

### cargo.toml Tips

```toml
[profile.dev]
opt-level = 1  # Slightly optimize debug builds (faster runtime)

[profile.dev.package."*"]
opt-level = 3  # Fully optimize dependencies (once)

# Faster linking (macOS/Linux)
[profile.dev]
split-debuginfo = "unpacked"
```

---

## Debugging Techniques

### Print Debugging

```rust
// Go style
println!("x = {}", x);

// Debug format (like %v in Go)
println!("{:?}", complex_struct);

// Pretty debug
println!("{:#?}", complex_struct);

// Debug macro (prints file & line!)
dbg!(x);
dbg!(&my_vec);

// Multiple values
dbg!(x, y, z);
```

### Better than fmt.Printf

```rust
// This:
dbg!(user.age * 2);

// Prints:
// [src/main.rs:12] user.age * 2 = 60

// Shows the expression AND the value!
```

### Error Context

```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let content = fs::read_to_string("config.json")
        .context("reading config.json")?;

    let config: Config = serde_json::from_str(&content)
        .context("parsing config JSON")?;

    Ok(config)
}

// Error output shows full context chain!
// Error: reading config.json
// Caused by: No such file or directory (os error 2)
```

### Panic Backtraces

```bash
# Get full backtrace on panic (like Go's default)
RUST_BACKTRACE=1 cargo run

# Even more detailed
RUST_BACKTRACE=full cargo run
```

### Using a Debugger

```bash
# Install lldb/gdb
# VS Code: Use CodeLLDB extension
# Run with debugger
rust-lldb target/debug/myapp

# Or use rust-gdb
rust-gdb target/debug/myapp
```

---

## IDE Tips

### rust-analyzer Features

**Go to Definition** (like in Go):
- Ctrl/Cmd + Click on any identifier
- Works across crates!

**Auto-import** (better than gopls):
- Type a struct/function name
- rust-analyzer suggests imports
- Press Tab to accept

**Inline Type Hints**:
```rust
let x = vec![1, 2, 3];  // rust-analyzer shows: Vec<i32>
```

**Expand Macros**:
- Right-click on macro → "Expand macro recursively"
- See what `println!` actually does!

### Keyboard Shortcuts

**VS Code**:
- `Ctrl+Shift+P` → "Rust Analyzer: Expand Macro"
- `F2` → Rename (updates all references)
- `Ctrl+.` → Quick fix (auto-implement trait, etc.)

**IntelliJ**:
- `Alt+Enter` → Quick fix
- `Ctrl+Shift+A` → Find action
- `Ctrl+B` → Go to definition

---

## Common Patterns

### Option and Result Combinators

```rust
// Instead of this:
let x = match some_option {
    Some(v) => v * 2,
    None => 0,
};

// Use map and unwrap_or:
let x = some_option.map(|v| v * 2).unwrap_or(0);

// Chain operations:
let result = some_option
    .filter(|x| *x > 0)
    .map(|x| x * 2)
    .unwrap_or_default();
```

### Converting Types

```rust
// String conversions
let s: String = "hello".to_string();
let s: String = String::from("hello");
let s: String = format!("hello");

// Into trait (generic conversion)
fn takes_string(s: impl Into<String>) {
    let owned = s.into();
}
takes_string("hello");           // Works!
takes_string(String::from("x")); // Also works!

// From trait
let s = String::from("hello");
let v = Vec::from([1, 2, 3]);
```

### Working with Slices

```rust
// Vec to slice
let vec = vec![1, 2, 3];
let slice: &[i32] = &vec;
let slice: &[i32] = &vec[1..3];  // Subset

// Array to slice
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..4];  // [2, 3, 4]

// Slice methods
let first = slice.first();  // Option<&i32>
let last = slice.last();
let contains = slice.contains(&3);
```

### Collecting Iterators

```rust
// Collect to Vec
let v: Vec<i32> = (0..10).collect();

// Collect to HashMap
use std::collections::HashMap;
let map: HashMap<_, _> = vec![("a", 1), ("b", 2)]
    .into_iter()
    .collect();

// Turbofish syntax when type is ambiguous
let v = (0..10).collect::<Vec<i32>>();
```

---

## Performance Tips

### Use Iterators

```rust
// ❌ Slower (allocates intermediate Vec)
let result: Vec<i32> = vec.iter()
    .map(|x| x * 2)
    .collect::<Vec<i32>>()
    .into_iter()
    .filter(|x| *x > 10)
    .collect();

// ✅ Faster (lazy, no intermediate allocation)
let result: Vec<i32> = vec.iter()
    .map(|x| x * 2)
    .filter(|x| *x > 10)
    .collect();
```

### Avoid Unnecessary Clones

```rust
// ❌ Clones on every iteration
for item in &vec {
    process(item.clone());
}

// ✅ Pass by reference
fn process(item: &Item) { }
for item in &vec {
    process(item);
}
```

### Use Cow for Conditional Allocation

```rust
use std::borrow::Cow;

fn transform(s: &str) -> Cow<str> {
    if s.contains("hello") {
        Cow::Owned(s.replace("hello", "hi"))  // Allocates
    } else {
        Cow::Borrowed(s)  // No allocation!
    }
}
```

### Preallocate Collections

```rust
// Like make([]int, 0, 100) in Go
let mut vec = Vec::with_capacity(100);

// HashMap with capacity
let mut map = HashMap::with_capacity(50);
```

### Use Small Types

```rust
// ❌ 24 bytes
struct Large {
    x: String,   // 24 bytes
    y: i32,      // 4 bytes
}

// ✅ 8 bytes when possible
struct Small {
    x: &'static str,  // 16 bytes (if static)
    y: i32,           // 4 bytes
}
```

---

## Testing Tricks

### Test Organization

```rust
// Unit tests in same file (like Go's _test.go)
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
        panic!("oh no");
    }

    #[test]
    #[ignore]  // Skip this test unless --ignored
    fn expensive_test() {
        // ...
    }
}
```

### Test Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored

# Run tests in parallel (default)
cargo test

# Run tests sequentially
cargo test -- --test-threads=1
```

### Assertions

```rust
assert_eq!(a, b);           // Like if a != b { panic!(...) }
assert_ne!(a, b);           // Not equal
assert!(condition);         // Boolean check

// With custom messages
assert_eq!(a, b, "a should equal b, got {} and {}", a, b);

// Debug assertions (only in debug builds)
debug_assert!(expensive_check());
```

### Property Testing

```rust
// Install proptest
// cargo add --dev proptest

use proptest::prelude::*;

proptest! {
    #[test]
    fn reverse_reverse_is_identity(v: Vec<i32>) {
        let mut v2 = v.clone();
        v2.reverse();
        v2.reverse();
        assert_eq!(v, v2);
    }
}
```

---

## Documentation

### Writing Docs

```rust
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// This function panics if the result would overflow.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Generating Docs

```bash
# Generate and open docs for your crate
cargo doc --open

# Include private items
cargo doc --document-private-items --open

# Search in docs with "S" key when viewing
```

### Doc Tests

```rust
/// Example in docs that's also a test!
///
/// ```
/// assert_eq!(2 + 2, 4);
/// ```
pub fn example() {}

// These run with cargo test!
```

---

## Learning Resources

### Essential Crates (Like Go Packages)

**Error Handling**:
- `anyhow` - Easy error handling (use for applications)
- `thiserror` - Custom error types (use for libraries)

**Serialization** (like encoding/json):
- `serde` + `serde_json` - JSON
- `serde_yaml` - YAML
- `bincode` - Binary

**HTTP** (like net/http):
- `reqwest` - HTTP client (async)
- `axum` or `actix-web` - HTTP server

**Async Runtime** (like goroutines):
- `tokio` - Most popular async runtime
- `async-std` - Alternative async runtime

**CLI** (like cobra):
- `clap` - Command-line argument parsing

**Logging** (like log package):
- `env_logger` - Simple logger
- `tracing` - Structured logging

### Quick References

- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) - Common patterns
- [Cheat.rs](https://cheats.rs/) - Language cheat sheet
- [Crates.io](https://crates.io/) - Package registry

### When You're Stuck

1. **Read the compiler error** - It usually tells you the fix
2. **cargo clippy** - Suggests better approaches
3. **Search error code** - `E0382` → Google "rust E0382"
4. **r/rust on Reddit** - Friendly community
5. **Rust Discord** - Real-time help
6. **This Old Crate** podcast - Learn from others' code

---

## Workflow Tips

### Rapid Prototyping

```rust
// Use unwrap() while prototyping
let config = load_config().unwrap();

// Add proper error handling later
let config = load_config()?;
```

### Handling "I Don't Know the Type"

```rust
// Let the compiler tell you
let x = some_function();
let () = x;  // Compiler error shows the type!
```

### Refactoring Workflow

1. Change function signature
2. `cargo check` shows all call sites that break
3. Fix each one
4. Compiler guarantees you didn't miss anything!

### Speed Up Compile Times

```bash
# Use mold linker (macOS/Linux)
brew install mold  # or your package manager

# Add to .cargo/config.toml:
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

# Shared compilation cache (like Go's build cache)
cargo install sccache
export RUSTC_WRAPPER=sccache
```

---

## Pro Tips

1. **Start with cargo check, not cargo build** - 10x faster feedback

2. **Use clippy religiously** - It teaches you idiomatic Rust

3. **Read the module docs** - `std::vec`, `std::option`, etc.

4. **Use turbofish when stuck** - `.collect::<Vec<_>>()` helps type inference

5. **Clone first, optimize later** - Get it working, then remove clones

6. **Use dbg!() instead of println!** - Shows expression + value + location

7. **Learn iterator combinators** - They're zero-cost and more readable

8. **Use anyhow for applications** - Makes error handling painless

9. **Trust the borrow checker** - It's preventing real bugs

10. **When frustrated, take a break** - The solution is usually simpler than you think

---

## Transition Strategy

### Week 1: Get comfortable
- Use `.clone()` liberally
- Focus on learning syntax
- Don't worry about optimization

### Week 2-3: Understand ownership
- Reduce clones
- Use references properly
- Learn when to move vs borrow

### Week 4+: Write idiomatic Rust
- Use iterator chains
- Leverage the type system
- Embrace zero-cost abstractions

### Remember

You learned Go in days/weeks. Rust takes months. That's okay—it's a different kind of language. The payoff is worth it:

- No GC pauses
- Fearless refactoring
- Impossible to have data races
- Performance of C++ with safety of Go

Keep at it! Every Rust developer was frustrated by the borrow checker at first. You've got this! 🦀
