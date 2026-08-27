# Kata Level Challenges (Beginner)

These are simple, focused problems perfect for warming up and practicing specific concepts. Each problem should take 5-15 minutes and requires only a single function (5-20 lines).

## String Manipulation (5 Problems)

### 1. Reverse String
**Task**: Return the input string in reverse order.

**Example**:
```
Input: "hello"
Output: "olleh"
```

**Concepts**: String iteration, reversal

**Solution Approach**: Use `chars().rev().collect()` or iterate backwards

---

### 2. Count Vowels
**Task**: Count the number of vowels in a string (a, e, i, o, u).

**Example**:
```
Input: "hello world"
Output: 3
```

**Concepts**: String iteration, character checking

**Solution Approach**: Iterate through chars, check if vowel, count

---

### 3. Capitalize Words
**Task**: Capitalize the first letter of each word.

**Example**:
```
Input: "hello world"
Output: "Hello World"
```

**Concepts**: String manipulation, iteration

**Solution Approach**: Split by spaces, capitalize first char, join

---

### 4. Remove Whitespace
**Task**: Remove all whitespace characters from a string.

**Example**:
```
Input: "hello   world  test"
Output: "helloworldtest"
```

**Concepts**: String filtering, characters

**Solution Approach**: Filter out chars where `is_whitespace()`

---

### 5. Convert to Snake Case
**Task**: Convert a string to snake_case (lowercase with underscores between words).

**Example**:
```
Input: "HelloWorld Test"
Output: "hello_world_test"
```

**Concepts**: String transformation, case conversion

**Solution Approach**: Lowercase, handle spaces and case transitions

---

## Number Operations (5 Problems)

### 6. Sum Array
**Task**: Return the sum of all numbers in an array.

**Example**:
```
Input: [1, 2, 3, 4, 5]
Output: 15
```

**Concepts**: Iteration, addition

**Solution Approach**: Use `iter().sum()`

---

### 7. Calculate Average
**Task**: Return the average of numbers in an array.

**Example**:
```
Input: [10, 20, 30]
Output: 20.0
```

**Concepts**: Math, collections

**Solution Approach**: Sum divided by length

---

### 8. Check if Prime
**Task**: Return true if a number is prime.

**Example**:
```
Input: 7
Output: true

Input: 9
Output: false
```

**Concepts**: Math, loops, conditionals

**Solution Approach**: Check divisibility from 2 to sqrt(n)

---

### 9. Calculate Factorial
**Task**: Return n! (n factorial).

**Example**:
```
Input: 5
Output: 120
```

**Concepts**: Recursion or loops, math

**Solution Approach**: Multiply 1×2×3×...×n

---

### 10. Find Maximum
**Task**: Return the largest number in an array.

**Example**:
```
Input: [3, 1, 4, 1, 5, 9]
Output: 9
```

**Concepts**: Iteration, comparison

**Solution Approach**: Use `iter().max()`

---

## Collections (5 Problems)

### 11. Find First Duplicate
**Task**: Return the first duplicate element in an array.

**Example**:
```
Input: [1, 2, 3, 2, 4]
Output: Some(2)
```

**Concepts**: Collections, sets

**Solution Approach**: Use HashSet to track seen elements

---

### 12. Remove Duplicates
**Task**: Return an array without duplicates.

**Example**:
```
Input: [1, 2, 2, 3, 3, 3]
Output: [1, 2, 3]
```

**Concepts**: Collections, uniqueness

**Solution Approach**: Convert to HashSet then back to Vec

---

### 13. Merge Two Arrays
**Task**: Combine two arrays into one.

**Example**:
```
Input: [1, 2], [3, 4]
Output: [1, 2, 3, 4]
```

**Concepts**: Collections, concatenation

**Solution Approach**: Use extend() or chain iterators

---

### 14. Filter Numbers
**Task**: Keep only numbers from a mixed collection.

**Example**:
```
Input: [1, "a", 2, "b", 3]
Output: [1, 2, 3]
```

**Concepts**: Filtering, type handling

**Solution Approach**: Pattern match to filter numbers

---

### 15. Flatten One Level
**Task**: Flatten a nested array one level.

**Example**:
```
Input: [[1, 2], [3, 4], [5]]
Output: [1, 2, 3, 4, 5]
```

**Concepts**: Nested collections, flattening

**Solution Approach**: Use `flat_map()` or `flatten()`

---

## Logic Puzzles (5 Problems)

### 16. Generate Pattern
**Task**: Generate a simple pattern based on input.

**Example**:
```
Input: 3
Output: "***\n**\n*"
```

**Concepts**: Loops, string building

**Solution Approach**: Nested loops to build pattern

---

### 17. FizzBuzz
**Task**: Classic FizzBuzz (1-n: "Fizz" if divisible by 3, "Buzz" if by 5, "FizzBuzz" if both, else number).

**Example**:
```
Input: 15
Output: "1,2,Fizz,4,Buzz,Fizz,7,8,Fizz,Buzz,11,Fizz,13,14,FizzBuzz"
```

**Concepts**: Conditionals, iteration, string formatting

---

### 18. Number Sequence
**Task**: Identify and continue a numeric pattern.

**Example**:
```
Input: [1, 2, 4, 8]
Output: [1, 2, 4, 8, 16]
```

**Concepts**: Pattern recognition, math

**Solution Approach**: Identify the rule (doubling, adding, etc.)

---

### 19. Boolean Logic
**Task**: Simplify logical expressions.

**Example**: Evaluate truth tables

**Concepts**: Boolean operations, logic

---

### 20. Simple Sort
**Task**: Sort a small array of numbers.

**Example**:
```
Input: [3, 1, 4, 1, 5]
Output: [1, 1, 3, 4, 5]
```

**Concepts**: Sorting, comparison

**Solution Approach**: Use sort() or sorting algorithm

---

## How to Use These Challenges

1. **Read the problem** - Understand what's needed
2. **Write the function** - Implement in 5-15 minutes
3. **Test with examples** - Verify against provided cases
4. **Check solution** - Reference solution provided below

## Tips

- Start with string manipulation challenges
- Progress to number operations
- Collections challenges teach iteration patterns
- Logic puzzles combine multiple concepts

