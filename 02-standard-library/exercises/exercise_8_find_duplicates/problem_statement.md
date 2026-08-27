# Exercise 8: Find Duplicates

## Difficulty: Medium to Hard
## Concepts Tested: HashSet, HashMap, Collections, Algorithms
## Prerequisites: Module 02 - Collections, Iterator Patterns

---

## Problem Statement

Write a Rust program that finds duplicate elements in various collections using efficient algorithms. The program should:

1. Detect duplicate values in collections
2. Find the first occurrence of a duplicate
3. Count duplicate occurrences
4. Identify elements appearing multiple times
5. Use HashSet and HashMap efficiently

## Requirements

- Implement duplicate detection for integers
- Implement duplicate detection for strings
- Find first duplicate by value
- Find first duplicate by position
- Use HashSet for efficient lookups
- Build HashMap to count occurrences
- Return results in clear format
- Handle edge cases (empty collection, single element)

## Algorithms to Implement

### Algorithm 1: Simple Duplicate Detection
- Return true if collection has duplicates
- Return false if all elements unique

### Algorithm 2: Find First Duplicate Value
- Return the first value that appears more than once
- Return None if no duplicates

### Algorithm 3: Find All Duplicates
- Return a collection of all values that appear more than once
- Show count for each duplicate

### Algorithm 4: Position-Based Duplicates
- Find duplicates based on position in string
- Handle character-level duplicates

## Expected Input/Output

### Test Case 1: Integer Duplicates
```
Numbers: [1, 2, 3, 2, 4, 5, 3, 1]
Has duplicates: true
First duplicate value: Some(2)
All duplicates: [1, 2, 3]
Duplicate counts: 1->2, 2->2, 3->2
```

### Test Case 2: String Duplicates
```
Words: ["apple", "banana", "apple", "cherry", "banana"]
Has duplicates: true
First duplicate: Some("apple")
All duplicates: ["apple", "banana"]
Duplicate counts: apple->2, banana->2
```

### Test Case 3: Character Duplicates
```
Text: "programming"
Has duplicates: true
First duplicate char: Some('r')
All duplicates: ['r', 'g', 'm']
Character counts: r->2, g->2, m->2
```

## Hints

1. **Hint 1**: Use HashSet to efficiently track seen values
2. **Hint 2**: To find first duplicate, iterate and track with HashSet
3. **Hint 3**: HashMap can count occurrences of each value
4. **Hint 4**: Use `.insert()` which returns whether value was already present
5. **Hint 5**: `.filter()` with counts > 1 identifies duplicates
6. **Hint 6**: The broken code has issues with HashSet operations or counting logic

## Testing

Run your program:
```bash
cargo run
```

You should see:
- Correct duplicate detection results
- First duplicate values identified
- All duplicates listed with counts
- Proper handling of edge cases

## Learning Objectives

After completing this exercise, you should understand:
- HashSet operations and efficiency
- HashMap for counting and grouping
- Algorithms for duplicate detection
- Trade-offs between different approaches
- Performance implications of collection choices
- How to return collections from functions
- Working with collections of different types

