# Text Processing

## Overview

Text processing is fundamental for many programs: parsing configuration files, analyzing logs, transforming data, and extracting information. Rust provides powerful string methods and patterns for efficient text manipulation. Understanding text processing enables building tools that work with real-world data.

## Theory

### Text Fundamentals

1. **Strings** - Collections of characters (UTF-8 encoded)
2. **Patterns** - Searching for substrings or text characteristics
3. **Parsing** - Converting text to structured data
4. **Transformation** - Changing text format or content
5. **Validation** - Checking if text meets criteria

### Character Encoding

- Rust strings use UTF-8 encoding
- Characters can be 1-4 bytes
- Iteration by `.chars()` works correctly with multi-byte characters
- Iteration by `.bytes()` works on raw bytes (useful for some operations)

### Performance Considerations

- Text operations can be expensive for large files
- Iterators are lazy and memory efficient
- Regular expressions are powerful but have overhead
- Simple string methods are often sufficient

## Syntax

### String Splitting

```rust
fn main() {
    let text = "apple,banana,cherry";

    // Split by delimiter
    for word in text.split(',') {
        println!("{}", word);
    }

    // Split by whitespace
    for word in text.split_whitespace() {
        println!("{}", word);
    }

    // Split by pattern
    for part in text.split(|c: char| c == ',' || c == ';') {
        println!("{}", part);
    }
}
```

### Pattern Matching

```rust
fn main() {
    let text = "Hello, World!";

    // Contains
    println!("{}", text.contains("World"));

    // Starts with
    println!("{}", text.starts_with("Hello"));

    // Ends with
    println!("{}", text.ends_with("!"));

    // Find position
    if let Some(pos) = text.find("World") {
        println!("Found at: {}", pos);
    }
}
```

### String Transformation

```rust
fn main() {
    let text = "Hello World";

    // Case conversion
    println!("{}", text.to_uppercase());
    println!("{}", text.to_lowercase());

    // Trimming
    println!("{}", "  hello  ".trim());
    println!("{}", "  hello  ".trim_start());
    println!("{}", "  hello  ".trim_end());

    // Replacing
    println!("{}", text.replace("World", "Rust"));
}
```

### Line Processing

```rust
fn main() {
    let text = "line 1\nline 2\nline 3";

    for line in text.lines() {
        println!("{}", line);
    }
}
```

## Common Patterns

### Pattern 1: Word Frequency Counter

```rust
use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }

    counts
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    let frequencies = count_words(text);

    for (word, count) in frequencies.iter() {
        println!("{}: {}", word, count);
    }
}
```

### Pattern 2: Line-by-Line Processing

```rust
fn process_lines(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.to_uppercase())
        .collect()
}

fn main() {
    let text = "hello\n\nworld\nrust";
    let processed = process_lines(text);

    for line in processed {
        println!("{}", line);
    }
}
```

### Pattern 3: CSV Parsing

```rust
fn parse_csv_line(line: &str) -> Vec<&str> {
    line.split(',')
        .map(|field| field.trim())
        .collect()
}

fn main() {
    let csv = "John, 30, Engineer";
    let fields = parse_csv_line(csv);

    for (i, field) in fields.iter().enumerate() {
        println!("Field {}: {}", i, field);
    }
}
```

### Pattern 4: Character Filtering

```rust
fn remove_special_chars(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

fn main() {
    let text = "Hello, World! #2024";
    println!("{}", remove_special_chars(text));
}
```

### Pattern 5: Text Search and Highlighting

```rust
fn find_and_highlight(text: &str, keyword: &str) -> Vec<String> {
    text.lines()
        .map(|line| {
            if line.contains(keyword) {
                format!(">>> {}", line)
            } else {
                line.to_string()
            }
        })
        .collect()
}

fn main() {
    let text = "apple\nbanana\napricot\nblueberry";
    let highlighted = find_and_highlight(text, "apple");

    for line in highlighted {
        println!("{}", line);
    }
}
```

