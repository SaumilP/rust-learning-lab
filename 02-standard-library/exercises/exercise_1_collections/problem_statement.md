# Exercise 1: Collection Manipulation

## Difficulty: Easy
## Concepts Tested: Vec, HashMap, Iteration, Functions
## Prerequisites: Module 02 - Collections, Basic Rust

---

## Problem Statement

Write a Rust program that demonstrates manipulation of Rust's core collection types: Vectors and HashMaps. The program should create, modify, query, and transform collections while calculating basic statistics.

## Requirements

- Create and modify vectors (insert, remove, filter)
- Create and query a HashMap (word frequency counter)
- Find specific elements in collections
- Calculate statistics (sum, average, count)
- Use proper iterator methods for transformations
- Handle edge cases appropriately

## Tasks

### Task 1: Vector Operations
- Create a vector of integers
- Push and pop elements
- Remove element at specific index
- Filter elements by condition
- Find min and max values

### Task 2: Word Frequency Counter
- Take a sentence as input
- Count occurrences of each word using HashMap
- Find the most common word
- List all words with their counts

### Task 3: Statistics Calculator
- Calculate sum of vector elements
- Calculate average
- Count elements matching a condition
- Find elements greater than average

## Expected Output

```
=== Vector Operations ===
Original: [5, 2, 8, 1, 9, 3, 7]
After push 4: [5, 2, 8, 1, 9, 3, 7, 4]
After pop: [5, 2, 8, 1, 9, 3, 7]
After remove index 2: [5, 2, 1, 9, 3, 7]
Even numbers: [2]
Min: 1, Max: 9

=== Word Frequency ===
Text: "the quick brown fox jumps over the lazy dog the"
Word counts:
  the: 3
  quick: 1
  brown: 1
  fox: 1
  jumps: 1
  over: 1
  lazy: 1
  dog: 1
Most common word: "the" (3 times)

=== Statistics ===
Numbers: [10, 20, 30, 40, 50]
Sum: 150
Average: 30.0
Count > 25: 3
Above average: [30, 40, 50]
```

## Notes

- Use `vec![]` macro for vector creation
- HashMap requires `use std::collections::HashMap;`
- Iterator methods like `.iter()`, `.filter()`, `.map()` are essential
- Remember that `.collect()` is needed to materialize iterator results
