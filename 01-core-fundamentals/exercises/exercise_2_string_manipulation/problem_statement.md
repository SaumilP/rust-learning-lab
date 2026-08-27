# Exercise 2: String Manipulation Challenge

## Difficulty: Easy
## Concepts Tested: String Operations, Ownership, Iterators, Methods
## Prerequisites: Module 01 - Data Types, Variables

---

## Problem Statement

Write a Rust program that manipulates strings in various ways to demonstrate string handling, ownership, and common string methods. The program should:

1. Accept string input or use hardcoded test strings
2. Perform multiple transformations on strings (uppercase, lowercase, reverse, remove spaces)
3. Count occurrences of characters
4. Extract substrings or segments
5. Combine strings using different methods
6. Demonstrate the difference between String and &str

## Requirements

- Work with both `String` and `&str` types
- Use string methods: `to_uppercase()`, `to_lowercase()`, `chars()`, `split()`, `trim()`, `push()`, `push_str()`
- Perform character iteration and counting
- Handle string slicing safely
- Demonstrate string concatenation vs string building
- Show final results in multiple formats
- Handle at least 3 different input strings

## Expected Input/Output

### Test Case 1:
```
Input string: "Hello Rust"
Output:
Original: "Hello Rust"
Uppercase: "HELLO RUST"
Lowercase: "hello rust"
Length: 10
Reversed: "tsuR olleH"
Without spaces: "HelloRust"
Character count (l): 3
Words: ["Hello", "Rust"]
```

### Test Case 2:
```
Input string: "   Rust Programming   "
Output:
Original: "   Rust Programming   "
Trimmed: "Rust Programming"
Uppercase: "RUST PROGRAMMING"
Length (trimmed): 16
Reversed: "gnimmargorP tsuR"
Without spaces: "RustProgramming"
Character count (r): 4
Words: ["Rust", "Programming"]
```

### Test Case 3:
```
Input string: "The quick brown fox"
Output:
Original: "The quick brown fox"
Uppercase: "THE QUICK BROWN FOX"
Lowercase: "the quick brown fox"
Length: 19
Reversed: "xof nworb kciuq ehT"
Without spaces: "Thequickbrownfox"
Character count (o): 4
Words: ["The", "quick", "brown", "fox"]
```

## Hints

1. **Hint 1**: Use `.to_uppercase()` and `.to_lowercase()` methods for case conversion
2. **Hint 2**: Use `.chars()` to iterate through characters, `.rev()` for reversal
3. **Hint 3**: Use `.split()` with a delimiter to break strings into parts
4. **Hint 4**: Use `.filter()` to remove characters (e.g., spaces)
5. **Hint 5**: The `&str` slice type is immutable; convert to `String` for mutations
6. **Hint 6**: Use `.len()` for byte count and `.chars().count()` for character count
7. **Hint 7**: The broken code has issues with ownership, method chaining, and type mismatches

## Testing

Run your program with test strings:
```bash
cargo run
# Try: "Hello Rust", "   Rust Programming   ", "The quick brown fox"
```

Expected behavior:
- Accepts string input
- Performs all transformations without errors
- Displays all required formats
- No compile errors about borrowed values or type mismatches

## Learning Objectives

After completing this exercise, you should understand:
- The difference between `String` and `&str`
- How to use common string methods
- Character iteration and counting
- String slicing and indexing (safely)
- String concatenation and building
- Ownership rules with string data
- Method chaining with strings
- Converting between &str and String types