## Common Mistakes

### Mistake 1: Not Trimming Input

```rust
// ❌ WRONG - Spaces affect comparison
let line = "  hello  ";
if line == "hello" { }  // false because of spaces

// ✅ CORRECT - Trim first
if line.trim() == "hello" { }  // true
```

### Mistake 2: Case Sensitivity Issues

```rust
// ❌ WRONG - Case-sensitive comparison
if text.contains("Hello") { }
// Won't find "hello", "HELLO", "hElLo"

// ✅ CORRECT - Convert to consistent case
if text.to_lowercase().contains("hello") { }
```

### Mistake 3: Assuming Single-Byte Characters

```rust
// ❌ WRONG - Won't work correctly with emoji/unicode
let len = text.len();  // Bytes, not characters

// ✅ CORRECT - Count actual characters
let len = text.chars().count();
```

### Mistake 4: Not Handling Empty Strings

```rust
// ❌ WRONG - May panic or give wrong results
let first_char = text.chars().next().unwrap();

// ✅ CORRECT - Handle empty case
if let Some(first_char) = text.chars().next() {
    println!("{}", first_char);
}
```

### Mistake 5: Inefficient String Concatenation

```rust
// ❌ WRONG - Creates new string each time
let mut result = String::new();
for word in words {
    result = result + word + " ";  // Inefficient
}

// ✅ CORRECT - Collect then join
let result = words.iter()
    .map(|w| *w)
    .collect::<Vec<_>>()
    .join(" ");
```

## Real-World Examples

### Example 1: Log File Analyzer

```rust
use std::fs;

fn analyze_logs(filename: &str) -> std::io::Result<()> {
    let contents = fs::read_to_string(filename)?;

    let mut error_count = 0;
    let mut warning_count = 0;

    for line in contents.lines() {
        if line.contains("ERROR") {
            error_count += 1;
        } else if line.contains("WARNING") {
            warning_count += 1;
        }
    }

    println!("Errors: {}", error_count);
    println!("Warnings: {}", warning_count);
    Ok(())
}
```

### Example 2: Configuration Parser

```rust
fn parse_config(text: &str) -> std::collections::HashMap<String, String> {
    let mut config = std::collections::HashMap::new();

    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            config.insert(
                key.trim().to_string(),
                value.trim().to_string()
            );
        }
    }

    config
}
```

### Example 3: Text Transformation

```rust
fn convert_to_snake_case(text: &str) -> String {
    text.replace(' ', "_")
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn main() {
    println!("{}", convert_to_snake_case("Hello World Example"));
}
```

## Related Concepts

### Prerequisites
- Module 01: Strings, Functions
- Module 02: Collections, Iterators
- Module 04: File I/O

### Follow-ups
- Regular expressions (advanced pattern matching)
- Serialization (parsing structured text: JSON, TOML)
- Natural language processing (advanced text analysis)
- External crates: `regex`, `serde_json`, `toml`

## Best Practices

1. **Trim input** - Remove leading/trailing whitespace
2. **Handle encoding** - Be aware of UTF-8 multi-byte characters
3. **Use built-in methods** - Before writing custom logic
4. **Choose right iterators** - `.chars()` vs `.bytes()` vs `.lines()`
5. **Test edge cases** - Empty strings, special characters
6. **Consider performance** - For large files, use streaming
7. **Document assumptions** - What encoding, line endings, etc.

## Summary

Text processing is essential for practical Rust programs. Rust's comprehensive string methods and UTF-8 handling make text work safe and efficient. Understanding common patterns enables quick development of tools and utilities.

## Practice Exercise Ideas

1. Create a line-number tool (like Unix `nl`)
2. Build a simple word counter with frequency analysis
3. Implement a grep-like search tool
4. Create a configuration file parser
5. Build a text case converter (camelCase, snake_case, etc.)

