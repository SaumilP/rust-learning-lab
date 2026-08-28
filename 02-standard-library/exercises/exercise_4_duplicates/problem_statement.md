# Exercise 4: Find Duplicates in Vector

## Difficulty: Medium
## Concepts Tested: Collections, HashSet, Functions, Iterators
## Prerequisites: Module 02 - Collections, HashSet/HashMap

---

## Problem Statement

Write a Rust program that finds and reports duplicate values in collections. This exercise demonstrates efficient use of HashSet and HashMap for duplicate detection, a common real-world programming task.

## Requirements

- Detect duplicate elements in a collection
- Report first occurrence and all duplicates
- Count occurrences of each element
- Group duplicates together
- Return results as structured data
- Handle edge cases (empty collection, single element, no duplicates)

## Tasks

### Task 1: Simple Duplicate Detection
- Return `true` if collection has any duplicates
- Return `false` if all elements are unique
- Use HashSet for O(n) performance

### Task 2: Find First Duplicate
- Return the first value that appears more than once
- Return `None` if no duplicates exist
- Order matters - first duplicate by position in collection

### Task 3: Find All Duplicates with Counts
- Return all values that appear more than once
- Include count of occurrences for each duplicate
- Use HashMap for counting

### Task 4: Character Duplicates in Strings
- Find duplicate characters in a string
- Report which characters repeat and how many times

## Expected Output

```
=== Integer Duplicates ===
Numbers: [1, 2, 3, 2, 4, 5, 3, 1]
Has duplicates: true
First duplicate: Some(2)
All duplicates with counts:
  1 appears 2 times
  2 appears 2 times
  3 appears 2 times
Unique values: [1, 2, 3, 4, 5]

=== No Duplicates Case ===
Numbers: [1, 2, 3, 4, 5]
Has duplicates: false
First duplicate: None

=== String/Word Duplicates ===
Words: ["apple", "banana", "apple", "cherry", "banana", "apple"]
Has duplicates: true
First duplicate: Some("apple")
Duplicate counts:
  "apple" appears 3 times
  "banana" appears 2 times

=== Character Duplicates ===
Text: "programming"
Has duplicate chars: true
First duplicate char: Some('r')
Character counts for duplicates:
  'r' appears 2 times
  'g' appears 2 times
  'm' appears 2 times
```

## Notes

- Use `HashSet::insert()` which returns `false` if element already exists
- HashMap's entry API is efficient for counting
- Consider using `chars()` for character iteration
- Think about time complexity - HashSet operations are O(1) average
