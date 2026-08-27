# Beginner Level: Number Theory Problems

## Problem 1: GCD and LCM

**Difficulty**: Beginner-Intermediate
**Concepts**: Number theory, Euclidean algorithm
**Prerequisites**: 01-core-fundamentals (Functions, Control Flow)

### Problem Statement

Write functions to calculate the Greatest Common Divisor (GCD) and Least Common Multiple (LCM) of two numbers.

### Requirements
- GCD: Find largest number that divides both
- LCM: Find smallest number divisible by both
- Handle positive integers
- Edge case: if either number is 0, GCD is the non-zero number

### Function Signatures
```rust
pub fn gcd(a: u32, b: u32) -> u32 {
    // Your implementation here
}

pub fn lcm(a: u32, b: u32) -> u32 {
    // Your implementation here (use GCD)
}
```

### Test Cases for GCD
```
Input: gcd(12, 8)
Output: 4

Input: gcd(17, 19)
Output: 1

Input: gcd(100, 50)
Output: 50

Input: gcd(0, 5)
Output: 5

Input: gcd(21, 14)
Output: 7
```

### Test Cases for LCM
```
Input: lcm(4, 6)
Output: 12

Input: lcm(3, 5)
Output: 15

Input: lcm(12, 18)
Output: 36

Input: lcm(1, 5)
Output: 5

Input: lcm(7, 7)
Output: 7
```

### Hints for GCD (Euclidean Algorithm)
- GCD(a, b) = GCD(b, a mod b)
- Base case: GCD(a, 0) = a
- Repeat until b becomes 0

### Hints for LCM
- LCM(a, b) = (a * b) / GCD(a, b)
- First calculate GCD, then apply formula

### Algorithm Approach (GCD)
1. While b ≠ 0:
   - temp = b
   - b = a % b
   - a = temp
2. Return a

### Related Concepts
- Euclidean algorithm
- Modulo operator
- Mathematical number theory

---

## Problem 2: Fibonacci Sequence

**Difficulty**: Beginner-Intermediate
**Concepts**: Recursion/iteration, sequence generation
**Prerequisites**: 01-core-fundamentals (Functions, Control Flow)

### Problem Statement

Generate the Fibonacci sequence up to n terms or up to a maximum value.

### Requirements
- Generate Fibonacci sequence
- Accept count of terms to generate
- Return vector of Fibonacci numbers
- Handle edge cases (0 or 1 terms)
- Use efficient iteration (not exponential recursion)

### Function Signature
```rust
pub fn fibonacci(n: usize) -> Vec<u64> {
    // Your implementation here - return first n Fibonacci numbers
}
```

### Test Cases
```
Input: 0
Output: []

Input: 1
Output: [0]

Input: 2
Output: [0, 1]

Input: 5
Output: [0, 1, 1, 2, 3]

Input: 8
Output: [0, 1, 1, 2, 3, 5, 8, 13]

Input: 10
Output: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

### Hints
- Fibonacci sequence: each number is sum of previous two
- Start with F(0) = 0, F(1) = 1
- Use iteration with two variables to track last two numbers
- Avoid recursion for efficiency

### Algorithm Approach (Iterative)
1. If n == 0, return empty
2. If n == 1, return [0]
3. Initialize result with [0, 1]
4. For i from 2 to n:
   - next = result[i-1] + result[i-2]
   - append to result
5. Return result

### Related Concepts
- Sequence generation
- Iterative vs recursive approaches
- Variable tracking and updates

---

## Problem 3: Roman Numeral Conversion

**Difficulty**: Beginner-Intermediate
**Concepts**: String building, mapping, iteration
**Prerequisites**: 02-standard-library (Collections, HashMap)

### Problem Statement

Write a function to convert integers to Roman numerals.

### Requirements
- Convert positive integers (1-3999) to Roman numerals
- Use correct Roman numeral symbols: I, V, X, L, C, D, M
- Handle subtractive cases (e.g., IV for 4, IX for 9)
- Return as String

### Function Signature
```rust
pub fn int_to_roman(num: u32) -> String {
    // Your implementation here
}
```

### Test Cases
```
Input: 1
Output: "I"

Input: 4
Output: "IV"

Input: 9
Output: "IX"

Input: 27
Output: "XXVII"

Input: 58
Output: "LVIII"

Input: 1994
Output: "MCMXCIV"

Input: 3999
Output: "MMMCMXCIX"
```

### Hints
- Create mapping of value -> Roman symbol
- Include subtractive cases: 4->IV, 9->IX, 40->XL, 90->XC, 400->CD, 900->CM
- Sort by value descending
- Greedily use largest symbols first

### Algorithm Approach
1. Create sorted list of (value, symbol) pairs from largest to smallest
2. For each pair:
   - While number >= value:
     - Add symbol to result
     - Subtract value from number
3. Return result

### Value Mappings
```
1->I, 4->IV, 5->V, 9->IX, 10->X, 40->XL, 50->L, 90->XC,
100->C, 400->CD, 500->D, 900->CM, 1000->M
```

### Related Concepts
- String building and appending
- Value to symbol mapping
- Greedy algorithms

---

## Summary of Beginner Level: Number Theory

These problems practice:
- Classic number theory algorithms
- Sequence generation
- Format conversion
- Mathematical problem solving

**Recommended Order**: Complete problems 1-3 sequentially
**Time Estimate**: 60-90 minutes total
**Difficulty Progression**: 1→2→3 (gradual increase)

---

## Prerequisites Before Starting

Before attempting these problems, ensure you understand:
- Loops and iteration
- Modulo operator and divisibility
- Function definition and recursion basics
- String building
- HashMap and vector operations
