# Kata: String Manipulation Problems

## Problem 1: Reverse String

**Difficulty**: Beginner
**Concepts**: String manipulation, iterators
**Prerequisites**: 01-core-fundamentals (Data Types, Functions)

### Problem Statement

Write a function that takes a string and returns it reversed.

### Requirements
- Take input as `&str`
- Return a `String` with characters in reverse order
- Handle empty strings

### Function Signature
```rust
pub fn reverse_string(s: &str) -> String {
    // Your implementation here
}
```

### Test Cases
```
Input: "hello"
Output: "olleh"

Input: "Rust"
Output: "tsuR"

Input: ""
Output: ""

Input: "a"
Output: "a"
```

### Hints
- Consider using `.chars().rev().collect()`
- Don't forget to handle unicode characters correctly

### Related Concepts
- String vs &str
- Character iteration
- Collecting iterators

---

## Problem 2: Count Vowels

**Difficulty**: Beginner
**Concepts**: String iteration, pattern matching
**Prerequisites**: 01-core-fundamentals (Control Flow, Data Types)

### Problem Statement

Write a function that counts the number of vowels (a, e, i, o, u) in a string. Case-insensitive.

### Requirements
- Count both uppercase and lowercase vowels
- Return count as `usize`
- Only count English vowels

### Function Signature
```rust
pub fn count_vowels(s: &str) -> usize {
    // Your implementation here
}
```

### Test Cases
```
Input: "hello world"
Output: 3

Input: "Rust Programming"
Output: 3

Input: "aeiouAEIOU"
Output: 10

Input: "xyz"
Output: 0

Input: ""
Output: 0
```

### Hints
- Use `.chars()` to iterate over characters
- Match statements can work with multiple patterns
- Convert to lowercase for case-insensitive comparison

### Related Concepts
- Character iteration
- Pattern matching
- String methods

---

## Problem 3: Capitalize Words

**Difficulty**: Beginner
**Concepts**: String operations, word splitting
**Prerequisites**: 01-core-fundamentals (Functions, Control Flow)

### Problem Statement

Write a function that capitalizes the first letter of each word in a string.

### Requirements
- Capitalize first character of each word
- Leave other characters unchanged
- Words are separated by spaces
- Handle empty strings and strings with multiple spaces

### Function Signature
```rust
pub fn capitalize_words(s: &str) -> String {
    // Your implementation here
}
```

### Test Cases
```
Input: "hello world"
Output: "Hello World"

Input: "rust programming language"
Output: "Rust Programming Language"

Input: "a quick brown fox"
Output: "A Quick Brown Fox"

Input: ""
Output: ""

Input: "single"
Output: "Single"
```

### Hints
- Split string by spaces using `.split(' ')`
- Use `.chars()` to access individual characters
- Capitalize using `.to_uppercase()` on first character
- Remember to rejoin with spaces

### Related Concepts
- String splitting and joining
- Character case conversion
- Iterator manipulation

---

## Problem 4: Remove Whitespace

**Difficulty**: Beginner
**Concepts**: String filtering, iterators
**Prerequisites**: 01-core-fundamentals (Functions, Data Types)

### Problem Statement

Write a function that removes all whitespace characters from a string.

### Requirements
- Remove all spaces, tabs, newlines, etc.
- Return cleaned string
- Preserve all non-whitespace characters

### Function Signature
```rust
pub fn remove_whitespace(s: &str) -> String {
    // Your implementation here
}
```

### Test Cases
```
Input: "hello world"
Output: "helloworld"

Input: "  spaces  everywhere  "
Output: "spaceseverywhere"

Input: "no\tspaces\nhere"
Output: "nospaceshere"

Input: ""
Output: ""

Input: "   "
Output: ""
```

### Hints
- Use `.filter()` on chars iterator
- `char::is_whitespace()` checks if character is whitespace
- Collect filtered chars back into String

### Related Concepts
- Character properties
- Iterator filtering
- Character case classification

---

## Problem 5: Snake Case Conversion

**Difficulty**: Beginner
**Concepts**: String transformation, iteration
**Prerequisites**: 01-core-fundamentals (Functions, Control Flow)

### Problem Statement

Write a function that converts a string to snake_case.

### Requirements
- Convert spaces to underscores
- Convert uppercase to lowercase
- Remove non-alphanumeric characters except underscores
- Handle multiple consecutive spaces as single underscore

### Function Signature
```rust
pub fn to_snake_case(s: &str) -> String {
    // Your implementation here
}
```

### Test Cases
```
Input: "Hello World"
Output: "hello_world"

Input: "Rust Programming"
Output: "rust_programming"

Input: "convert this STRING"
Output: "convert_this_string"

Input: ""
Output: ""

Input: "already_snake_case"
Output: "already_snake_case"
```

### Hints
- Replace spaces with underscores
- Convert to lowercase with `.to_lowercase()`
- Filter to keep only alphanumeric and underscore
- Handle multiple spaces by removing duplicates

### Related Concepts
- String replacement
- Case conversion
- Character filtering

---

## Summary of Kata: String Manipulation

These problems practice:
- Basic string operations
- Character iteration
- Simple transformations
- Building strings from parts

**Recommended Order**: Complete problems 1-5 sequentially
**Time Estimate**: 30-45 minutes total
**Difficulty Progression**: 1→2→3→4→5 (slight increase)
