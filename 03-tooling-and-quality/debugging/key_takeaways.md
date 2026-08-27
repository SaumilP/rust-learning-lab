# Key Takeaways: Debugging

## Quick Reference

### Print Debugging

```rust
println!("{}", value);      // Simple value
println!("{:?}", value);    // Debug format
println!("{:#?}", value);   // Pretty debug
dbg!(value);                // Print and return
```

### Environment Variables

```bash
RUST_BACKTRACE=1 cargo run      # With backtrace
RUST_BACKTRACE=full cargo run   # Full backtrace
RUST_LOG=debug cargo run        # With logging
```

### Debugging Commands

```bash
# GDB
gdb ./target/debug/app
(gdb) break main
(gdb) run
(gdb) print x
(gdb) step
(gdb) continue

# LLDB
lldb ./target/debug/app
(lldb) breakpoint set --name main
(lldb) run
(lldb) frame variable x
(lldb) step-into
(lldb) continue
```

## Essential Concepts

### 1. Compiler Error Messages

```
error[CODE]: Description
 --> file:line:column
  |
  | code here
  | ^^^ details
```

**Key parts:**
- **CODE** - Error identifier (e.g., E0425)
- **Description** - What's wrong
- **Location** - Exact line and column
- **Suggestion** - How to fix (often included)

### 2. Common Error Codes

| Code | Meaning |
|------|---------|
| E0425 | Value not found in scope |
| E0308 | Type mismatch |
| E0382 | Value used after move |
| E0506 | Mutable borrow conflict |
| E0507 | Cannot move out of borrow |

### 3. Debug vs Release

```bash
cargo build         # Debug: slow runtime, good debugging
cargo build --release  # Release: fast runtime, poor debugging
```

**Always debug in debug mode!**

### 4. println! Debugging

```rust
println!("value: {}", x);           // Show value
println!("debug: {:?}", data);      // Debug format
println!("pretty: {:#?}", complex); // Pretty printed
```

**Tips:**
- Use targeted prints
- Include context ("x=")
- Remove after debugging

### 5. dbg! Macro

```rust
dbg!(x);              // Print: [file:line] x = value
dbg!(&collection);    // Works with references
let y = dbg!(x * 2);  // Returns the value
```

**Better than println! because:**
- Shows file and line
- Returns the value
- Explicit "DEBUG" output

### 6. Backtraces on Panic

```bash
RUST_BACKTRACE=1 cargo run
```

Shows stack trace leading to panic:
- Which functions called which
- Line numbers for each frame
- Clear error location

### 7. Logging Levels

```
TRACE - Most detailed
DEBUG - Development info
INFO  - Informational
WARN  - Warnings
ERROR - Errors (most important)
```

Use `log` crate:
```rust
use log::{trace, debug, info, warn, error};

trace!("Very detailed");
debug!("Debug info");
info!("Information");
warn!("Warning");
error!("Error!");
```

### 8. GDB/LLDB Basics

| Command | Purpose |
|---------|---------|
| `break` | Set breakpoint |
| `run` | Run program |
| `print` | Print variable |
| `step` | Step into function |
| `next` | Step over function |
| `continue` | Resume execution |
| `backtrace` | Show stack trace |
| `quit` | Exit debugger |

## Common Patterns

### Pattern 1: Quick debugging
```rust
println!("x = {}", x);
dbg!(result);
```

### Pattern 2: Targeted output
```rust
println!("DEBUG: Processing {}", item);
println!("DEBUG: Result = {:?}", result);
```

### Pattern 3: Conditional debugging
```rust
if DEBUG {
    println!("State: {:?}", state);
}
```

### Pattern 4: Error context
```rust
match operation() {
    Ok(val) => println!("Success: {}", val),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Pattern 5: Assertion messages
```rust
assert_eq!(
    actual, expected,
    "Assertion failed: expected {}, got {}",
    expected, actual
);
```

### Pattern 6: Backtrace on panic
```bash
RUST_BACKTRACE=1 cargo run
```

### Pattern 7: Logging in library
```rust
use log::{debug, warn, error};

