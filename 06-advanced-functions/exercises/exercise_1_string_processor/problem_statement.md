# Exercise 1: String Text Processor with Traits

## Difficulty: Medium
## Concepts Tested: Ownership, Borrowing, Traits
## Prerequisites: Module 06 concepts

## Problem Statement

Create a text processor that:

1. Defines a `StringProcessor` trait with methods for text operations
2. Implements trait for different processing types (Uppercase, Lowercase, ReverseWords, CharCount)
3. Accepts text through borrowing
4. Returns processed results

## Requirements

- Define trait with process() method taking &str, returning String
- Implement for 4 different processor types
- Support processing text without taking ownership
- Display statistics about original and processed text

## Expected Output

```
Original text: "Hello World from Rust"
Length: 21 characters

Uppercase processor:
Result: HELLO WORLD FROM RUST

Lowercase processor:
Result: hello world from rust

Reverse words processor:
Result: Rust from World Hello

Character counter processor:
Result: Contains 16 letters and 4 spaces
```

## Hints

1. **Trait Definition**: Method signature should use &str for borrowed input
2. **Borrowing**: Use references to avoid ownership transfer
3. **Multiple Implementations**: Implement same trait for different types
4. **String Manipulation**: Use split, collect, reverse, chars methods
5. **Statistics**: Calculate before and after metrics

## Solution Concepts

- Trait definition and implementation
- Borrowing patterns
- String operations
- Ownership semantics

