# Key Takeaways: String Operations

## Quick Reference

### Creating Strings

```rust
// Literal
let s: &str = "hello";

// String::new
let s = String::new();

// String::from
let s = String::from("hello");

// to_string
let s = "hello".to_string();

// format! macro
let s = format!("Hello, {}", name);
```

### Basic Methods

```rust
s.len();                    // Byte length
s.chars().count();          // Character count
s.is_empty();               // Check if empty
s.contains("text");         // Check substring
s.starts_with("prefix");    // Check start
s.ends_with("suffix");      // Check end
s.find("text");             // Find position
```

### Modification (needs mut)

```rust
let mut s = String::from("hello");
s.push('!');                // Add char
s.push_str(" world");       // Add string
s.insert(0, 'H');           // Insert at position
s.remove(0);                // Remove at position
s.pop();                    // Remove last
s.clear();                  // Remove all
```

### Transformation

```rust
s.to_uppercase();           // HELLO
s.to_lowercase();           // hello
s.trim();                   // Remove whitespace
s.replace("old", "new");    // Replace substring
s.split(' ').collect();     // Split to Vec
s.chars().collect();        // Chars to Vec
```

## Essential Concepts

### 1. &str vs String

| Aspect | `&str` | `String` |
|--------|-------|--------|
| Type | String slice (reference) | Owned string |
| Storage | Stack reference | Heap data + stack reference |
| Mutability | Always immutable | Can be mutable |
| Size | Fixed/slice | Dynamic |
| Creation | Literals, slicing | new(), from(), to_string() |
| When to use | Function parameters | Ownership needed |

### 2. Creating Strings

```rust
// &str (string slice)
let s: &str = "literal";      // Static
let s: &str = &s_string;      // From String reference

// String (owned)
let s = String::new();        // Empty
let s = String::from("hello"); // From &str
let s = "hello".to_string();  // From &str
let s = format!("x{}", 10);   // Formatted
```

### 3. Critical: Byte vs Character Length

```rust
// ASCII: works fine
let s = "hello";
s.len();              // 5 bytes = 5 characters

// UTF-8: bytes ≠ characters
let s = "café";
s.len();              // 5 bytes!
s.chars().count();    // 4 characters

// Always use chars().count() for Unicode
```

### 4. Function Parameters Best Practice

```rust
// ✅ CORRECT: accepts both &str and String
fn greet(name: &str) {
    println!("Hello, {}", name);
}

greet("Alice");                  // &str works
greet(&String::from("Bob"));     // String reference works

// ❌ WRONG: only accepts String
fn greet(name: String) {
    println!("Hello, {}", name);
}

greet("Alice");  // ERROR: can't pass &str
```

### 5. Ownership with Strings

```rust
// String ownership
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2

// Borrowing with &str
let s1 = String::from("hello");
let s2 = &s1;  // Borrow, s1 still valid

// Mutable borrow for mutation
let mut s = String::from("hello");
s.push_str(" world");  // Requires mut
```

## Common Patterns

### Pattern 1: String building loop
```rust
let mut result = String::new();
for item in items {
    result.push_str(&item);
    result.push(' ');
}
```

### Pattern 2: Case-insensitive comparison
```rust
if text.to_lowercase() == "hello" {
    // Match found
}
```

### Pattern 3: Split and process
```rust
for word in text.split_whitespace() {
    let trimmed = word.trim();
    println!("{}", trimmed);
}
```

### Pattern 4: String formatting
```rust
let formatted = format!("User: {}, Age: {}", name, age);
println!("Debug: {:#?}", data);
```

### Pattern 5: Multi-line text
```rust
for line in text.lines() {
    println!("{}", line);
}
```

## Checklist: &str or String?

**Use &str when:**
- [ ] Writing function parameters
- [ ] Working with string literals
- [ ] Don't need to modify string
- [ ] Maximum flexibility needed

**Use String when:**
- [ ] Need to modify string
- [ ] Building string dynamically
- [ ] Taking ownership
- [ ] Storing in collections/structs

## Error Prevention

### ❌ DON'T: Mutate &str
```rust
let s: &str = "hello";
s.push_str(" world");  // ERROR: &str immutable
```

### ✅ DO: Use String for mutation
```rust
let mut s = String::from("hello");
s.push_str(" world");  // OK
```

### ❌ DON'T: Assume byte length = char count
```rust
let s = "café";
let len = s.len();  // 5 bytes, not 4 chars!
```

### ✅ DO: Use chars() for Unicode
```rust
let s = "café";
let len = s.chars().count();  // 4 characters
```

