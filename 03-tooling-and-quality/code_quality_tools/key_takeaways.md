# Key Takeaways: Code Quality Tools

## Quick Reference

### Main Commands

```bash
cargo fmt              # Format code
cargo clippy           # Lint code
cargo check            # Fast compile check
cargo test             # Run tests
cargo build --release  # Optimized build
```

### Configuration

```toml
# rustfmt.toml
max_width = 100
tab_spaces = 4

# clippy.toml
cognitive-complexity-threshold = 30
```

### Suppress Warnings

```rust
#[allow(clippy::lint_name)]
fn function() { }
```

## Essential Concepts

### 1. Tool Purposes

| Tool | Purpose | Speed |
|------|---------|-------|
| `cargo check` | Type/syntax check | Very fast |
| `cargo fmt` | Format code | Very fast |
| `cargo clippy` | Lint & suggestions | Fast |
| `cargo test` | Run tests | Medium |
| `cargo build` | Compile | Slow |

### 2. Development Workflow

```bash
# 1. Code and check frequently
cargo check          # Fast, during development

# 2. Before committing
cargo fmt            # Format consistently
cargo clippy         # Check for issues
cargo test           # Verify behavior

# 3. Before release
cargo build --release  # Optimized binary
```

### 3. rustfmt Formatting

```bash
cargo fmt           # Format all
cargo fmt -- --check  # Check without modifying
cargo fmt -- src/main.rs  # Format specific file
```

**Common options:**

```toml
max_width = 100          # Line length
tab_spaces = 4           # Indentation
trailing_comma = "Vertical"  # Comma style
format_strings = true    # Format string literals
```

### 4. Clippy Linting

```bash
cargo clippy                    # Show warnings
cargo clippy -- -D warnings     # Treat as errors
cargo clippy -- -W clippy::all  # All warnings
```

**Common lints:**

- `clippy::clone_on_copy` - Unnecessary clone
- `clippy::eq_op` - Comparing value to itself
- `clippy::manual_map` - Use map() instead
- `clippy::needless_return` - Unnecessary return

### 5. cargo check vs build

```bash
cargo check   # Type check only (very fast)
cargo build   # Full compilation (slower)
cargo build --release  # Optimized (slowest)
```

**Use check for:** Fast feedback during coding
**Use build for:** Creating binaries

### 6. Suppressing Warnings

```rust
// Specific lint
#[allow(clippy::needless_return)]
fn function() {
    return 5;  // Intentional for clarity
}

// Multiple lints
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
fn cast_value(x: u64) -> u32 {
    x as u32
}

// File-wide
#![forbid(unsafe_code)]
#![deny(warnings)]
```

### 7. Build Profiles

```toml
[profile.dev]
opt-level = 0      # Debug: no optimization

[profile.release]
opt-level = 3      # Release: full optimization
lto = true         # Link-time optimization
codegen-units = 1  # Single compilation unit
```

### 8. Continuous Quality

**Pre-commit checklist:**
1. `cargo fmt` - Format code
2. `cargo clippy` - Check for issues
3. `cargo test` - Run tests
4. `cargo build --release` - Final build

## Common Patterns

### Pattern 1: Quality workflow
```bash
cargo check
cargo clippy
cargo test
cargo build --release
```

### Pattern 2: Makefile
```makefile
quality: fmt lint test build
	@echo "Done!"

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

test:
	cargo test

build:
	cargo build --release
```

### Pattern 3: Suppress with reason
```rust
#[allow(clippy::cast_possible_truncation)]
// This truncation is intentional for memory efficiency
fn pack_data(x: u64) -> u32 { }
```

### Pattern 4: Profile override
```toml
[profile.dev.package."*"]
opt-level = 3  # Optimize dependencies in debug
```

### Pattern 5: CI configuration
```yaml
- run: cargo fmt -- --check
- run: cargo clippy -- -D warnings
- run: cargo test
- run: cargo build --release
```

### Pattern 6: Multiple profiles
```bash
cargo build --release  # Release binary
cargo build --profile=custom  # Custom profile
cargo build  # Debug binary
```

### Pattern 7: Format check only
```bash
cargo fmt -- --check  # Exit with error if not formatted
```

