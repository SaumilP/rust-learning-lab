# Kata: Number Operations Problems

## Problem 1: Sum Array

**Difficulty**: Beginner
**Concepts**: Iteration, arithmetic
**Prerequisites**: 01-core-fundamentals (Functions, Control Flow)

### Problem Statement

Write a function that calculates the sum of all integers in an array.

### Requirements
- Accept a slice of integers
- Return the sum as an integer
- Handle empty arrays (return 0)
- Handle negative numbers

### Function Signature
```rust
pub fn sum_array(arr: &[i32]) -> i32 {
    // Your implementation here
}
```

### Test Cases
```
Input: &[1, 2, 3, 4, 5]
Output: 15

Input: &[10, -5, 20, -3]
Output: 22

Input: &[0, 0, 0]
Output: 0

Input: &[]
Output: 0

Input: &[-1, -2, -3]
Output: -6
```

### Hints
- Use `.iter()` and `.sum()` for concise solution
- Or use a loop with accumulator variable
- Consider using `fold()` for a functional approach

### Related Concepts
- Array/slice iteration
- Accumulator pattern
- Iterator methods

---

## Problem 2: Calculate Average

**Difficulty**: Beginner
**Concepts**: Arithmetic, division, handling empty cases
**Prerequisites**: 01-core-fundamentals (Functions, Data Types)

### Problem Statement

Write a function that calculates the average of numbers in an array.

### Requirements
- Accept a slice of numbers (f64)
- Return average as f64
- Handle empty arrays (return 0.0)
- Return exact average

### Function Signature
```rust
pub fn calculate_average(arr: &[f64]) -> f64 {
    // Your implementation here
}
```

### Test Cases
```
Input: &[1.0, 2.0, 3.0, 4.0, 5.0]
Output: 3.0

Input: &[10.5, 20.5, 30.0]
Output: 20.333...

Input: &[5.0]
Output: 5.0

Input: &[]
Output: 0.0

Input: &[0.0, 0.0, 0.0]
Output: 0.0
```

### Hints
- Sum all elements then divide by count
- Use `arr.len()` to get array length
- Handle empty array before division
- Remember: sum / length as f64

### Related Concepts
- Type casting
- Floating-point arithmetic
- Edge case handling

---

## Problem 3: Check Prime Number

**Difficulty**: Beginner
**Concepts**: Conditional logic, loops
**Prerequisites**: 01-core-fundamentals (Control Flow, Functions)

### Problem Statement

Write a function that determines if a number is prime.

### Requirements
- Accept an integer
- Return true if prime, false otherwise
- A prime number is only divisible by 1 and itself
- Consider edge cases (numbers ≤ 1)

### Function Signature
```rust
pub fn is_prime(n: i32) -> bool {
    // Your implementation here
}
```

### Test Cases
```
Input: 2
Output: true

Input: 17
Output: true

Input: 1
Output: false

Input: 0
Output: false

Input: -5
Output: false

Input: 100
Output: false

Input: 97
Output: true
```

### Hints
- Numbers ≤ 1 are not prime
- Check divisibility from 2 to sqrt(n)
- If any number divides n, it's not prime
- Use `i * i <= n` instead of computing sqrt

### Related Concepts
- Loop optimization
- Conditional logic
- Mathematical algorithms

---

## Problem 4: Calculate Factorial

**Difficulty**: Beginner
**Concepts**: Loops, multiplication, edge cases
**Prerequisites**: 01-core-fundamentals (Control Flow, Functions)

### Problem Statement

Write a function that calculates the factorial of a number.

### Requirements
- Accept a non-negative integer
- Return factorial as u64
- 0! = 1
- Handle large factorials (up to 20!)

### Function Signature
```rust
pub fn factorial(n: u32) -> u64 {
    // Your implementation here
}
```

### Test Cases
```
Input: 0
Output: 1

Input: 1
Output: 1

Input: 5
Output: 120

Input: 10
Output: 3628800

Input: 3
Output: 6

Input: 20
Output: 2432902008176640000
```

### Hints
- Factorial of n = n × (n-1) × (n-2) × ... × 1
- Base case: 0! and 1! are both 1
- Use a loop or recursion
- Use u64 to avoid overflow

### Related Concepts
- Loops with accumulation
- Mathematical definition implementation
- Type selection for large numbers

---

## Problem 5: Find Maximum

**Difficulty**: Beginner
**Concepts**: Comparison, iteration
**Prerequisites**: 01-core-fundamentals (Functions, Control Flow)

### Problem Statement

Write a function that finds the maximum value in an array.

### Requirements
- Accept a slice of integers
- Return the maximum value
- Array is guaranteed to have at least one element
- Handle negative numbers

### Function Signature
```rust
pub fn find_maximum(arr: &[i32]) -> i32 {
    // Your implementation here
}
```

### Test Cases
```
Input: &[3, 1, 4, 1, 5, 9]
Output: 9

Input: &[-10, -5, -20]
Output: -5

Input: &[42]
Output: 42

Input: &[0, 0, 0]
Output: 0

Input: &[100, 50, 75, 25]
Output: 100
```

### Hints
- Use `.max()` iterator method for simple solution
- Or manually iterate with comparison
- Keep track of maximum seen so far
- Initialize with first array element

### Related Concepts
- Iterator methods
- Comparison operators
- Accumulator pattern

---

## Summary of Kata: Number Operations

These problems practice:
- Basic arithmetic operations
- Loop constructs and iteration
- Conditional logic
- Type selection and casting

**Recommended Order**: Complete problems 1-5 sequentially
**Time Estimate**: 30-45 minutes total
**Difficulty Progression**: 1→2→3→4→5 (slight increase)
