# File I/O Basics - Key Takeaways

## Core Functions

### Reading Files
```rust
// Read entire file to String
fs::read_to_string("file.txt")?;

// Read file as bytes
fs::read("file.txt")?;

// Read line by line
let reader = BufReader::new(File::open("file.txt")?);
for line in reader.lines() { }
```

### Writing Files
```rust
// Write string to file (overwrites)
fs::write("file.txt", "content")?;

// Append to file
OpenOptions::new().append(true).open("file.txt")?;

// Manual write
let mut file = File::create("file.txt")?;
file.write_all(b"content")?;
```

## Common Patterns

| Operation | Function | Returns |
|-----------|----------|---------|
| Read entire text | `fs::read_to_string()` | `Result<String>` |
| Read binary | `fs::read()` | `Result<Vec<u8>>` |
| Write text | `fs::write()` | `Result<()>` |
| Append | `OpenOptions::new().append()` | File handle |
| Check exists | `Path::exists()` | `bool` |
| Get metadata | `fs::metadata()` | `Result<Metadata>` |

## Error Handling

```rust
// Match pattern
match fs::read_to_string("file.txt") {
    Ok(content) => println!("{}", content),
    Err(e) => eprintln!("Error: {}", e),
}

// Using ? operator
fn process() -> io::Result<()> {
    let contents = fs::read_to_string("file.txt")?;
    fs::write("output.txt", contents)?;
    Ok(())
}

// With unwrap_or
let contents = fs::read_to_string("file.txt")
    .unwrap_or_else(|_| "default".to_string());
```

## Important Functions

| Function | Purpose |
|----------|---------|
| `fs::read_to_string()` | Read text file |
| `fs::write()` | Create/overwrite file |
| `fs::read()` | Read binary file |
| `fs::metadata()` | Get file info |
| `Path::exists()` | Check if file exists |
| `OpenOptions::new()` | Advanced file operations |

## Important Notes

✓ Always handle `Result` errors explicitly
✓ Use `?` operator in functions returning Result
✓ Rust closes files automatically
✓ UTF-8 is default encoding
✓ Use appropriate function for data type (string vs binary)
✓ Flush explicitly if writing critical data

## Working with Lines

```rust
// Read entire file and process lines
let contents = fs::read_to_string("file.txt")?;
for line in contents.lines() {
    println!("{}", line);
}

// Read file line by line (memory efficient)
let file = File::open("file.txt")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;
    println!("{}", line);
}
```

## File Paths

```rust
use std::path::{Path, PathBuf};

// String path
let path = "folder/file.txt";

// PathBuf (owned, mutable)
let mut path = PathBuf::from("folder");
path.push("file.txt");

// Path (borrowed, immutable)
let path: &Path = Path::new("file.txt");
```

## Common Mistakes to Avoid

1. ❌ Using `.unwrap()` instead of error handling
2. ❌ Checking exists() then reading (race condition)
3. ❌ Hard-coding path separators
4. ❌ Forgetting to flush on important writes
5. ❌ Assuming files are UTF-8 without checking

## Quick Reference

```rust
use std::fs;

// Read all
let content = fs::read_to_string("file.txt")?;

// Write all
fs::write("file.txt", "content")?;

// Append
use std::fs::OpenOptions;
use std::io::Write;
let mut file = OpenOptions::new()
    .append(true)
    .open("file.txt")?;
writeln!(file, "new line")?;

// Exists
if fs::metadata("file.txt").is_ok() {
    // file exists
}
```

