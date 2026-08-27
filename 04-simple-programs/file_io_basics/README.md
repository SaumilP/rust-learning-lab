# File I/O Basics

## Overview

File input/output (I/O) is fundamental for practical programs. Rust's `std::fs` module provides safe, efficient ways to read and write files. Unlike some languages, Rust forces you to handle errors explicitly, resulting in more robust file handling code.

## Theory

### File Operations

Files on disk represent persistent data storage. When working with files:
- **Opening** - Establishing a connection to a file
- **Reading** - Extracting data from a file
- **Writing** - Storing data to a file
- **Closing** - Releasing file resources (automatic in Rust)

### Why Explicit Error Handling Matters

File operations can fail for many reasons:
- File doesn't exist
- Permission denied
- Disk is full
- Path is invalid

Rust's `Result` type forces you to handle these cases explicitly.

### Resource Management

Rust automatically closes files when they go out of scope, preventing resource leaks. This is guaranteed by the language, not by best-effort cleanup.

## Syntax

### Reading Entire File to String

```rust
use std::fs;

fn main() {
    match fs::read_to_string("file.txt") {
        Ok(contents) => println!("{}", contents),
        Err(error) => eprintln!("Error reading file: {}", error),
    }
}
```

### Writing String to File

```rust
use std::fs;

fn main() {
    let contents = "Hello, World!";
    match fs::write("output.txt", contents) {
        Ok(_) => println!("File written successfully"),
        Err(e) => eprintln!("Error writing file: {}", e),
    }
}
```

### Line-by-Line Reading

```rust
use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = fs::File::open("data.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
```

### Appending to File

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open("log.txt")?;

    writeln!(file, "New log entry")?;
    Ok(())
}
```

## Common Patterns

### Pattern 1: Simple File Read with Error Message

```rust
use std::fs;

fn main() {
    match fs::read_to_string("config.txt") {
        Ok(config) => {
            println!("Configuration loaded");
            println!("Content: {}", config);
        }
        Err(err) => {
            eprintln!("Failed to read config: {}", err);
            std::process::exit(1);
        }
    }
}
```

### Pattern 2: Reading and Processing Lines

```rust
use std::fs;

