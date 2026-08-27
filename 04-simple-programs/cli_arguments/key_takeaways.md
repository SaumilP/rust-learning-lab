# CLI Arguments - Key Takeaways

## Core Concepts

### 1. Accessing Arguments
- **`std::env::args()`** - Returns iterator of command-line arguments
- **Index 0** - Contains program name, not first user argument
- **Index 1+** - User-provided arguments

### 2. Collecting Arguments
```rust
let args: Vec<String> = env::args().collect();
let first_arg = &args[1];  // Skip program name
```

### 3. Safe Access Patterns
```rust
// Check before accessing
if args.len() > 1 {
    let arg = &args[1];
}

// Use iterator
if let Some(arg) = env::args().nth(1) {
    // Process arg
}
```

## Common Patterns

| Pattern | Use Case |
|---------|----------|
| Index access | Known number of arguments |
| Iterator | Unknown number of arguments |
| Flag checking | Optional boolean flags |
| Defaults | Optional values with fallbacks |
| Validation | Parsing and error checking |

## Key Functions

| Function | Purpose |
|----------|---------|
| `env::args()` | Get argument iterator |
| `.collect()` | Convert to Vec |
| `.skip(n)` | Skip first n arguments |
| `.nth(i)` | Get specific argument |
| `.contains()` | Check if flag exists |

## Important Notes

✓ Always check argument count before accessing
✓ Use `eprintln!()` for error messages
✓ Provide usage message on error
✓ Call `std::process::exit(1)` on error
✓ Validate argument values (parse, range, etc.)

## Error Handling

```rust
// Missing required argument
if args.len() < expected {
    eprintln!("Usage: {} {}", args[0], usage);
    std::process::exit(1);
}

// Invalid argument value
if let Ok(value) = args[1].parse::<i32>() {
    // Use value
} else {
    eprintln!("Invalid number: {}", args[1]);
    std::process::exit(1);
}
```

## Running Programs with Arguments

```bash
# Single argument
cargo run -- filename.txt

# Multiple arguments
cargo run -- source.txt destination.txt

# With flags
cargo run -- --verbose --config config.toml

# In release mode
cargo run --release -- arg1 arg2
```

## Quick Reference

```rust
use std::env;

// Basic usage
let args: Vec<String> = env::args().collect();

// Check count
if args.len() < 2 { eprintln!("Missing args"); }

// Access safely
let filename = &args.get(1).unwrap_or(&"default.txt".to_string());

// Parse value
let count: i32 = args[1].parse().expect("Invalid number");

// Check for flag
let verbose = args.contains(&"--verbose".to_string());
```

## Common Mistakes to Avoid

1. ❌ Accessing args[1] without checking length
2. ❌ Forgetting args[0] is program name
3. ❌ Using println! for error messages
4. ❌ Not validating argument values
5. ❌ Unclear error messages

