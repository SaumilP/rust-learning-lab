# Kata: Logic Puzzles Problems

## Problem 1: Simple Pattern Generation

**Difficulty**: Beginner
**Concepts**: Loops, strings, printing
**Prerequisites**: 01-core-fundamentals (Control Flow, Functions)

### Problem Statement

Write a function that generates a simple repeating pattern and returns it as a string.

### Requirements
- Accept a character and count
- Return a string with character repeated count times
- Handle zero and negative counts (return empty string)
- Build string efficiently

### Function Signature
```rust
pub fn generate_pattern(ch: char, count: usize) -> String {
    // Your implementation here
}
```

### Test Cases
```
Input: 'x', 5
Output: "xxxxx"

Input: '*', 3
Output: "***"

Input: 'a', 0
Output: ""

Input: '#', 1
Output: "#"

Input: '-', 7
Output: "-------"
```

### Hints
- Use `.repeat()` method on char (convert to string first)
- Or use a loop to build the string
- Or use `std::iter::repeat()`

### Related Concepts
- String building
- Repetition patterns
- Character to string conversion

---

## Problem 2: FizzBuzz

**Difficulty**: Beginner
**Concepts**: Control flow, pattern matching
**Prerequisites**: 01-core-fundamentals (Control Flow, Functions)

### Problem Statement

Classic FizzBuzz: Write a function that prints numbers 1 to n, but:
- For multiples of 3, print "Fizz"
- For multiples of 5, print "Buzz"
- For multiples of both, print "FizzBuzz"

### Requirements
- Accept n as input
- Return vector of strings (or print)
- Handle n from 1 to 100+
- Follow standard FizzBuzz rules

### Function Signature
```rust
pub fn fizzbuzz(n: usize) -> Vec<String> {
    // Your implementation here
}
```

### Test Cases
```
Input: 5
Output: ["1", "2", "Fizz", "4", "Buzz"]

Input: 15
Output: ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz",
         "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"]

Input: 3
Output: ["1", "2", "Fizz"]

Input: 1
Output: ["1"]
```

### Hints
- Use modulo operator % to check divisibility
- Check divisibility by 15 first (both 3 and 5)
- Or nest if statements
- Use match or if-else chains

### Related Concepts
- Modulo operator
- Conditional logic
- String conversion

---

## Problem 3: Number Sequence

**Difficulty**: Beginner
**Concepts**: Pattern recognition, loops
**Prerequisites**: 01-core-fundamentals (Control Flow, Functions)

### Problem Statement

Write a function that generates a sequence following a pattern.

### Requirements
- Identify and continue a number pattern
- Common patterns: arithmetic (add constant), geometric (multiply constant), Fibonacci-like
- Return next N numbers in sequence

### Function Signature
```rust
pub fn continue_sequence(start: i32, difference: i32, count: usize) -> Vec<i32> {
    // Your implementation here - arithmetic sequence
}
```

### Test Cases (Arithmetic Sequence)
```
Input: start=1, difference=2, count=5
Output: [1, 3, 5, 7, 9]

Input: start=10, difference=-3, count=4
Output: [10, 7, 4, 1]

Input: start=0, difference=5, count=3
Output: [0, 5, 10]

Input: start=100, difference=0, count=3
Output: [100, 100, 100]
```

### Hints
- Next number = current + difference
- Use a loop to generate count numbers
- Or use iterator and collect
- Start with first number in sequence

### Related Concepts
- Iterative generation
- Accumulator patterns
- Sequence definition

---

## Problem 4: Boolean Logic

**Difficulty**: Beginner
**Concepts**: Boolean operations, logic
**Prerequisites**: 01-core-fundamentals (Control Flow, Data Types)

### Problem Statement

Write functions that solve simple boolean logic puzzles.

### Requirements
- Evaluate boolean expressions
- Implement basic logic gates (AND, OR, NOT, XOR)
- Return boolean results

### Function Signature
```rust
pub fn logical_and(a: bool, b: bool) -> bool { /* ... */ }
pub fn logical_or(a: bool, b: bool) -> bool { /* ... */ }
pub fn logical_not(a: bool) -> bool { /* ... */ }
pub fn logical_xor(a: bool, b: bool) -> bool { /* ... */ }
```

### Test Cases
```
AND: (true, true) -> true, (true, false) -> false, (false, false) -> false
OR:  (true, false) -> true, (false, false) -> false, (true, true) -> true
NOT: (true) -> false, (false) -> true
XOR: (true, false) -> true, (true, true) -> false, (false, false) -> false
```

### Hints
- AND: both must be true
- OR: at least one must be true
- NOT: opposite value
- XOR: exactly one must be true (not both)

### Related Concepts
- Boolean types
- Logical operators
- Truth tables

---

## Problem 5: Simple Sort

**Difficulty**: Beginner
**Concepts**: Sorting, comparison
**Prerequisites**: 01-core-fundamentals (Functions, Control Flow)

### Problem Statement

Write a function that sorts a small array of numbers in ascending order.

### Requirements
- Sort integers from smallest to largest
- Use a simple sorting algorithm (bubble sort, insertion sort, etc.)
- Handle arrays up to 10 elements
- Preserve duplicates

### Function Signature
```rust
pub fn simple_sort(arr: &[i32]) -> Vec<i32> {
    // Your implementation here
}
```

### Test Cases
```
Input: &[3, 1, 4, 1, 5]
Output: [1, 1, 3, 4, 5]

Input: &[5, 4, 3, 2, 1]
Output: [1, 2, 3, 4, 5]

Input: &[1, 2, 3]
Output: [1, 2, 3]

Input: &[]
Output: []

Input: &[42]
Output: [42]
```

### Hints
- Bubble sort: repeatedly swap adjacent elements
- Insertion sort: build sorted array one element at a time
- Or use built-in `.sort()` method
- Compare with `<` operator

### Related Concepts
- Sorting algorithms
- Comparison operators
- Looping with swaps

---

## Summary of Kata: Logic Puzzles

These problems practice:
- Control flow patterns
- Loop structures
- Boolean logic
- Pattern generation and recognition
- Basic algorithms

**Recommended Order**: Complete problems 1-5 sequentially
**Time Estimate**: 30-45 minutes total
**Difficulty Progression**: 1→2→3→4→5 (slight increase)
