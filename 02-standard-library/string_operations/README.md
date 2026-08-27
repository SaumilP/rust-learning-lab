# Concept: String Operations

## Overview

Strings are fundamental data structures for working with text in Rust. However, Rust distinguishes between different string types with different semantics: `&str` (string slices), `String` (owned strings), and other specialized types. Understanding when and how to use each type is crucial for writing efficient, idiomatic Rust code. This concept covers string manipulation, conversion, and best practices.

## Learning Objectives

By the end of this concept, you will understand:
- Difference between `&str` (slice) and `String` (owned)
- Creating and initializing strings
- Common string methods and operations
- String conversion and formatting
- When to use each string type
- Performance implications of different approaches
- Working with string ownership and borrowing

## Theory

### &str vs String

Rust has two main string types, and understanding the distinction is essential:

**`&str` - String Slice**

A string slice is a reference to a sequence of bytes that represent valid UTF-8. It's immutable:

```rust
let s: &str = "hello";        // String literal
let s: &str = &String::from("hello");  // Reference to String

// String slices have:
// - Fixed length at compile time (for literals)
// - No ownership (borrowed)
// - Can't be modified
// - Very efficient
```

**`String` - Owned String**

A String is a growable, heap-allocated UTF-8 string that owns its data:

```rust
let s: String = String::new();
let s = String::from("hello");
let s = "hello".to_string();
let s = format!("Hello, {}", name);

// String has:
// - Dynamic length
// - Owns its data on the heap
// - Can be modified (if mut)
// - More overhead than &str
```

**Key Differences**

| Aspect | `&str` | `String` |
|--------|-------|--------|
| Storage | Stack (reference) | Heap (data) + Stack (reference) |
| Mutability | Always immutable | Can be mutable |
| Size | Fixed or slice | Dynamic |
| Creation | Literals, slicing | `new()`, `from()`, `to_string()` |
| Methods | Many (borrowed) | Many (owned) |
| Performance | Very fast | Slight overhead |
| Ownership | Borrowed | Owned |

### Creating Strings

**String Literals (`&'static str`)**

```rust
let s = "hello";  // Type: &str
let s = "hello world";
let s = "line1\nline2";  // Escape sequences
let s = r"raw \n string";  // Raw string (no escapes)
```

**String::new() and String::from()**

```rust
let s = String::new();  // Empty string
let s = String::from("hello");  // From &str
let s = String::from("world").to_uppercase();
```

**String Interpolation**

```rust
let name = "Alice";
let age = 30;

// Using format! macro
let s = format!("My name is {}, age {}", name, age);

// Using println! (doesn't return string)
println!("My name is {}, age {}", name, age);

// Multiple placeholders
let s = format!("{} + {} = {}", 2, 3, 5);
```

**Converting to String**

```rust
let s = "hello".to_string();  // &str to String
let s = 42.to_string();       // number to String
let s = true.to_string();     // bool to String
```

### String Methods

**Length and Capacity**

```rust
let s = String::from("hello");

s.len();           // 5 (bytes, not characters!)
s.chars().count(); // 5 (actual character count)
s.is_empty();      // false
s.capacity();      // Internal capacity
```

**Modification**

```rust
let mut s = String::from("hello");

s.push('!');                // Add character
s.push_str(" world");       // Add string slice
s.insert(5, '!');           // Insert at position
s.remove(0);                // Remove at position (returns char)
s.pop();                    // Remove last char (returns Option)
s.clear();                  // Remove all
```

**Searching and Slicing**

```rust
let s = "hello world";

s.contains("world");        // true
s.starts_with("hello");     // true
s.ends_with("world");       // true
s.find("world");            // Some(6)
s.rfind("o");               // Some(7) - right find

// String slicing (by byte index)
&s[0..5];                   // "hello"
&s[6..];                    // "world"
```

**Case Conversion**

```rust
let s = "Hello World";

s.to_uppercase();           // "HELLO WORLD"
s.to_lowercase();           // "hello world"
```

**Trimming**

```rust
let s = "  hello  ";

s.trim();                   // "hello"
s.trim_start();             // "hello  "
s.trim_end();               // "  hello"
s.trim_matches('h');        // "  ello  "
```

**Splitting**

```rust
let s = "hello world foo";

s.split(' ').collect::<Vec<_>>();  // vec!["hello", "world", "foo"]
s.split_whitespace().collect::<Vec<_>>();  // Same, more flexible
s.chars().collect::<Vec<_>>();     // vec!['h','e','l','l','o'...]
```

**Replacing**

```rust
let s = "hello world";

s.replace("world", "Rust");         // "hello Rust"
s.replace('l', "L");                // "heLLo worLd"
```

### String vs &str Parameters

**In function signatures**, prefer `&str` unless you need ownership:

