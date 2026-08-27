# Beginner Level: Intermediate String Problems

## Problem 1: Anagram Checker

**Difficulty**: Beginner-Intermediate
**Concepts**: String sorting, comparison
**Prerequisites**: 02-standard-library (String Operations, Collections)

### Problem Statement

Write a function that checks if two strings are anagrams of each other. Two words are anagrams if they contain the same letters in different orders.

### Requirements
- Accept two string references
- Return true if anagrams, false otherwise
- Case-insensitive comparison
- Ignore spaces
- Handle empty strings

### Function Signature
```rust
pub fn is_anagram(word1: &str, word2: &str) -> bool {
    // Your implementation here
}
```

### Test Cases
```
Input: "listen", "silent"
Output: true

Input: "Hello", "Olleh"
Output: true

Input: "rust", "star"
Output: false

Input: "a gentleman", "elegant man"
Output: true (ignoring spaces)

Input: "abc", "def"
Output: false

Input: "", ""
Output: true
```

### Hints
- Convert both strings to lowercase
- Remove spaces
- Sort characters in both
- Compare sorted results
- Or use character counts with HashMap

### Algorithm Approach
1. Normalize: lowercase + remove spaces
2. Sort characters
3. Compare sorted strings

### Related Concepts
- Character sorting
- String normalization
- Comparison operations

---

## Problem 2: Palindrome Validator

**Difficulty**: Beginner-Intermediate
**Concepts**: String reversal, comparison
**Prerequisites**: 02-standard-library (String Operations)

### Problem Statement

Write a function that checks if a string is a palindrome. A palindrome reads the same forwards and backwards.

### Requirements
- Accept a string reference
- Return true if palindrome, false otherwise
- Case-insensitive
- Ignore spaces and punctuation (alphanumeric only)
- Handle single characters and empty strings

### Function Signature
```rust
pub fn is_palindrome(s: &str) -> bool {
    // Your implementation here
}
```

### Test Cases
```
Input: "racecar"
Output: true

Input: "A man, a plan, a canal: Panama"
Output: true

Input: "hello"
Output: false

Input: "Madam"
Output: true

Input: "12321"
Output: true

Input: "a"
Output: true

Input: ""
Output: true
```

### Hints
- Filter to keep only alphanumeric characters
- Convert to lowercase
- Reverse and compare with original
- Or use two-pointer approach (from start and end)

### Algorithm Approach
1. Clean string (lowercase, alphanumeric only)
2. Compare with its reverse
3. Or check from both ends moving inward

### Related Concepts
- String filtering
- Character classification
- Comparison algorithms

---

## Problem 3: Word Frequency Counter

**Difficulty**: Beginner-Intermediate
**Concepts**: HashMap, string splitting
**Prerequisites**: 02-standard-library (Collections, HashMap)

### Problem Statement

Write a function that counts how many times each word appears in a text.

### Requirements
- Accept text as string reference
- Return HashMap with word counts
- Case-insensitive counting
- Remove punctuation from words
- Ignore common English stop words (optional enhancement)

### Function Signature
```rust
use std::collections::HashMap;

pub fn count_word_frequency(text: &str) -> HashMap<String, usize> {
    // Your implementation here
}
```

### Test Cases
```
Input: "hello world hello"
Output: {"hello": 2, "world": 1}

Input: "The quick brown fox jumps over the lazy dog"
Output: {"the": 2, "quick": 1, "brown": 1, "fox": 1, ...}

Input: "rust rust RUST"
Output: {"rust": 3}

Input: "a"
Output: {"a": 1}

Input: ""
Output: {} (empty HashMap)
```

### Hints
- Split by whitespace with `.split_whitespace()`
- Convert each word to lowercase
- Remove punctuation with `.filter()`
- Use HashMap entry API to increment counts
- Or use `.entry().or_insert(0) += 1`

### Algorithm Approach
1. Split text into words
2. For each word:
   - Convert to lowercase
   - Remove punctuation
   - Increment count in HashMap
3. Return HashMap

### Related Concepts
- HashMap usage
- String iteration and splitting
- Entry API and in-place updates

---

## Problem 4: Title Case Formatter

**Difficulty**: Beginner-Intermediate
**Concepts**: String manipulation, word processing
**Prerequisites**: 02-standard-library (String Operations)

### Problem Statement

Write a function that converts a string to title case (first letter of each word capitalized).

### Requirements
- Capitalize first letter of each word
- Lowercase remaining letters
- Handle punctuation (don't capitalize after punctuation)
- Preserve spaces and structure

### Function Signature
```rust
pub fn to_title_case(s: &str) -> String {
    // Your implementation here
}
```

### Test Cases
```
Input: "hello world"
Output: "Hello World"

Input: "the quick brown fox"
Output: "The Quick Brown Fox"

Input: "HELLO WORLD"
Output: "Hello World"

Input: "it's a beautiful day"
Output: "It's A Beautiful Day"

Input: ""
Output: ""

Input: "single"
Output: "Single"
```

### Hints
- Split by spaces
- For each word, capitalize first char, lowercase rest
- Use `.chars()` to access first character
- Rejoin with spaces

### Algorithm Approach
1. Split by space
2. For each word:
   - Get first character and uppercase it
   - Get rest and lowercase it
   - Combine
3. Join back with spaces

### Related Concepts
- Character case manipulation
- String splitting and joining
- Character indexing and slicing

---

## Problem 5: Simple CSV Parser

**Difficulty**: Beginner-Intermediate
**Concepts**: String parsing, collections
**Prerequisites**: 02-standard-library (String Operations, Collections)

### Problem Statement

Write a function that parses a CSV line into fields.

### Requirements
- Parse comma-separated values
- Handle quoted fields (fields containing commas)
- Remove quotes from output
- Trim whitespace
- Return vector of field strings

### Function Signature
```rust
pub fn parse_csv_line(line: &str) -> Vec<String> {
    // Your implementation here
}
```

### Test Cases
```
Input: "John,Doe,30"
Output: ["John", "Doe", "30"]

Input: "John,\"Doe, Jr.\",30"
Output: ["John", "Doe, Jr.", "30"]

Input: "name,age,city"
Output: ["name", "age", "city"]

Input: "\"quoted field\",unquoted"
Output: ["quoted field", "unquoted"]

Input: ""
Output: []

Input: "single"
Output: ["single"]
```

### Hints
- Simple version: just split by comma and trim
- Advanced version: handle quoted fields
- Track whether inside quotes
- When inside quotes, don't split on comma

### Algorithm Approach (Simple)
1. Split by comma
2. Trim whitespace from each field
3. Return vector

Algorithm Approach (Quoted Fields)
1. Iterate through characters
2. Track if inside quotes
3. Build fields based on comma boundaries
4. Remove quotes from fields

### Related Concepts
- String parsing
- State machines (tracking quote state)
- String building

---

## Summary of Beginner Level Problems

These problems practice:
- Intermediate string manipulation
- Working with collections and maps
- String parsing and normalization
- Algorithms like sorting and searching strings

**Recommended Order**: Complete problems 1-5 sequentially
**Time Estimate**: 60-90 minutes total
**Difficulty Progression**: 1→2→3→4→5 (gradual increase)

---

## Prerequisites Before Starting

Before attempting these problems, ensure you understand:
- String types (&str vs String)
- String methods (split, chars, to_lowercase, etc.)
- HashMap usage and entry API
- Iterators and collect
- Basic pattern matching
