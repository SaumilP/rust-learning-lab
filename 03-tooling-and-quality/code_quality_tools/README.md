# Concept: Code Quality Tools

## Overview

Code quality tools help maintain consistent, clean, and idiomatic code. Rust has exceptional tooling built into its ecosystem. Tools like `rustfmt` for formatting, `clippy` for linting, and `cargo check` for quick compilation all work together to ensure code quality. This concept covers the most important tools and how to use them effectively.

## Learning Objectives

By the end of this concept, you will understand:
- `rustfmt` for automatic code formatting
- `clippy` for linting and code suggestions
- `cargo check` for fast compilation
- `cargo build` vs `cargo check`
- Configuration files (rustfmt.toml, clippy.toml)
- Addressing common clippy warnings
- Integrating tools into workflow
- Continuous quality practices

## Theory

### rustfmt - Code Formatting

`rustfmt` automatically formats Rust code to a consistent standard:

```bash
# Format current crate
cargo fmt

# Format specific file
cargo fmt -- src/main.rs

# Check without modifying
cargo fmt -- --check
```

**Before:**

```rust
fn main(){let x=5;let y=3;println!("{}",x+y);}
```

**After:**

```rust
fn main() {
    let x = 5;
    let y = 3;
    println!("{}", x + y);
}
```

**Configuration - rustfmt.toml:**

```toml
# Line length
max_width = 100

# Indentation
tab_spaces = 4

# Trailing comma
trailing_comma = "Vertical"

# Format strings
format_strings = true
```

### clippy - Linting

Clippy provides warnings about non-idiomatic or potentially problematic code:

```bash
# Run clippy
cargo clippy

# With warnings as errors
cargo clippy -- -D warnings

# Specific lint level
cargo clippy -- -W clippy::all
```

**Common Clippy Warnings:**

```rust
// ❌ clippy: matches on bool, use if instead
match some_bool {
    true => { println!("yes"); }
    false => { println!("no"); }
}

// ✅ Better: use if/else
if some_bool {
    println!("yes");
} else {
    println!("no");
}
```

**Ignoring Clippy Warnings:**

```rust
#[allow(clippy::needless_return)]
fn function() -> i32 {
    return 5;  // Clippy would warn about this
}

// For entire file
#![allow(clippy::all)]
```

### Common Clippy Lints

**Unnecessarily complex patterns:**

```rust
// ❌ clippy::eq_op - comparing value to itself
if x == x { }

// ✅ Better: check what you meant
if x == y { }
```

**Inefficient patterns:**

```rust
// ❌ clippy::clone_on_copy - unnecessary clone
let x = value.clone();  // value implements Copy

// ✅ Better: just copy
let x = value;
```

**Style issues:**

```rust
// ❌ clippy::manual_map - could use map()
match opt {
    Some(x) => Some(x * 2),
    None => None,
}

// ✅ Better: use iterator adapter
opt.map(|x| x * 2)
```

**Panic scenarios:**

```rust
// ❌ clippy::expect_used - might panic
let val = config.get("key").expect("Key should exist");

// ✅ Better: handle the error
let val = config.get("key").unwrap_or_default();
```

### cargo check - Fast Compilation

`cargo check` compiles code without generating binaries (much faster):

```bash
# Quick syntax/type check
cargo check

# Check tests
cargo check --tests

# Check specific target
cargo check --bin my_app
```

**Typical workflow:**

```bash
cargo check    # Quick feedback while coding
cargo clippy   # Get lint suggestions
cargo test     # Run tests
cargo build    # Build release binary
```

### cargo build

Compiles the project into an executable:

```bash
# Debug build (slower, includes debug info)
cargo build

# Release build (optimized, faster runtime)
cargo build --release

# Specific target
cargo build --bin my_app
```

### Performance Profiling

Finding and fixing performance issues:

```bash
# Generate binary with debug info
cargo build

# Run with profiling
perf record ./target/debug/my_app

# View results
perf report
```

### Tool Configuration

**rustfmt.toml or .rustfmt.toml:**