```rust
// ✅ Good: accepts both String and &str
fn greet(name: &str) {
    println!("Hello, {}", name);
}

// ❌ Avoid: only accepts String, less flexible
fn greet(name: String) {
    println!("Hello, {}", name);
}

// Usage - both work with &str parameter
greet("Alice");                          // &str literal
greet(&String::from("Bob"));             // &String
```

### String Formatting

**Format specifiers**

```rust
// Basic
println!("{}", value);

// Named arguments
println!("{name} is {age}", name = "Alice", age = 30);

// Multiple formatting
println!("{:?}", vec![1, 2, 3]);  // Debug format
println!("{:#?}", complex_struct);  // Pretty debug

// Width and precision
println!("{:5}", 42);         // "   42"
println!("{:<5}", 42);        // "42   " (left-aligned)
println!("{:>5}", 42);        // "   42" (right-aligned)
println!("{:.2}", 3.14159);   // "3.14" (2 decimals)
```

### String Ownership and Borrowing

**Ownership rules affect strings**

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2, s1 no longer valid

// To avoid move, borrow
let s1 = String::from("hello");
let s2 = &s1;  // Borrow instead of move

// Mutable borrow for modification
let mut s = String::from("hello");
let s_ref = &mut s;
s_ref.push_str(" world");
```

**String slicing and lifetimes**

```rust
let s = String::from("hello world");
let hello = &s[0..5];  // Slice (borrow)

// Slices have lifetime tied to original String
// s.clear();  // ❌ ERROR: can't clear while slice exists
```

## Syntax

### String Creation

```rust
// Literals
let s = "hello";

// From String::from
let s = String::from("hello");

// From to_string
let s = "hello".to_string();

// From format!
let s = format!("Hello, {}", name);

// From interpolation with String methods
let s = format!("{} world!", "hello");
```

### String Modification

```rust
let mut s = String::new();
s.push('h');
s.push_str("ello");
s.insert(0, 'H');
s.remove(0);
s.pop();
s.clear();
```

### String Inspection

```rust
s.len();
s.is_empty();
s.contains("substring");
s.starts_with("prefix");
s.ends_with("suffix");
s.find("substring");
```

### String Transformation

```rust
s.to_uppercase();
s.to_lowercase();
s.trim();
s.replace("old", "new");
s.split(' ').collect();
s.chars().collect();
```

## Common Patterns

### Pattern 1: Building strings dynamically

```rust
let mut result = String::new();
for word in words {
    result.push_str(word);
    result.push(' ');
}
result.trim().to_string()
```

### Pattern 2: String formatting for display

```rust
fn format_user_info(name: &str, email: &str, age: u32) -> String {
    format!("{} ({}) - {}", name, email, age)
}

let info = format_user_info("Alice", "alice@example.com", 30);
```

### Pattern 3: Parsing and processing

```rust
let input = "apple,banana,orange";
let fruits: Vec<&str> = input.split(',').collect();

for fruit in fruits {
    let trimmed = fruit.trim();
    println!("{}", trimmed);
}
```

### Pattern 4: Case-insensitive comparison

```rust
let user_input = "Hello";
let target = "hello";

if user_input.to_lowercase() == target {
    println!("Match!");
}
```

### Pattern 5: Multi-line string handling

```rust
let text = "Line 1
Line 2
Line 3";

for line in text.lines() {
    println!("{}", line);
}
```

### Pattern 6: String concatenation

```rust
// Using format!
let s1 = "hello";
let s2 = "world";
let combined = format!("{} {}", s1, s2);

// Using push_str with owned String
let mut result = String::from("hello");
result.push_str(" world");

// Using + operator (consumes left String)
let s1 = String::from("hello");
let s2 = " world";
let combined = s1 + s2;  // s1 moved
```

## Common Mistakes

### Mistake 1: Confusing &str and String

```rust
// ❌ ERROR: can't mutate &str
let s: &str = "hello";
s.push_str(" world");  // ERROR: &str is immutable

// ✅ CORRECT: use String for mutation
let mut s = String::from("hello");
s.push_str(" world");
```

### Mistake 2: Byte indexing vs character indexing

```rust
let s = "hello";
// ✅ This works for ASCII
let first = &s[0..1];  // "h"

// ❌ This fails with non-ASCII
let s = "café";
let first = &s[0..1];  // ERROR: invalid byte index
// Use chars() for Unicode
let first: char = s.chars().next().unwrap();
```

### Mistake 3: Ownership issues with String parameters

```rust
// ❌ Less flexible: only takes String
fn process(s: String) {
    println!("{}", s);
}

process("hello");  // ERROR: can't pass &str literal

// ✅ Better: accepts &str
fn process(s: &str) {
    println!("{}", s);
}

process("hello");  // OK
process(&String::from("world"));  // OK
```

### Mistake 4: Inefficient string building

```rust
// ❌ Inefficient: many allocations
let mut result = String::new();
for item in items {
    result = result + &item;  // Creates new String each iteration
}