### Pattern 8: Ignore specific files
```toml
# rustfmt.toml
exclude = ["target/", "build/"]
```

## Checklist: Quality Standards

- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings
- [ ] All tests passing
- [ ] Documentation present
- [ ] No `unsafe` code (or justified)
- [ ] Release build compiles
- [ ] Performance tested

## Error Prevention

### ❌ DON'T: Skip formatting
```bash
git add unformatted_code.rs
git commit
```

### ✅ DO: Format before committing
```bash
cargo fmt
git add formatted_code.rs
git commit
```

### ❌ DON'T: Ignore clippy without reason
```rust
#[allow(clippy::all)]
fn function() { }
```

### ✅ DO: Suppress specific lint with reason
```rust
#[allow(clippy::needless_return)]
// Explicit return makes control flow clear
fn function() { }
```

### ❌ DON'T: Use debug build for performance testing
```bash
cargo build
./target/debug/app  # Slow!
```

### ✅ DO: Use release build
```bash
cargo build --release
./target/release/app  # Fast!
```

### ❌ DON'T: Only check quality when releasing
```bash
# Many commits later...
cargo clippy  # Lots of issues
```

### ✅ DO: Check quality during development
```bash
# After each coding session
cargo fmt
cargo clippy
cargo test
```

## Tool Commands

| Command | Purpose |
|---------|---------|
| `cargo fmt` | Format code |
| `cargo fmt -- --check` | Check without modifying |
| `cargo clippy` | Show warnings |
| `cargo clippy -- -D warnings` | Warnings as errors |
| `cargo check` | Quick type check |
| `cargo build` | Build debug binary |
| `cargo build --release` | Build optimized binary |
| `cargo test` | Run tests |
| `cargo doc` | Generate documentation |

## Configuration Files

**rustfmt.toml:**
```toml
max_width = 100
tab_spaces = 4
trailing_comma = "Vertical"
format_strings = true
```

**clippy.toml:**
```toml
cognitive-complexity-threshold = 30
too-many-arguments-threshold = 10
```

**.cargo/config.toml:**
```toml
[build]
jobs = 4

[target.x86_64-unknown-linux-gnu]
linker = "clang"
```

## Common Clippy Warnings

| Warning | Issue | Fix |
|---------|-------|-----|
| `clone_on_copy` | Clone when Copy available | Just use value |
| `eq_op` | Comparing to itself | Check logic |
| `manual_map` | Use map() instead | `opt.map(\|x\| f(x))` |
| `needless_return` | Explicit return | Remove return |
| `cast_possible_truncation` | Loss of data | Handle explicitly |

## Performance Tips

### Debug vs Release

```bash
# Debug: quick compile, slow runtime
cargo build
time ./target/debug/app

# Release: slow compile, fast runtime
cargo build --release
time ./target/release/app
```

Release is typically 10-100x faster!

## Related Concepts

- **Testing** - Quality verification
- **Documentation** - Code quality aspect
- **Cargo** - Tool runner
- **CI/CD** - Automated quality

## Time Estimates

- Reading this takeaway: 10-15 minutes
- Setting up tools: 10 minutes
- Practice: 20-30 minutes
- Total: 40-55 minutes

## Practice Questions

1. What's the difference between check and build?
2. How do you run clippy with warnings as errors?
3. What does rustfmt.toml configure?
4. Why suppress a specific lint instead of all?
5. When should you use --release build?
6. How do you format code without modifying?
7. What's the typical pre-commit workflow?
8. How do you suppress clippy for a file?

## Quick Workflow

```bash
# During development
cargo check             # Frequent checking

# Before commit
cargo fmt              # Format
cargo clippy           # Lint
cargo test             # Test

# Before release
cargo build --release  # Optimized
cargo test --release   # Test optimized
```

## Makefile Template

```makefile
.PHONY: check fmt lint test build clean quality help

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

clean:
	cargo clean

quality: fmt lint test build
	@echo "✓ All quality checks passed!"

help:
	@echo "Available targets: check, fmt, lint, test, build, clean, quality"
```

---

**Status**: Quick reference guide
**Importance**: ⭐⭐⭐⭐ (Important)
**Difficulty**: Beginner
**Part of**: Module 03 - Tooling and Quality
