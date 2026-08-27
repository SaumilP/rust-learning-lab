# CLI Arguments

## Overview

Command-line arguments allow programs to receive input from the user who runs them. In Rust, you can access command-line arguments using the `std::env` module. This is essential for building practical tools and utilities that can be configured at runtime without modifying the code.

## Theory

When you run a Rust program like `cargo run arg1 arg2`, the program can access those arguments and process them accordingly. The arguments include:
- **Program name**: Usually the binary name (index 0)
- **Arguments**: User-provided values (index 1+)

### Why CLI Arguments Matter

- **Flexibility**: Program behavior changes based on input
- **Scripting**: Enables automation and batch processing
- **User Experience**: No need to modify code for different use cases
- **Portability**: Same binary works in different contexts

## Syntax

### Basic Argument Access

```rust
use std::env;

fn main() {
    // Get arguments as an iterator
    let args: Vec<String> = env::args().collect();

    // Access individual arguments
    let program_name = &args[0];
    let first_arg = &args[1];
    let all_args = &args[1..];  // Skip program name
}
```

### Safer Argument Access

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if argument exists before accessing
    if args.len() > 1 {
        let first_arg = &args[1];
        println!("First argument: {}", first_arg);
    } else {
        println!("No arguments provided");
    }
}
```

### Using Iterator Methods

```rust
use std::env;

fn main() {
    let mut args = env::args();

    // Skip program name
    args.next();

    // Get next argument safely
    if let Some(filename) = args.next() {
        println!("Processing: {}", filename);
    }
}
```

## Common Patterns

### Pattern 1: Simple Filename Argument

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    println!("Processing file: {}", filename);
}
```

### Pattern 2: Multiple Arguments

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <source> <destination>", args[0]);
        std::process::exit(1);
    }

    let source = &args[1];
    let destination = &args[2];

    println!("Copy from: {}", source);
    println!("Copy to: {}", destination);
}
```

### Pattern 3: Optional Arguments with Defaults

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_file = if args.len() > 1 {
        args[1].clone()
    } else {
        "input.txt".to_string()
    };

    let output_file = if args.len() > 2 {
        args[2].clone()
    } else {
        "output.txt".to_string()
    };

    println!("Input: {}", input_file);
    println!("Output: {}", output_file);
}
```

### Pattern 4: Flag-Based Arguments

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let verbose = args.contains(&"--verbose".to_string());
    let quiet = args.contains(&"--quiet".to_string());

    if verbose {
        println!("Verbose mode enabled");
    }
    if quiet {
        println!("Quiet mode enabled");
    }
}
```

### Pattern 5: Collecting Arguments by Type

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Process all arguments
    for (i, arg) in args.iter().enumerate() {
        println!("Arg {}: {}", i + 1, arg);
    }

    // Count arguments
    println!("Total: {} arguments", args.len());
}
```

## Common Mistakes

### Mistake 1: Not Checking Argument Count

```rust
// ❌ WRONG - Will panic if argument missing
let filename = &args[1];

// ✅ CORRECT - Check length first
if args.len() > 1 {
    let filename = &args[1];
}
```

### Mistake 2: Forgetting Program Name at Index 0

```rust
// ❌ WRONG - Index 0 is program name, not first user argument
let first_user_arg = &args[0];

// ✅ CORRECT
let first_user_arg = &args[1];
```

### Mistake 3: Not Handling Missing Arguments Gracefully

```rust
// ❌ WRONG - Poor error message
if args.len() < 2 {
    println!("Error");
}

// ✅ CORRECT - Clear usage message
if args.len() < 2 {
    eprintln!("Usage: {} <filename>", args[0]);
    std::process::exit(1);
}
```

### Mistake 4: Cloning When Not Necessary

```rust
// ❌ WRONG - Unnecessary clone
let filename: String = args[1].clone();

// ✅ CORRECT - Use reference
let filename: &str = &args[1];

// Or when you need ownership
let filename = args[1].to_string();
```

### Mistake 5: Not Validating Argument Values

```rust
// ❌ WRONG - No validation
let port = args[1];
make_connection(port);

// ✅ CORRECT - Parse and validate
if let Ok(port) = args[1].parse::<u16>() {
    make_connection(port);
} else {
    eprintln!("Invalid port: {}", args[1]);
}
```

## Real-World Examples

### Example 1: File Copier

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <source> <destination>", args[0]);
        std::process::exit(1);
    }

    let source = &args[1];
    let destination = &args[2];

    match fs::copy(source, destination) {
        Ok(bytes) => println!("Copied {} bytes", bytes),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Example 2: Configuration Tool

```rust
use std::env;

fn main() {
    let mut verbose = false;
    let mut config_file = "config.toml".to_string();

    let args: Vec<String> = env::args().skip(1).collect();
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--verbose" => verbose = true,
            "--config" => {
                if i + 1 < args.len() {
                    config_file = args[i + 1].clone();
                    i += 1;
                }
            }
            _ => println!("Unknown argument: {}", args[i]),
        }
        i += 1;
    }

    if verbose {
        println!("Using config: {}", config_file);
    }
}
```

### Example 3: Grep-like Tool

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <pattern> <file>", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];
    let filename = &args[2];

    match fs::read_to_string(filename) {
        Ok(contents) => {
            for line in contents.lines() {
                if line.contains(pattern) {
                    println!("{}", line);
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
```

## Related Concepts

### Prerequisites
- Module 01: Variables and Functions
- Module 02: Error Handling (Option/Result)
- Module 03: Testing and Documentation

### Follow-ups
- File I/O Basics (reading/writing files)
- Text Processing (parsing and formatting)
- Simple Algorithms (processing argument data)
- External crates: `clap`, `structopt`, `argh` for advanced CLI parsing

## Best Practices

1. **Always validate arguments** - Check length and content
2. **Provide clear error messages** - Help users understand what went wrong
3. **Use eprintln! for errors** - Separate error messages from normal output
4. **Document expected arguments** - Show usage in help message
5. **Consider using helper crates** - For complex CLI applications
6. **Handle edge cases** - Empty arguments, special characters, etc.
7. **Test with various inputs** - Verify behavior with different argument combinations

## Summary

Command-line arguments are essential for building practical Rust utilities. The `std::env::args()` function provides access to arguments, and careful validation ensures robust programs. For complex CLI applications, consider using dedicated crates that simplify argument parsing and provide automatic help generation.

## Practice Exercise Ideas

1. Create a program that takes a filename and prints its size
2. Build a simple calculator that takes numbers as arguments
3. Write a tool that copies files with optional verbose output
4. Implement a search tool that finds text in files
5. Create a configuration reader that accepts file paths as arguments

