# Concept: Debugging

## Overview

Debugging is the process of finding and fixing errors in code. Unlike languages that rely on print statements or external debuggers, Rust provides excellent debugging capabilities through compiler messages, logging, and integration with standard debuggers like GDB and LLDB. This concept covers practical debugging techniques, reading compiler errors, and using debugging tools effectively.

## Learning Objectives

By the end of this concept, you will understand:
- Reading and understanding compiler error messages
- Using println! debugging effectively
- Structured logging with libraries
- Using GDB/LLDB debuggers
- Backtrace interpretation
- Common error patterns
- Debugging strategies
- Performance debugging with profiling

## Theory

### Compiler Error Messages

Rust's compiler provides detailed, helpful error messages:

```rust
// This code has an error
fn main() {
    let x = 5;
    println!("{}", y);  // y doesn't exist
}
```

**Error output:**

```
error[E0425]: cannot find value `y` in this scope
 --> src/main.rs:3:20
  |
3 |     println!("{}", y);
  |                    ^ not found in this scope
```

**Reading the message:**
1. **Error code** (E0425) - Identifies the error type
2. **Location** (src/main.rs:3:20) - Line and column
3. **Error text** - Description of the problem
4. **Suggestions** - How to fix (often included)

**Common error codes:**

| Code | Meaning |
|------|---------|
| E0425 | Cannot find value in scope |
| E0308 | Type mismatch |
| E0382 | Value used after move |
| E0506 | Cannot mutate while borrowed |
| E0507 | Cannot move out of borrowed context |

### Using println! for Debugging

Simple but effective debugging:

```rust
fn calculate(x: i32, y: i32) -> i32 {
    println!("calculate called with x={}, y={}", x, y);
    let result = x + y;
    println!("result = {}", result);
    result
}

fn main() {
    let a = 5;
    println!("a = {}", a);
    let b = calculate(a, 3);
    println!("b = {}", b);
}
```

**Output:**

```
a = 5
calculate called with x=5, y=3
result = 8
b = 8
```

**Debug formatting:**

```rust
let x = vec![1, 2, 3];
println!("{:?}", x);      // Concise debug format
println!("{:#?}", x);     // Pretty-printed format
```

### dbg! Macro

Rust provides `dbg!` macro for convenient debugging:

```rust
let x = 5;
dbg!(x);  // Prints: [src/main.rs:2] x = 5

let y = dbg!(x * 2);  // Prints value and returns it

// Complex debugging
let data = vec![1, 2, 3];
dbg!(&data);
```

### Structured Logging

Use the `log` crate for production-ready debugging:

```rust
use log::{debug, info, warn, error};

fn process_data(data: &str) {
    debug!("Processing: {}", data);

    if data.is_empty() {
        warn!("Received empty data");
        return;
    }

    info!("Data processed successfully");
}
```

### RUST_BACKTRACE Environment Variable

When a program panics, you can get a stack trace:

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo run

# Full backtrace (verbose)
RUST_BACKTRACE=full cargo run
```

**Example panic output:**

```
thread 'main' panicked at 'index out of bounds', src/main.rs:5:5
stack backtrace:
   0: rust_begin_unwind
      at /rustc/.../src/libcore/panicking.rs:58
   1: core::panicking::panic_fmt
      at /rustc/.../src/libcore/panicking.rs:67
   2: core::panicking::panic_bounds_check
   3: my_app::main
      at src/main.rs:5
```

### Using GDB Debugger

Debug Rust programs with GDB:

```bash
# Build with debug symbols
cargo build

# Start debugger
gdb ./target/debug/my_app

# Common GDB commands
(gdb) break main           # Set breakpoint
(gdb) run                  # Run program
(gdb) print x              # Print variable
(gdb) step                 # Step into function
(gdb) next                 # Step over function
(gdb) continue             # Continue execution
(gdb) backtrace            # Show stack trace
(gdb) quit                 # Exit
```

### LLDB Debugger (macOS)

Similar to GDB but for LLDB:

```bash
# Build with debug symbols
cargo build

# Start debugger
lldb ./target/debug/my_app

