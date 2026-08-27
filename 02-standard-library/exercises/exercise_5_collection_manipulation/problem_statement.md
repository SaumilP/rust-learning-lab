# Exercise 5: Collection Manipulation

## Difficulty: Medium
## Concepts Tested: Vectors, HashMaps, Iterators, Collection Methods
## Prerequisites: Module 02 - Collections, Iterator Patterns

---

## Problem Statement

Write a Rust program that demonstrates collection manipulation using Vectors and HashMaps. The program should:

1. Create and modify vectors with various operations
2. Use HashMap for grouping and counting data
3. Chain iterator methods to transform collections
4. Filter, map, and collect results
5. Handle edge cases in collection operations

## Requirements

- Create vectors and perform push, pop, remove, insert operations
- Use iterators to transform vector elements
- Build and query HashMaps efficiently
- Implement a word frequency counter using HashMap
- Group numbers into even/odd categories
- Demonstrate map and filter operations
- Handle duplicate detection in collections
- Show final results in formatted output

## Use Cases

### Use Case 1: Vector Operations
- Create a vector of numbers
- Add and remove elements
- Filter even numbers
- Square the values
- Collect back into a vector

### Use Case 2: Word Frequency Counting
- Take a sentence or text
- Count occurrences of each word
- Find most common words
- Display in frequency order

### Use Case 3: Number Grouping
- Group numbers by even/odd
- Find unique values
- Calculate statistics (sum, average, count)

## Expected Input/Output

### Test Case 1: Vector Operations
```
Original vector: [2, 4, 1, 5, 3, 8]
Length: 6
After push 7: [2, 4, 1, 5, 3, 8, 7]
After pop: [2, 4, 1, 5, 3, 8]
Even numbers: [2, 4, 8]
Even numbers squared: [4, 16, 64]
Doubled all: [4, 8, 2, 10, 6, 16]
```

### Test Case 2: Word Frequency
```
Text: "rust is great rust is fun"
Word counts:
  rust: 2
  is: 2
  great: 1
  fun: 1
```

### Test Case 3: Number Grouping
```
Numbers: [10, 15, 20, 25, 30, 35]
Even: [10, 20, 30] (sum: 60, avg: 20)
Odd: [15, 25, 35] (sum: 75, avg: 25)
```

## Hints

1. **Hint 1**: Use `.iter()` for non-consuming iteration, `.into_iter()` for consuming
2. **Hint 2**: Chain methods: `.filter()`, `.map()`, `.collect()` work together
3. **Hint 3**: Use `.entry()` API for efficient HashMap insertions with counting
4. **Hint 4**: `split_whitespace()` is useful for splitting text into words
5. **Hint 5**: `partition()` can split a collection based on a condition
6. **Hint 6**: The broken code has issues with iteration, collection methods, or type handling

## Testing

Run your program:
```bash
cargo run
```

You should see:
- Vector operations performed correctly
- Word frequencies counted accurately
- Number grouping working properly
- No compile errors about borrowed values or type mismatches

## Learning Objectives

After completing this exercise, you should understand:
- Vector methods and operations
- HashMap insertion and querying
- Iterator chaining and composition
- Collection transformation methods
- Ownership and borrowing with collections
- Grouping and aggregation patterns
- Handling collection results safely