```toml
[default]
max_width = 100
indent_style = "Tab"
tab_spaces = 4
format_code_in_doc_comments = true
```

**clippy.toml:**

```toml
# Configure specific lints
cognitive-complexity-threshold = 30
too-many-arguments-threshold = 10
```

**.gitignore for build artifacts:**

```
/target/
Cargo.lock
.DS_Store
```

### Continuous Quality

**Pre-commit checklist:**

```bash
# 1. Format code
cargo fmt

# 2. Run linter
cargo clippy -- -D warnings

# 3. Run tests
cargo test

# 4. Build release
cargo build --release
```

**Create a Makefile:**

```makefile
.PHONY: check fmt lint test build

check:
	cargo check

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

test:
	cargo test

build:
	cargo build --release

all: fmt lint test build
```

## Syntax

### Running Tools

```bash
# Formatting
cargo fmt
cargo fmt -- --check

# Linting
cargo clippy
cargo clippy -- -D warnings

# Type checking
cargo check

# Building
cargo build
cargo build --release

# Testing
cargo test
cargo test --doc
```

### Configuration Files

```toml
# rustfmt.toml
max_width = 100
tab_spaces = 4

# clippy.toml
cognitive-complexity-threshold = 30
```

### Attribute Directives

```rust
#[allow(clippy::lint_name)]
#[warn(clippy::lint_name)]
#[forbid(unsafe_code)]
```

## Common Patterns

### Pattern 1: Standard project setup

```bash
# Create project
cargo new my_project
cd my_project

# Configure formatting
echo "max_width = 100" > rustfmt.toml

# Check quality
cargo fmt
cargo clippy
cargo test
```

### Pattern 2: Pre-commit workflow

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release
```

### Pattern 3: Continuous integration

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
      - run: cargo fmt -- --check
      - run: cargo clippy -- -D warnings
      - run: cargo test
```

### Pattern 4: Suppressing specific warnings

```rust
#[allow(clippy::needless_return)]
fn function() -> i32 {
    return 5;  // Return is explicit for clarity
}
```

### Pattern 5: File-level configuration

```rust
#![forbid(unsafe_code)]
#![deny(warnings)]

// File content
```

### Pattern 6: Workspace-wide settings

**Cargo.toml:**

```toml
[workspace]

[profile.release]
opt-level = 3
lto = true

[lint.clippy]
all = { level = "warn", priority = 100 }
```

## Common Mistakes

### Mistake 1: Ignoring format errors

```bash
# ❌ Don't: leave code unformatted
git add unformatted_code.rs
git commit

# ✅ Do: format before committing
cargo fmt
git add formatted_code.rs
git commit
```

### Mistake 2: Ignoring clippy warnings

```rust
// ❌ Don't: suppress without reason
#[allow(clippy::all)]
fn function() { }

// ✅ Do: allow specific lint with reason
#[allow(clippy::needless_return)]  // Early return adds clarity
fn function() {
    return result;
}
```

### Mistake 3: Not running tests before build

```bash
# ❌ Wrong order
cargo build --release
cargo test

# ✅ Right order
cargo test
cargo build --release
```

### Mistake 4: Using debug builds in production

```bash
# ❌ Debug: slow, large
cargo build
./target/debug/app

# ✅ Release: optimized, small
cargo build --release
./target/release/app
```

### Mistake 5: Not checking during development

```bash
# ❌ Only check when committing
git add .
cargo check

# ✅ Check continuously while developing
cargo check  # After each edit
cargo clippy # Before commit
```

## Real-World Examples

### Example 1: Linting a function

```rust
// ❌ Code with clippy issues
fn process_data(data: &Vec<i32>) -> Option<i32> {  // &Vec<i32> should be &[i32]
    if data.len() == 0 {  // use is_empty()
        return None;
    }
    for item in data {
        println!("{}", item);
    }
    return Some(data.len() as i32);  // unnecessary return
}

// ✅ After clippy fixes
fn process_data(data: &[i32]) -> Option<i32> {
    if data.is_empty() {
        return None;
    }
    for item in data {
        println!("{}", item);
    }
    Some(data.len() as i32)
}
```

