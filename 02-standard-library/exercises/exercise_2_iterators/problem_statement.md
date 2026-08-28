# Exercise 2: Iterator Chain Puzzle

## Difficulty: Medium
## Concepts Tested: Iterators, map, filter, fold, collect
## Prerequisites: Module 02 - Iterator Patterns

---

## Problem Statement

Write a Rust program that demonstrates complex iterator chains and transformations. Iterators in Rust are lazy and can be chained together to create powerful data processing pipelines. This exercise will help you master iterator composition.

## Requirements

- Filter vector elements by condition
- Map elements to new values
- Collect results into new collections
- Chain multiple iterator operations
- Use fold/reduce for aggregation
- Demonstrate enumerate for indexed iteration

## Tasks

### Task 1: Number Processing Pipeline
- Start with a range of numbers (1 to 20)
- Filter to keep only even numbers
- Square each number
- Skip the first 2 results
- Take the next 3 results
- Calculate sum and product of final results

### Task 2: String Word Processing
- Split a sentence into words
- Filter words longer than 3 characters
- Convert to uppercase
- Join back with a custom separator

### Task 3: Data Aggregation
- Use fold to calculate running totals
- Implement custom aggregation logic
- Find elements meeting complex criteria

## Expected Output

```
=== Number Processing Pipeline ===
Range: 1..=20
Even numbers: [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
Squared: [4, 16, 36, 64, 100, 144, 196, 256, 324, 400]
Skip 2, take 3: [36, 64, 100]
Sum: 200
Product: 230400

=== String Processing ===
Original: "the quick brown fox jumps over the lazy dog"
Words > 3 chars: ["quick", "brown", "jumps", "over", "lazy"]
Uppercase: ["QUICK", "BROWN", "JUMPS", "OVER", "LAZY"]
Joined: "QUICK-BROWN-JUMPS-OVER-LAZY"

=== Data Aggregation ===
Numbers: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
Running sum using fold: 55
Running product using fold: 3628800
Count of odds: 5
Sum of squares of evens: 220
```

## Notes

- Iterator chains are lazy - nothing happens until consumed
- Use `.collect()` to materialize results
- `.fold()` is the most flexible aggregation method
- Chain order matters for efficiency
- Type annotations may be needed for complex chains