// ✅ Efficient: single String mutated
let mut result = String::new();
for item in items {
    result.push_str(&item);
}
```

### Mistake 5: Panicking on indexing

```rust
// ❌ Can panic with non-ASCII
let s = "hello";
let ch = &s[0];  // ERROR: indexing returns byte

// ✅ Safe: use chars
let s = "hello";
let ch = s.chars().next().unwrap();
```

### Mistake 6: Inefficient substring search

```rust
// ❌ Multiple iterations
if s.contains("a") && s.contains("b") {
    // Process
}

// ✅ Single pass or efficient method
if s.contains("a") {
    if s.contains("b") {
        // Process
    }
}
```

## Real-World Examples

### Example 1: CSV line parser

```rust
fn parse_csv_line(line: &str) -> Vec<String> {
    line.split(',')
        .map(|field| field.trim().to_string())
        .collect()
}

let data = "John, 30, Engineer";
let fields = parse_csv_line(data);
// ["John", "30", "Engineer"]
```

### Example 2: Text normalization

```rust
fn normalize_text(text: &str) -> String {
    text.trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

let input = "  Hello  WORLD!!! ";
let normalized = normalize_text(input);  // "hello world"
```

### Example 3: Template substitution

```rust
fn substitute(template: &str, values: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (key, value) in values {
        let placeholder = format!("{{{}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}

let template = "Hello {name}, you are {age} years old";
let values = vec![("name", "Alice"), ("age", "30")];
let result = substitute(template, &values);
// "Hello Alice, you are 30 years old"
```

### Example 4: Word frequency counter

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

let text = "hello world hello";
let freq = count_words(text);
// {"hello": 2, "world": 1}
```

### Example 5: Command-line argument processor

```rust
fn parse_command(input: &str) -> (String, Vec<String>) {
    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap_or("").to_string();
    let args = parts.map(|s| s.to_string()).collect();
    (command, args)
}

let cmd = "echo hello world";
let (command, args) = parse_command(cmd);
// command = "echo", args = ["hello", "world"]
```

## Related Concepts

### Prerequisites
- **Data Types** - Understanding string types as types
- **Functions** - Passing strings as parameters
- **Ownership** - How strings move and borrow

### What comes next
- **Iterators** - Advanced string iteration and transformation
- **Collections** - Using strings in HashMap, Vec, etc.
- **Error Handling** - Result types for string parsing

### Cross-references
- Module 01: Variables, Data Types, Functions
- Module 02: Collections (storing strings), Iterators (processing strings)
- Module 06: Trait implementations for custom string types

## Best Practices

### Prefer &str in function signatures

```rust
// ✅ Good: flexible, accepts both &str and String
fn process(text: &str) -> usize {
    text.len()
}

// ❌ Avoid: only accepts String
fn process(text: String) -> usize {
    text.len()
}
```

### Use format! for complex string building

```rust
// ✅ Good: readable, handles formatting
let result = format!("User: {}, Age: {}", name, age);

// ❌ Less clear: manual concatenation
let result = name.to_string() + ", Age: " + &age.to_string();
```

### Be aware of Unicode

```rust
// ✅ Good: handles Unicode correctly
let len = s.chars().count();

// ❌ Wrong: byte length, not character count
let len = s.len();
```

### Use appropriate methods

```rust
// ✅ Good: semantic clarity
if s.is_empty() { }
if s.contains("text") { }

// ❌ Less clear: manual checks
if s.len() == 0 { }
if s.find("text").is_some() { }
```

### Avoid unnecessary allocations

```rust
// ✅ Good: borrows, no allocation
fn validate(input: &str) -> bool {
    !input.is_empty()
}

// ❌ Unnecessary: creates String
fn validate(input: &str) -> bool {
    let s = input.to_string();
    !s.is_empty()
}
```

## Summary

- **&str** - String slice, immutable, borrowed, stack-based reference
- **String** - Owned string, mutable, heap-allocated, dynamic
- **Choice**: Use `&str` in function signatures for flexibility
- **Modification**: Only String can be modified (with mut)
- **Formatting**: Use `format!` macro for string composition
- **Methods**: Rich set of transformation methods available
- **UTF-8**: Strings are always valid UTF-8

## Key Takeaways

1. Rust has two main string types: `&str` and `String`
2. Use `&str` for function parameters to accept both types
3. String methods enable efficient text manipulation
4. Strings are always UTF-8 encoded
5. format! macro is the preferred way to build strings
6. Be aware of byte vs character length with Unicode
7. Push/push_str for mutable string building is efficient

## Practice Exercise Ideas

1. Write a function to reverse a string (handle Unicode)
2. Create a simple template substitution engine
3. Implement a text normalizer (trim, lowercase, remove special chars)
4. Build a word frequency analyzer
5. Parse a simple CSV format

---

**Time to complete this concept**: 1.5-2 hours
**Difficulty**: Beginner-Intermediate
**Prerequisite**: Data Types, Functions, Ownership basics
**Next concept**: Iterator Patterns

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