### Example 2: Formatting before commit

```bash
$ cargo fmt
$ cargo clippy
warning: this expression can be simplified
 --> src/main.rs:10:13
  |
10 |     if x == true {
   |        ^^^^^^^^^^
   |
   = help: consider using the variable directly

$ cargo test
   Compiling project v0.1.0
    Finished test [unoptimized + debuginfo]
     Running unittests

test result: ok. 8 passed

$ cargo build --release
    Finished release [optimized]
```

### Example 3: CI configuration

```yaml
name: CI
on: [push, pull_request]
jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy

      - name: Check format
        run: cargo fmt -- --check

      - name: Lint
        run: cargo clippy -- -D warnings

      - name: Test
        run: cargo test

      - name: Build release
        run: cargo build --release
```

### Example 4: Makefile for workflow

```makefile
.PHONY: fmt lint test build check clean

fmt:
	@cargo fmt

lint:
	@cargo clippy -- -D warnings

test:
	@cargo test

check:
	@cargo check

build:
	@cargo build --release

clean:
	@cargo clean

# Run all checks
quality: fmt lint test build
	@echo "All quality checks passed!"

help:
	@echo "Available targets:"
	@echo "  make fmt    - Format code"
	@echo "  make lint   - Run clippy"
	@echo "  make test   - Run tests"
	@echo "  make build  - Build release"
	@echo "  make quality - Run all checks"
```

## Related Concepts

### Prerequisites
- **Cargo** - Rust's package manager
- **Source Code** - Code being checked
- **Testing** - Related quality aspect

### What comes next
- **CI/CD** - Automated quality checks
- **Performance** - Profiling and optimization
- **Debugging** - Finding issues

### Cross-references
- Module 02: Code patterns (clippy suggests idiomatic patterns)
- Module 03: Testing and documentation (part of quality)

## Best Practices

### Run tools frequently

```bash
# After each coding session
cargo fmt      # Keep consistent
cargo clippy   # Catch issues
cargo test     # Verify behavior
```

### Make linting part of workflow

```bash
# Quick check while coding
cargo check

# Before commits
cargo clippy
cargo test

# Before releases
cargo build --release
```

### Use meaningful allow attributes

```rust
// ✅ Good: explains why
#[allow(clippy::cast_possible_truncation)]
// This cast is intentional for API compatibility
fn get_size() -> u32 { }

// ❌ Bad: no explanation
#[allow(clippy::all)]
fn get_size() -> u32 { }
```

### Keep tool configurations simple

```toml
# rustfmt.toml - Keep it minimal
max_width = 100
```

Use defaults for everything else.

## Summary

- **rustfmt** - Automatic code formatting
- **clippy** - Linting and suggestions
- **cargo check** - Fast type/syntax checking
- **cargo build** - Compilation (debug/release)
- **cargo test** - Testing code
- **Configuration** - Customize tool behavior
- **Continuous quality** - Integrate tools into workflow
- **Pre-commit** - Run tools before committing

## Key Takeaways

1. `cargo fmt` ensures consistent formatting
2. `cargo clippy` catches non-idiomatic patterns
3. `cargo check` provides quick feedback during development
4. Tools integrate seamlessly with cargo
5. Configuration files customize behavior
6. Make quality checks part of daily workflow
7. CI/CD automates quality enforcement
8. Quality tools are zero-friction in Rust

## Practice Exercise Ideas

1. Run clippy on existing code
2. Address clippy warnings with fixes
3. Set up rustfmt.toml with preferences
4. Create Makefile for quality workflow
5. Set up GitHub Actions for CI
6. Suppress specific warnings appropriately
7. Compare debug vs release performance
8. Create pre-commit hook with tools

---

**Time to complete this concept**: 1.5-2 hours
**Difficulty**: Beginner-Intermediate
**Prerequisite**: Cargo basics, Source Code
**Next concept**: Debugging

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