fn main() {
    match fs::read_to_string("data.txt") {
        Ok(contents) => {
            for line in contents.lines() {
                if !line.is_empty() {
                    println!("Processing: {}", line);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Pattern 3: Creating and Writing to New File

```rust
use std::fs;

fn main() {
    let content = "Line 1\nLine 2\nLine 3\n";

    match fs::write("output.txt", content) {
        Ok(_) => println!("File created and written"),
        Err(e) => eprintln!("Failed to write: {}", e),
    }
}
```

### Pattern 4: File Exists Check

```rust
use std::fs;
use std::path::Path;

fn main() {
    let path = "file.txt";

    if Path::new(path).exists() {
        println!("File exists");
        if let Ok(metadata) = fs::metadata(path) {
            println!("File size: {} bytes", metadata.len());
        }
    } else {
        println!("File does not exist");
    }
}
```

### Pattern 5: Using Result with Question Mark Operator

```rust
use std::fs;
use std::io;

fn read_and_process(filename: &str) -> io::Result<String> {
    let contents = fs::read_to_string(filename)?;
    let processed = contents.to_uppercase();
    Ok(processed)
}

fn main() -> io::Result<()> {
    let result = read_and_process("input.txt")?;
    fs::write("output.txt", result)?;
    println!("Processing complete");
    Ok(())
}
```

## Common Mistakes

### Mistake 1: Not Handling Errors

```rust
// ❌ WRONG - Will panic if file doesn't exist
let contents = fs::read_to_string("file.txt").unwrap();

// ✅ CORRECT - Explicit error handling
match fs::read_to_string("file.txt") {
    Ok(contents) => println!("{}", contents),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Mistake 2: Checking File Existence Separately

```rust
// ❌ WRONG - Race condition: file could be deleted between check and read
if Path::new("file.txt").exists() {
    let contents = fs::read_to_string("file.txt")?;
}

// ✅ CORRECT - Just try to read and handle error
match fs::read_to_string("file.txt") {
    Ok(contents) => { /* process */ }
    Err(e) => eprintln!("Cannot read: {}", e),
}
```

### Mistake 3: Not Considering Path Separators

```rust
// ❌ WRONG - Hard-coded path separator (Windows vs Unix)
let path = "folder/file.txt";

// ✅ CORRECT - Use path module or standard separator
use std::path::PathBuf;
let path = PathBuf::from("folder").join("file.txt");
```

### Mistake 4: Forgetting to Flush Output

```rust
use std::fs::File;
use std::io::Write;

// ❌ WRONG - Data might not be written if program exits
let mut file = File::create("output.txt")?;
file.write_all(b"data")?;
// Data might not be on disk yet

// ✅ CORRECT - Explicit flush ensures data is written
file.flush()?;
```

### Mistake 5: Not Handling UTF-8 Encoding Issues

```rust
// ❌ WRONG - Assumes all files are UTF-8
let contents = fs::read_to_string("file.bin")?;

// ✅ CORRECT - Use read_to_bytes for binary data
let bytes = fs::read("file.bin")?;
```

## Real-World Examples

### Example 1: Text File Processor

```rust
use std::fs;
use std::io;

fn process_file(input: &str, output: &str) -> io::Result<()> {
    let contents = fs::read_to_string(input)?;

    let processed: String = contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_uppercase())
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(output, processed)?;
    println!("Processed {} to {}", input, output);
    Ok(())
}

fn main() -> io::Result<()> {
    process_file("input.txt", "output.txt")?;
    Ok(())
}
```

### Example 2: Log File Appender

```rust
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

fn log_message(message: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("app.log")?;

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    writeln!(file, "[{}] {}", timestamp, message)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    log_message("Application started")?;
    log_message("Processing data")?;
    log_message("Application finished")?;
    Ok(())
}
```

### Example 3: File Comparison

```rust
use std::fs;

fn files_equal(path1: &str, path2: &str) -> std::io::Result<bool> {
    let content1 = fs::read_to_string(path1)?;
    let content2 = fs::read_to_string(path2)?;
    Ok(content1 == content2)
}

fn main() -> std::io::Result<()> {
    match files_equal("file1.txt", "file2.txt")? {
        true => println!("Files are identical"),
        false => println!("Files are different"),
    }
    Ok(())
}
```

## Related Concepts

### Prerequisites
- Module 01: Functions and Error Handling Basics
- Module 02: Error Handling (Result, Option)
- Module 04: CLI Arguments

### Follow-ups
- Text Processing (parsing file contents)
- Simple Algorithms (processing file data)
- External crates: `serde` for file parsing, `walkdir` for directory traversal
- Binary I/O (reading non-text files)

## Best Practices

1. **Always handle errors** - Use `?` operator or match expressions
2. **Use appropriate functions** - `read_to_string` for text, `read` for binary
3. **Consider file encoding** - UTF-8 is default but not universal
4. **Flush when necessary** - Ensure data reaches disk
5. **Validate paths** - Use `Path` and `PathBuf` for safety
6. **Use high-level functions** - Prefer `fs::write` over manual `File` handling
7. **Close files properly** - Rust does this automatically

## Summary

File I/O is essential for practical programs. Rust's `std::fs` module provides safe, ergonomic access to the filesystem. The explicit error handling prevents common bugs like missing error cases. The automatic resource cleanup ensures files are properly closed.

## Practice Exercise Ideas

1. Create a program that reads a text file and prints each line with line numbers
2. Build a tool that copies a file and reports bytes copied
3. Write a simple word counter that reads a file
4. Implement a text file merger that combines multiple files
5. Create a backup tool that copies files with timestamps

