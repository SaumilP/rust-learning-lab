# Exercise 6: Iterator Chains and Transformations

## Difficulty: Medium
## Concepts Tested: Iterators, Method Chaining, Functional Programming, Lazy Evaluation
## Prerequisites: Module 02 - Iterator Patterns, Collections

---

## Problem Statement

Write a Rust program that demonstrates complex iterator chains and transformations. The program should:

1. Chain multiple iterator methods together
2. Perform complex data transformations
3. Demonstrate lazy evaluation
4. Use functional programming patterns
5. Build comprehensive pipelines for data processing

## Requirements

- Use filter, map, skip, take methods in chains
- Demonstrate enumerate for indexed iteration
- Use find, fold, and reduce operations
- Chain complex transformations
- Process data through multiple stages
- Show intermediate and final results
- Demonstrate performance benefit of lazy evaluation

## Transformations to Implement

### Transformation 1: Number Processing Pipeline
- Start with range of numbers
- Filter even numbers
- Map to squares
- Skip first 2 results
- Take 3 results
- Collect into vector

### Transformation 2: String Processing
- Split text into words
- Filter by length
- Map to uppercase
- Enumerate to get positions
- Join back into string

### Transformation 3: Complex Chain
- Process collection through multiple stages
- Use fold to aggregate results
- Calculate statistics from filtered data

## Expected Input/Output

### Test Case 1: Number Pipeline
```
Original: 1..=20
Evens: [2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
Evens squared: [4, 16, 36, 64, 100, 144, 196, 256, 324, 400]
Skip 2, take 3: [36, 64, 100]
Sum of result: 200
Product of result: 230400
```

### Test Case 2: String Processing
```
Original: "the quick brown fox jumps"
Words with length > 3: ["quick", "brown", "jumps"]
Uppercase: ["QUICK", "BROWN", "JUMPS"]
Enumerated: [(0, "QUICK"), (1, "BROWN"), (2, "JUMPS")]
Joined: "QUICK-BROWN-JUMPS"
```

### Test Case 3: Statistics
```
Numbers: [5, 12, 8, 15, 3, 20, 7, 9]
Greater than 5: [12, 8, 15, 20, 7, 9]
Count: 6
Sum: 71
Average: 11.83
```

## Hints

1. **Hint 1**: Iterator chains are lazy - nothing happens until you collect or consume
2. **Hint 2**: Use `.enumerate()` to get (index, value) pairs
3. **Hint 3**: `.fold(initial, |acc, val| ...)` accumulates values
4. **Hint 4**: `.collect()` is the terminal operation that forces evaluation
5. **Hint 5**: `.sum()` and `.product()` are consuming operations
6. **Hint 6**: The broken code has issues with chaining, type conversion, or consumption

## Testing

Run your program:
```bash
cargo run
```

You should see:
- Three complete transformation examples
- Correct intermediate and final results
- No compile errors about type mismatches or borrowing
- Demonstration of chaining efficiency

## Learning Objectives

After completing this exercise, you should understand:
- Iterator chaining and composition
- Method ordering in iterator chains
- Lazy vs eager evaluation
- Terminal operations that consume iterators
- Functional programming patterns in Rust
- Fold and reduce for aggregation
- Enumerate for indexed iteration
- Type inference with complex chains