fn process(data: &str) -> Result<(), String> {
    debug!("Processing: {}", data);
    if data.is_empty() {
        warn!("Received empty data");
        return Err("Empty".to_string());
    }
    Ok(())
}
```

### Pattern 8: Temporary debug
```rust
// TODO: remove debug prints
eprintln!("DEBUG: {:?}", state);
```

## Checklist: Debugging Approach

- [ ] Read compiler error messages carefully
- [ ] Note the error code (E####)
- [ ] Look at suggested fixes
- [ ] Use println! for quick checks
- [ ] Use dbg! for expressions
- [ ] Run with RUST_BACKTRACE for panics
- [ ] Use GDB/LLDB for complex debugging
- [ ] Check ownership/borrow issues first

## Error Prevention

### ❌ DON'T: Ignore compiler message
```rust
// Just try random fixes instead of reading message
```

### ✅ DO: Read compiler message
```rust
// Message explains the problem and suggests fix
```

### ❌ DON'T: Debug without symbols
```bash
cargo build --release
gdb ./target/release/app  # Poor info
```

### ✅ DO: Debug with symbols
```bash
cargo build
gdb ./target/debug/app  # Good info
```

### ❌ DON'T: Leave debug output
```rust
pub fn function() {
    println!("DEBUG");  // Remove!
}
```

### ✅ DO: Use logging framework
```rust
pub fn function() {
    debug!("Processing");  // Can disable in release
}
```

### ❌ DON'T: Too much output
```rust
println!("1");
println!("2");
println!("3");
// 50 more prints...
```

### ✅ DO: Targeted debugging
```rust
println!("x = {}", x);  // Clear purpose
```

## Print Formatting

| Format | Use | Example |
|--------|-----|---------|
| `{}` | Display | `{}`prints "5" |
| `{:?}` | Debug | `{:?}`prints some_value |
| `{:#?}` | Pretty debug | Indented output |
| `{:x}` | Hex | `{:x}`prints "1a" |
| `{:b}` | Binary | `{:b}`prints "1010" |

## Debugging Tools

| Tool | Purpose | Use |
|------|---------|-----|
| Compiler | First level | Always |
| println! | Quick checks | Development |
| dbg! | Expression debug | Development |
| BACKTRACE | Stack traces | Panics |
| GDB/LLDB | Deep debugging | Complex issues |
| perf | Performance | Optimization |
| log crate | Production | Libraries |

## Related Concepts

- **Error Handling** - Understanding errors
- **Testing** - Prevention before debugging
- **Compiler** - First debugging tool
- **Tools** - cargo, GDB, LLDB

## Time Estimates

- Reading this takeaway: 10-15 minutes
- Reviewing patterns: 10 minutes
- Practice drills: 20-30 minutes
- Total: 40-55 minutes

## Practice Questions

1. What are the parts of a compiler error message?
2. What's the difference between println! and dbg!?
3. How do you get a backtrace on panic?
4. What are the six logging levels?
5. How do you set a breakpoint in GDB?
6. When should you use debug vs release build?
7. How do you pretty-print with println!?
8. When is structured logging better than println!?

## Quick Commands

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo run

# Full backtrace
RUST_BACKTRACE=full cargo run

# Run with logging
RUST_LOG=debug cargo run

# Build for debugging
cargo build

# Start GDB
gdb ./target/debug/app

# Start LLDB
lldb ./target/debug/app
```

## Compiler Error Solving Steps

1. Read the error message carefully
2. Note the error code (E####)
3. Identify the location (file:line)
4. Read the suggestion
5. Understand why it's wrong
6. Apply the fix
7. Compile again

## GDB Cheat Sheet

```bash
gdb ./program
(gdb) break function_name
(gdb) run [args]
(gdb) print variable
(gdb) step           # Into function
(gdb) next           # Over function
(gdb) continue       # Resume
(gdb) backtrace      # Stack trace
(gdb) quit           # Exit
```

## LLDB Cheat Sheet

```bash
lldb ./program
(lldb) breakpoint set --name function_name
(lldb) run [args]
(lldb) frame variable variable_name
(lldb) step-into
(lldb) step-over
(lldb) continue
(lldb) bt            # Backtrace
(lldb) quit          # Exit
```

## Debugging Best Practices

1. **Compiler first** - Trust compiler messages
2. **Targeted prints** - Focus on key values
3. **Backtraces** - Use when panicking
4. **Structured logging** - For production code
5. **Prevention** - Testing prevents bugs
6. **Debugger** - For complex issues
7. **Code review** - Catch bugs early
8. **Isolation** - Test small parts

---

**Status**: Quick reference guide
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Intermediate
**Part of**: Module 03 - Tooling and Quality