### ❌ DON'T: Restrict to String parameter
```rust
fn process(s: String) {
    println!("{}", s);
}

process("hello");  // ERROR: can't pass literal
```

### ✅ DO: Accept &str
```rust
fn process(s: &str) {
    println!("{}", s);
}

process("hello");  // OK
process(&my_string);  // OK
```

### ❌ DON'T: Index by byte position with Unicode
```rust
let s = "café";
let first = &s[0..1];  // May panic with non-ASCII
```

### ✅ DO: Use chars() for safe indexing
```rust
let s = "café";
let first: char = s.chars().next().unwrap();
```

### ❌ DON'T: Use + repeatedly
```rust
let mut s = String::from("a");
s = s + "b";  // Moves s
s = s + "c";  // Moves s again (inefficient)
```

### ✅ DO: Use push_str for loop
```rust
let mut s = String::from("a");
s.push_str("b");
s.push_str("c");
```

## Performance Tips

### String Building
- Use `push_str()` for loop building (efficient)
- Use `format!()` for complex formatting
- Avoid repeated `+` operations
- Consider `String::with_capacity()` if size known

### String Inspection
- `contains()` for substring checks
- `starts_with()` / `ends_with()` for prefix/suffix
- `find()` returns Option (efficient)
- `split()` for parsing

### Memory
- `&str` has zero allocation overhead
- `String` allocates on heap
- Each `format!()` allocates new String
- String capacity grows exponentially (avoid reallocations)

## String Methods Quick Lookup

| Method | Purpose | Example |
|--------|---------|---------|
| `len()` | Byte length | `s.len()` returns 5 |
| `chars().count()` | Character count | Unicode safe |
| `is_empty()` | Check if empty | returns bool |
| `contains()` | Check substring | `s.contains("x")` |
| `starts_with()` | Check prefix | `s.starts_with("h")` |
| `ends_with()` | Check suffix | `s.ends_with("o")` |
| `find()` | Find position | returns Option<usize> |
| `to_uppercase()` | Uppercase | `s.to_uppercase()` |
| `to_lowercase()` | Lowercase | `s.to_lowercase()` |
| `trim()` | Remove whitespace | returns &str |
| `trim_start()` | Trim left | returns &str |
| `trim_end()` | Trim right | returns &str |
| `split()` | Split string | returns iterator |
| `split_whitespace()` | Split by spaces | returns iterator |
| `replace()` | Replace substring | returns String |
| `chars()` | Get characters | returns iterator |
| `lines()` | Get lines | returns iterator |
| `push()` | Add char (mut) | `s.push('!')` |
| `push_str()` | Add &str (mut) | `s.push_str("text")` |
| `pop()` | Remove last (mut) | returns Option<char> |
| `clear()` | Remove all (mut) | `s.clear()` |

## Related Concepts

- **Data Types** - String type fundamentals
- **Ownership** - How Strings own data
- **Collections** - Vec<String>, HashMap with String keys
- **Iterators** - chars(), split(), lines()

## Time Estimates

- Reading this takeaway: 10-15 minutes
- Reviewing patterns: 10 minutes
- Practice drills: 20-30 minutes
- Total: 40-55 minutes

## Practice Questions

1. What's the difference between &str and String?
2. When should you use &str in function parameters?
3. Why does s.len() give bytes for UTF-8 strings?
4. How do you safely split a string into parts?
5. What's the difference between push() and push_str()?
6. How do you case-insensitive compare strings?
7. When would you use format! vs push_str?
8. How do you handle Unicode characters correctly?

## Common Conversions

| From | To | Code |
|------|-------|------|
| &str | String | `"text".to_string()` or `String::from("text")` |
| String | &str | `&my_string` |
| number | String | `42.to_string()` or `format!("{}", 42)` |
| bool | String | `true.to_string()` |
| char | String | `'c'.to_string()` |
| Vec<String> | String | `vec.join(", ")` |

## Real-World Scenarios

| Problem | Solution | Key Method |
|---------|----------|-----------|
| Trim user input | `input.trim()` | trim() |
| Case-insensitive match | `text.to_lowercase() == target` | to_lowercase() |
| Split CSV | `line.split(',')` | split() |
| Build SQL query | `format!("SELECT * FROM {}", table)` | format!() |
| Count words | `text.split_whitespace().count()` | split_whitespace() |
| Replace text | `text.replace("old", "new")` | replace() |
| Process lines | `text.lines()` | lines() |
| Parse command | `input.split_whitespace()` | split_whitespace() |

---

**Status**: Quick reference guide
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Beginner-Intermediate
**Part of**: Module 02 - Standard Library