# Common LLDB commands
(lldb) breakpoint set --name main
(lldb) run
(lldb) frame variable x
(lldb) step-into
(lldb) step-over
(lldb) continue
(lldb) bt
(lldb) exit
```

### Understanding Ownership Errors

Most errors come from Rust's ownership system:

```rust
// ❌ Error: value moved
let s = String::from("hello");
let s2 = s;
println!("{}", s);  // Error: s was moved

// ✅ Fix: borrow instead
let s = String::from("hello");
let s2 = &s;
println!("{}", s);  // OK: s still usable
```

### Testing Edge Cases

Debug by testing edge cases:

```rust
fn divide(a: i32, b: i32) -> i32 {
    a / b
}

#[test]
fn test_divide() {
    assert_eq!(divide(10, 2), 5);    // Normal
    assert_eq!(divide(0, 5), 0);     // Zero dividend
    // What about divide(10, 0)? This panics!
}
```

### Logging Levels

Different levels of detail:

```rust
use log::{debug, info, warn, error, trace};

trace!("Very detailed");  // Lowest level
debug!("Debug info");
info!("Informational");
warn!("Warning");
error!("Error!");  // Highest level
```

## Syntax

### println! Debugging

```rust
println!("{}", value);           // Single value
println!("{:?}", value);         // Debug format
println!("{:#?}", value);        // Pretty debug
println!("x={}, y={}", x, y);   // Multiple values
```

### dbg! Macro

```rust
dbg!(x);               // Print and return value
dbg!(&collection);     // Debug reference
let y = dbg!(x * 2);   // Use in expressions
```

### Backtraces

```bash
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full cargo run
```

### Debugger Commands

```bash
# GDB
gdb program
(gdb) break function_name
(gdb) run
(gdb) print variable
(gdb) step
(gdb) continue
(gdb) quit

# LLDB
lldb program
(lldb) breakpoint set --name function_name
(lldb) run
(lldb) frame variable variable_name
(lldb) step-into
(lldb) continue
(lldb) quit
```

## Common Patterns

### Pattern 1: Targeted println debugging

```rust
fn process(data: &[i32]) -> i32 {
    println!("Input: {:?}", data);

    let sum: i32 = data.iter().sum();
    println!("Sum: {}", sum);

    let average = sum / data.len() as i32;
    println!("Average: {}", average);

    average
}
```

### Pattern 2: Conditional debugging

```rust
const DEBUG: bool = true;

fn process(x: i32) -> i32 {
    if DEBUG {
        println!("Processing: {}", x);
    }
    x * 2
}
```

### Pattern 3: Using dbg! in expressions

```rust
let result = some_function(dbg!(value));
```

### Pattern 4: Backtrace when panicking

```bash
RUST_BACKTRACE=1 cargo run
```

### Pattern 5: Logging at different levels

```rust
use log::{debug, info, warn, error};

fn operation() {
    debug!("Starting operation");
    info!("Operation in progress");
    warn!("Unusual condition");
    error!("Operation failed");
}
```

### Pattern 6: Assertion messages

```rust
assert!(
    value > 0,
    "Expected positive value, got {}",
    value
);
```

### Pattern 7: Result debugging

```rust
let result = risky_operation()
    .expect("Operation failed with details here");

// Or
let result = match risky_operation() {
    Ok(val) => val,
    Err(e) => {
        eprintln!("Error: {}", e);
        return;
    }
};
```

### Pattern 8: Temporary debug prints

```rust
// Temporary - remove after debugging
eprintln!("DEBUG: state = {:?}", state);
```

## Common Mistakes

### Mistake 1: Not reading compiler messages

```rust
// ❌ Ignoring helpful error
error[E0425]: cannot find value in this scope
// Compiler tells you exactly what's wrong!

// ✅ Read and understand
// The error message explains the problem and suggests fixes
```

### Mistake 2: Println debugging noise

```rust
// ❌ Too much output
println!("1");
println!("2");
println!("3");
// ... dozens more

// ✅ Targeted debugging
println!("DEBUG: x = {}", x);  // Clear purpose
```

### Mistake 3: Forgetting to use debug symbols

```bash
# ❌ Debug symbols not included
cargo build --release
gdb ./target/release/app  # Poor debugging

# ✅ Build with debug symbols
cargo build
gdb ./target/debug/app  # Good debugging
```

### Mistake 4: Not using structured logging

```rust
// ❌ Unstructured
println!("Error occurred");

