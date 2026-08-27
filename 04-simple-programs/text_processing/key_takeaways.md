# Text Processing - Key Takeaways

## Core String Methods

### Searching
```rust
// Contains substring
text.contains("pattern")

// Starts/ends with
text.starts_with("prefix")
text.ends_with("suffix")

// Find position
text.find("pattern")  // Option<usize>

// Count occurrences
text.matches("pattern").count()
```

### Splitting
```rust
// By delimiter
text.split(',')

// By whitespace
text.split_whitespace()

// Into lines
text.lines()

// Custom predicate
text.split(|c: char| c.is_whitespace())
```

### Transformation
```rust
// Case conversion
text.to_uppercase()
text.to_lowercase()

// Trimming
text.trim()           // Both sides
text.trim_start()     // Start only
text.trim_end()       // End only

// Replace
text.replace("old", "new")
```

## Character Operations

```rust
// Iterate characters
for ch in text.chars() { }

// Iterate bytes
for byte in text.bytes() { }

// First character
text.chars().next()

// Count characters
text.chars().count()
```

## Common Patterns

| Task | Code |
|------|------|
| Split by delimiter | `text.split(',')` |
| Process lines | `text.lines()` |
| Find substring | `text.find("pattern")` |
| Case insensitive search | `text.to_lowercase().contains("pattern")` |
| Count words | `text.split_whitespace().count()` |
| Remove whitespace | `text.split_whitespace().collect::<Vec<_>>().join("")` |

## Important Functions

```rust
// String creation from text
let s = "hello".to_string();
let s = String::from("hello");

// String from chars
let s: String = chars.iter().collect();

// Check if empty
text.is_empty()

// Get length
text.len()           // Bytes
text.chars().count() // Characters
```

## Important Notes

✓ Use `.trim()` on user input
✓ `.chars()` counts characters correctly with Unicode
✓ `.len()` returns bytes, not characters
✓ Case-sensitive comparison by default
✓ `.lines()` handles different line endings
✓ `.split_whitespace()` handles all whitespace
✓ UTF-8 encoding is automatic

## Processing Patterns

```rust
// Count word frequency
use std::collections::HashMap;
let mut freq = HashMap::new();
for word in text.split_whitespace() {
    *freq.entry(word).or_insert(0) += 1;
}

// Process each line
for line in text.lines() {
    if let Some(trimmed) = line.strip_prefix("  ") {
        println!("{}", trimmed);
    }
}

// Find and highlight
text.lines()
    .map(|line| if line.contains("keyword") {
        format!(">>> {}", line)
    } else {
        line.to_string()
    })
    .collect::<Vec<_>>()
```

## Edge Cases

```rust
// Empty string
if text.is_empty() { return; }

// No matches
if !text.contains("pattern") { }

// Single character
if let Some(ch) = text.chars().next() { }

// Split empty string
let parts: Vec<_> = "".split(',').collect();
// Result: [""]  - NOT empty vec!
```

## Quick Reference

```rust
use std::collections::HashMap;

// Read and process
let text = fs::read_to_string("file.txt")?;

// Count words
let word_count = text.split_whitespace().count();

// Frequency
let mut freq = HashMap::new();
for word in text.split_whitespace() {
    *freq.entry(word).or_insert(0) += 1;
}

// Find lines with pattern
text.lines()
    .filter(|line| line.contains("pattern"))
    .collect::<Vec<_>>()
```

## Common Mistakes to Avoid

1. ❌ Forgetting to trim user input
2. ❌ Using `.len()` instead of `.chars().count()`
3. ❌ Case-sensitive search when should be insensitive
4. ❌ Not handling empty strings
5. ❌ Inefficient string concatenation in loops