// ✅ Structured
error!("Error occurred: {:?}", err);
```

### Mistake 5: Leaving debug prints in production code

```rust
// ❌ Debug code left in
fn important_function() {
    println!("DEBUG: processing");  // Remove!
}

// ✅ Use logging framework
fn important_function() {
    debug!("Processing");  // Can be disabled in release
}
```

## Real-World Examples

### Example 1: Finding an ownership error

```rust
// ❌ Error: value moved
fn main() {
    let name = String::from("Alice");
    println!("{}", name);  // OK
    let copy = name;       // name moved here
    println!("{}", name);  // ERROR: use after move
}

// Diagnosis:
// error[E0382]: borrow of moved value: `name`
//
// ✅ Fix: borrow instead
fn main() {
    let name = String::from("Alice");
    println!("{}", name);
    let copy = &name;  // Borrow, not move
    println!("{}", name);  // OK
}
```

### Example 2: Type mismatch debugging

```rust
// ❌ Type error
let numbers: Vec<i32> = vec!["1", "2", "3"];  // String literals

// Error message:
// error[E0308]: mismatched types
//   expected `i32`
//      found `&str`

// ✅ Fix: correct types
let numbers: Vec<i32> = vec![1, 2, 3];
// Or
let strings: Vec<&str> = vec!["1", "2", "3"];
```

### Example 3: Using backtrace

```rust
fn recursive(n: i32) {
    println!("n = {}", n);
    if n > 0 {
        recursive(n - 1);
    } else {
        panic!("Hit bottom");  // Intentional for testing
    }
}

// Run with: RUST_BACKTRACE=1 cargo run
// Shows call stack leading to panic
```

### Example 4: Logging in library code

```rust
use log::{debug, info, error};

pub fn process(data: &[u8]) -> Result<String, String> {
    debug!("Processing {} bytes", data.len());

    if data.is_empty() {
        error!("Received empty data");
        return Err("Empty data".to_string());
    }

    info!("Processing successful");
    Ok(String::from_utf8_lossy(data).to_string())
}
```

## Related Concepts

### Prerequisites
- **Error Handling** - Understanding errors
- **Rust Basics** - Core language concepts
- **Testing** - Related quality aspect

### What comes next
- **Performance** - Profiling for debugging performance
- **Logging Frameworks** - Advanced debugging
- **CI/CD** - Automated debugging in pipeline

### Cross-references
- Module 02: Error handling patterns
- Module 03: Testing and quality tools
- Development workflow

## Best Practices

### Use compiler messages first

The compiler is your first debugging tool:

```rust
// Trust the compiler's error messages
// They explain what's wrong and suggest fixes
```

### Targeted debugging

```rust
// ✅ Focus on specific values
println!("x = {}", x);

// ❌ Avoid debug output overload
println!("1"); println!("2"); // Too much noise
```

### Use appropriate tools

| Situation | Tool |
|-----------|------|
| Quick check | println! |
| Production code | log crate |
| Complex debugging | GDB/LLDB |
| Performance | perf/flamegraph |

### Backtrace for panics

```bash
RUST_BACKTRACE=1 cargo run
```

Shows exactly where panic occurred.

## Summary

- **Compiler messages** are your primary debugging tool
- **println!** debugging is simple and effective
- **dbg!** macro is convenient for expressions
- **Logging frameworks** for production code
- **Backtraces** show call stacks on panic
- **Debuggers** (GDB/LLDB) for deep inspection
- **Testing** prevents bugs before debugging
- **Error types** encode information

## Key Takeaways

1. Read compiler error messages carefully
2. Compiler suggests many fixes automatically
3. Use println! for quick debugging
4. Use dbg! macro for expressions
5. Use logging frameworks in production
6. Use RUST_BACKTRACE for panic traces
7. GDB/LLDB for complex debugging
8. Prevention (testing) better than debugging

## Practice Exercise Ideas

1. Debug an ownership error using compiler message
2. Add strategic println! debugging
3. Use dbg! macro in expression
4. Run program with RUST_BACKTRACE
5. Set breakpoint in GDB/LLDB
6. Add logging to code
7. Find and fix a type mismatch
8. Debug a panic using backtrace

---

**Time to complete this concept**: 2-2.5 hours
**Difficulty**: Intermediate
**Prerequisite**: Error Handling, Testing
**Next concept**: End of Module 03

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
