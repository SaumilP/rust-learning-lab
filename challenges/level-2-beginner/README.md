# Level 2: Beginner (4-5 kyu) Challenges

Welcome to Level 2 Beginner Challenges! These problems are designed for developers who have mastered basic syntax and are ready to tackle more complex algorithmic problems. Each challenge requires 20-40 lines of code and tests 2 fundamental Rust concepts.

**Difficulty**: 4-5 kyu (Beginner to Lower Intermediate)
**Expected Time**: 20-45 minutes per problem
**Code Length**: 20-40 lines

---

## Table of Contents
1. [Intermediate Strings](#1-intermediate-strings)
2. [Array Algorithms](#2-array-algorithms)
3. [Number Theory](#3-number-theory)
4. [Data Structure Basics](#4-data-structure-basics)

---

## 1. Intermediate Strings

### 1.1 Anagram Checker

**Difficulty**: 5 kyu
**Concepts**: Strings, Sorting, Comparison
**Prerequisites**: String manipulation, char iteration, sorting

#### Problem Statement
Create a function that determines if two words are anagrams of each other. An anagram is a word formed by rearranging the letters of another word, using all original letters exactly once.

#### Signature
```rust
fn are_anagrams(word1: &str, word2: &str) -> bool
```

#### Input/Output Specification
- **Input**: Two string slices (case-insensitive)
- **Output**: `true` if the words are anagrams, `false` otherwise
- **Edge Cases**:
  - Empty strings are considered anagrams of each other
  - Spaces and punctuation should be ignored
  - Case should be ignored ("Listen" and "Silent" are anagrams)

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_anagrams() {
        assert_eq!(are_anagrams("listen", "silent"), true);
        assert_eq!(are_anagrams("hello", "world"), false);
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(are_anagrams("Listen", "Silent"), true);
        assert_eq!(are_anagrams("TRIANGLE", "integral"), true);
    }

    #[test]
    fn test_with_spaces() {
        assert_eq!(are_anagrams("conversation", "voices rant on"), true);
        assert_eq!(are_anagrams("a gentleman", "elegant man"), true);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(are_anagrams("", ""), true);
        assert_eq!(are_anagrams("a", "a"), true);
        assert_eq!(are_anagrams("ab", "ba"), true);
    }
}
```

#### Hints
1. Convert both strings to lowercase
2. Filter out non-alphabetic characters
3. Sort the characters and compare
4. Alternative: Use character frequency counting with HashMap

---

### 1.2 Palindrome Validator

**Difficulty**: 5 kyu
**Concepts**: Strings, Iterators, Comparison
**Prerequisites**: String manipulation, iterator methods, character filtering

#### Problem Statement
Create a function that checks if a given string is a palindrome. A palindrome reads the same forwards and backwards, ignoring spaces, punctuation, and case.

#### Signature
```rust
fn is_palindrome(s: &str) -> bool
```

#### Input/Output Specification
- **Input**: A string slice
- **Output**: `true` if the string is a palindrome, `false` otherwise
- **Edge Cases**:
  - Empty string is considered a palindrome
  - Single character is a palindrome
  - Ignore spaces, punctuation, and case
  - Handle Unicode characters properly

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_palindromes() {
        assert_eq!(is_palindrome("racecar"), true);
        assert_eq!(is_palindrome("hello"), false);
    }

    #[test]
    fn test_with_spaces_and_punctuation() {
        assert_eq!(is_palindrome("A man, a plan, a canal: Panama"), true);
        assert_eq!(is_palindrome("race a car"), false);
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(is_palindrome("RaceCar"), true);
        assert_eq!(is_palindrome("Noon"), true);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(is_palindrome(""), true);
        assert_eq!(is_palindrome("a"), true);
        assert_eq!(is_palindrome(" "), true);
    }

    #[test]
    fn test_longer_palindromes() {
        assert_eq!(is_palindrome("Was it a car or a cat I saw?"), true);
        assert_eq!(is_palindrome("No 'x' in Nixon"), true);
    }
}
```

#### Hints
1. Filter out non-alphanumeric characters
2. Convert to lowercase
3. Compare the string with its reverse
4. Use iterators efficiently with `chars()`, `rev()`, and `filter()`

---

### 1.3 Word Frequency Counter

**Difficulty**: 4 kyu
**Concepts**: Collections (HashMap), Strings, Iteration
**Prerequisites**: HashMap usage, string splitting, iteration

#### Problem Statement
Create a function that counts the frequency of each word in a text. Words should be case-insensitive, and punctuation should be removed. Return the results as a HashMap.

#### Signature
```rust
use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize>
```

#### Input/Output Specification
- **Input**: A text string containing words
- **Output**: HashMap with words as keys and their frequencies as values
- **Edge Cases**:
  - Empty string returns empty HashMap
  - Punctuation should be stripped from words
  - Case-insensitive counting (all keys lowercase)
  - Multiple spaces treated as single delimiter

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_counting() {
        let text = "hello world hello";
        let freq = word_frequency(text);
        assert_eq!(freq.get("hello"), Some(&2));
        assert_eq!(freq.get("world"), Some(&1));
    }

    #[test]
    fn test_case_insensitive() {
        let text = "Hello hello HELLO";
        let freq = word_frequency(text);
        assert_eq!(freq.get("hello"), Some(&3));
    }

    #[test]
    fn test_with_punctuation() {
        let text = "Hello, world! Hello world.";
        let freq = word_frequency(text);
        assert_eq!(freq.get("hello"), Some(&2));
        assert_eq!(freq.get("world"), Some(&2));
    }

    #[test]
    fn test_empty_string() {
        let text = "";
        let freq = word_frequency(text);
        assert_eq!(freq.len(), 0);
    }

    #[test]
    fn test_longer_text() {
        let text = "The quick brown fox jumps over the lazy dog. The dog was not amused.";
        let freq = word_frequency(text);
        assert_eq!(freq.get("the"), Some(&3));
        assert_eq!(freq.get("dog"), Some(&2));
        assert_eq!(freq.get("quick"), Some(&1));
    }
}
```

#### Hints
1. Split the text into words using `split_whitespace()`
2. Strip punctuation from each word
3. Convert to lowercase
4. Use HashMap's `entry()` API for counting
5. Consider using `or_insert()` for initialization

---

## 2. Array Algorithms

### 2.1 Two Sum Problem

**Difficulty**: 4 kyu
**Concepts**: Arrays, Iteration, HashMap
**Prerequisites**: Array/Vec manipulation, HashMap usage, tuple returns

#### Problem Statement
Given an array of integers and a target sum, find the indices of two numbers that add up to the target. You may assume that each input has exactly one solution, and you cannot use the same element twice.

#### Signature
```rust
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)>
```

#### Input/Output Specification
- **Input**:
  - `nums`: Slice of integers
  - `target`: Target sum
- **Output**: `Option<(usize, usize)>` containing indices, or `None` if no solution
- **Edge Cases**:
  - Return `None` if array has fewer than 2 elements
  - Return the first valid pair of indices
  - Indices should be in ascending order (i < j)

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_two_sum() {
        let nums = vec![2, 7, 11, 15];
        assert_eq!(two_sum(&nums, 9), Some((0, 1)));
    }

    #[test]
    fn test_different_positions() {
        let nums = vec![3, 2, 4];
        assert_eq!(two_sum(&nums, 6), Some((1, 2)));
    }

    #[test]
    fn test_same_numbers() {
        let nums = vec![3, 3];
        assert_eq!(two_sum(&nums, 6), Some((0, 1)));
    }

    #[test]
    fn test_no_solution() {
        let nums = vec![1, 2, 3];
        assert_eq!(two_sum(&nums, 10), None);
    }

    #[test]
    fn test_negative_numbers() {
        let nums = vec![-1, -2, -3, -4, -5];
        assert_eq!(two_sum(&nums, -8), Some((2, 4)));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(two_sum(&[], 5), None);
        assert_eq!(two_sum(&[1], 1), None);
    }
}
```

#### Hints
1. Use a HashMap to store seen numbers and their indices
2. For each number, check if `target - num` exists in the HashMap
3. Time complexity: O(n), Space complexity: O(n)
4. Remember to check the complement before adding to HashMap

---

### 2.2 Rotate Array

**Difficulty**: 5 kyu
**Concepts**: Arrays, Slicing, Vec Operations
**Prerequisites**: Vec manipulation, slicing, modulo arithmetic

#### Problem Statement
Rotate an array to the right by k steps. For example, rotating `[1,2,3,4,5]` by 2 steps results in `[4,5,1,2,3]`.

#### Signature
```rust
fn rotate_array(nums: &mut Vec<i32>, k: usize)
```

#### Input/Output Specification
- **Input**:
  - `nums`: Mutable reference to Vec of integers
  - `k`: Number of rotation steps (can be larger than array length)
- **Output**: The array is modified in-place
- **Edge Cases**:
  - Empty array remains empty
  - `k = 0` means no rotation
  - `k >= nums.len()` wraps around (use modulo)

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_rotation() {
        let mut nums = vec![1, 2, 3, 4, 5];
        rotate_array(&mut nums, 2);
        assert_eq!(nums, vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn test_full_rotation() {
        let mut nums = vec![1, 2, 3];
        rotate_array(&mut nums, 3);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_larger_than_length() {
        let mut nums = vec![1, 2, 3, 4];
        rotate_array(&mut nums, 6);
        assert_eq!(nums, vec![3, 4, 1, 2]);
    }

    #[test]
    fn test_no_rotation() {
        let mut nums = vec![1, 2, 3];
        rotate_array(&mut nums, 0);
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn test_edge_cases() {
        let mut nums = vec![];
        rotate_array(&mut nums, 5);
        assert_eq!(nums, vec![]);

        let mut nums = vec![1];
        rotate_array(&mut nums, 10);
        assert_eq!(nums, vec![1]);
    }
}
```

#### Hints
1. Use modulo to handle k larger than array length
2. Approach 1: Use `split_at()` and concatenate
3. Approach 2: Reverse the array in parts
4. Approach 3: Use `rotate_right()` method (if available)

---

### 2.3 Group By Property

**Difficulty**: 4 kyu
**Concepts**: Collections, Iterators, Grouping
**Prerequisites**: HashMap, struct manipulation, iterators

#### Problem Statement
Given an array of items (structs or tuples), group them by a specific property. For this challenge, group a list of students by their grade level.

#### Signature
```rust
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Student {
    name: String,
    grade: u8,
}

fn group_by_grade(students: Vec<Student>) -> HashMap<u8, Vec<Student>>
```

#### Input/Output Specification
- **Input**: Vec of Student structs
- **Output**: HashMap where keys are grade levels and values are Vec of students
- **Edge Cases**:
  - Empty input returns empty HashMap
  - Single student creates single-entry HashMap
  - Students in same grade should maintain their original order

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_grouping() {
        let students = vec![
            Student { name: "Alice".to_string(), grade: 10 },
            Student { name: "Bob".to_string(), grade: 11 },
            Student { name: "Charlie".to_string(), grade: 10 },
        ];

        let grouped = group_by_grade(students);
        assert_eq!(grouped.get(&10).unwrap().len(), 2);
        assert_eq!(grouped.get(&11).unwrap().len(), 1);
    }

    #[test]
    fn test_single_grade() {
        let students = vec![
            Student { name: "Alice".to_string(), grade: 10 },
            Student { name: "Bob".to_string(), grade: 10 },
        ];

        let grouped = group_by_grade(students);
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped.get(&10).unwrap().len(), 2);
    }

    #[test]
    fn test_empty_input() {
        let students = vec![];
        let grouped = group_by_grade(students);
        assert_eq!(grouped.len(), 0);
    }

    #[test]
    fn test_order_preservation() {
        let students = vec![
            Student { name: "Alice".to_string(), grade: 10 },
            Student { name: "Bob".to_string(), grade: 10 },
        ];

        let grouped = group_by_grade(students);
        let grade_10 = grouped.get(&10).unwrap();
        assert_eq!(grade_10[0].name, "Alice");
        assert_eq!(grade_10[1].name, "Bob");
    }
}
```

#### Hints
1. Use HashMap with grade as key and Vec<Student> as value
2. Use `entry()` API to get or create Vec for each grade
3. Use `or_insert_with()` to initialize empty Vec
4. Clone students when adding to Vec (or use references)

---

## 3. Number Theory

### 3.1 GCD/LCM Calculator

**Difficulty**: 5 kyu
**Concepts**: Recursion/Loops, Math Operations
**Prerequisites**: Euclidean algorithm, function composition

#### Problem Statement
Implement functions to calculate the Greatest Common Divisor (GCD) and Least Common Multiple (LCM) of two positive integers. The GCD is the largest number that divides both numbers, while the LCM is the smallest number that is divisible by both.

#### Signature
```rust
fn gcd(a: u64, b: u64) -> u64
fn lcm(a: u64, b: u64) -> u64
fn gcd_lcm(a: u64, b: u64) -> (u64, u64)
```

#### Input/Output Specification
- **Input**: Two positive integers (u64)
- **Output**:
  - `gcd`: The greatest common divisor
  - `lcm`: The least common multiple
  - `gcd_lcm`: Tuple containing both (gcd, lcm)
- **Edge Cases**:
  - `gcd(0, n) = n` and `gcd(n, 0) = n`
  - `gcd(a, a) = a`
  - `lcm(a, b) = (a * b) / gcd(a, b)`

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(7, 13), 1);
    }

    #[test]
    fn test_basic_lcm() {
        assert_eq!(lcm(12, 8), 24);
        assert_eq!(lcm(21, 6), 42);
        assert_eq!(lcm(7, 13), 91);
    }

    #[test]
    fn test_gcd_lcm_combined() {
        assert_eq!(gcd_lcm(12, 8), (4, 24));
        assert_eq!(gcd_lcm(21, 6), (3, 42));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(7, 7), 7);
        assert_eq!(lcm(1, 5), 5);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(gcd(1071, 462), 21);
        assert_eq!(lcm(15, 25), 75);
    }
}
```

#### Hints
1. Use Euclidean algorithm for GCD: `gcd(a, b) = gcd(b, a % b)`
2. Base case: when b is 0, return a
3. LCM formula: `lcm(a, b) = (a * b) / gcd(a, b)`
4. Be careful with overflow when multiplying large numbers
5. Can implement recursively or iteratively

---

### 3.2 Fibonacci Sequence

**Difficulty**: 5 kyu
**Concepts**: Iterators, Collections, Math
**Prerequisites**: Vec operations, iteration, pattern matching

#### Problem Statement
Generate the first n numbers of the Fibonacci sequence. The Fibonacci sequence starts with 0 and 1, and each subsequent number is the sum of the previous two: 0, 1, 1, 2, 3, 5, 8, 13, 21, ...

#### Signature
```rust
fn fibonacci(n: usize) -> Vec<u64>
```

#### Input/Output Specification
- **Input**: n (number of Fibonacci numbers to generate)
- **Output**: Vec containing the first n Fibonacci numbers
- **Edge Cases**:
  - `n = 0` returns empty Vec
  - `n = 1` returns `[0]`
  - `n = 2` returns `[0, 1]`
  - Handle potential overflow for large n

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_fibonacci() {
        assert_eq!(fibonacci(5), vec![0, 1, 1, 2, 3]);
        assert_eq!(fibonacci(8), vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(fibonacci(0), vec![]);
        assert_eq!(fibonacci(1), vec![0]);
        assert_eq!(fibonacci(2), vec![0, 1]);
    }

    #[test]
    fn test_longer_sequence() {
        let fib = fibonacci(10);
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
        assert_eq!(fib.len(), 10);
    }

    #[test]
    fn test_large_numbers() {
        let fib = fibonacci(15);
        assert_eq!(fib[14], 377);
        assert_eq!(fib[12], 144);
    }
}
```

#### Hints
1. Pre-allocate Vec with capacity for efficiency
2. Handle n = 0, 1, 2 as special cases
3. Use two variables to track previous two numbers
4. Iterative approach is more efficient than recursive
5. Consider using iterator adapters for an elegant solution

---

### 3.3 Roman Numeral Conversion

**Difficulty**: 4 kyu
**Concepts**: Strings, Pattern Matching, Collections
**Prerequisites**: String manipulation, HashMap/match expressions, algorithms

#### Problem Statement
Implement conversion between integers (1-3999) and Roman numerals. Roman numerals use letters: I=1, V=5, X=10, L=50, C=100, D=500, M=1000. Subtractive notation is used for 4 (IV), 9 (IX), 40 (XL), 90 (XC), 400 (CD), and 900 (CM).

#### Signature
```rust
fn int_to_roman(num: u32) -> String
fn roman_to_int(s: &str) -> Option<u32>
```

#### Input/Output Specification
- **Input**:
  - `int_to_roman`: Integer between 1 and 3999
  - `roman_to_int`: String containing Roman numeral
- **Output**:
  - `int_to_roman`: Roman numeral string
  - `roman_to_int`: `Option<u32>` (None for invalid input)
- **Edge Cases**:
  - Invalid Roman numerals return None
  - Case-insensitive for roman_to_int
  - Numbers outside 1-3999 range

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman_basic() {
        assert_eq!(int_to_roman(1), "I");
        assert_eq!(int_to_roman(5), "V");
        assert_eq!(int_to_roman(10), "X");
        assert_eq!(int_to_roman(50), "L");
        assert_eq!(int_to_roman(100), "C");
    }

    #[test]
    fn test_int_to_roman_subtractive() {
        assert_eq!(int_to_roman(4), "IV");
        assert_eq!(int_to_roman(9), "IX");
        assert_eq!(int_to_roman(40), "XL");
        assert_eq!(int_to_roman(90), "XC");
        assert_eq!(int_to_roman(400), "CD");
        assert_eq!(int_to_roman(900), "CM");
    }

    #[test]
    fn test_int_to_roman_complex() {
        assert_eq!(int_to_roman(1994), "MCMXCIV");
        assert_eq!(int_to_roman(58), "LVIII");
        assert_eq!(int_to_roman(1984), "MCMLXXXIV");
    }

    #[test]
    fn test_roman_to_int_basic() {
        assert_eq!(roman_to_int("I"), Some(1));
        assert_eq!(roman_to_int("V"), Some(5));
        assert_eq!(roman_to_int("X"), Some(10));
    }

    #[test]
    fn test_roman_to_int_subtractive() {
        assert_eq!(roman_to_int("IV"), Some(4));
        assert_eq!(roman_to_int("IX"), Some(9));
        assert_eq!(roman_to_int("XL"), Some(40));
    }

    #[test]
    fn test_roman_to_int_complex() {
        assert_eq!(roman_to_int("MCMXCIV"), Some(1994));
        assert_eq!(roman_to_int("LVIII"), Some(58));
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(roman_to_int("mcmxciv"), Some(1994));
        assert_eq!(roman_to_int("Lviii"), Some(58));
    }

    #[test]
    fn test_invalid_input() {
        assert_eq!(roman_to_int("ABC"), None);
        assert_eq!(roman_to_int(""), None);
    }
}
```

#### Hints
1. For int_to_roman: Use an array of (value, numeral) pairs in descending order
2. For roman_to_int: Process character by character, comparing with next
3. If current < next, subtract current (subtractive notation)
4. Otherwise, add current
5. Consider using a HashMap for character-to-value mapping

---

## 4. Data Structure Basics

### 4.1 Stack Implementation

**Difficulty**: 5 kyu
**Concepts**: Structs, Generics, Option
**Prerequisites**: Generic types, Option handling, Vec manipulation

#### Problem Statement
Implement a generic stack data structure with basic operations: push, pop, peek, and is_empty. The stack should follow Last-In-First-Out (LIFO) principle.

#### Signature
```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self
    fn push(&mut self, item: T)
    fn pop(&mut self) -> Option<T>
    fn peek(&self) -> Option<&T>
    fn is_empty(&self) -> bool
    fn size(&self) -> usize
}
```

#### Input/Output Specification
- **Operations**:
  - `new()`: Create empty stack
  - `push(item)`: Add item to top
  - `pop()`: Remove and return top item (None if empty)
  - `peek()`: View top item without removing (None if empty)
  - `is_empty()`: Check if stack is empty
  - `size()`: Return number of items
- **Type**: Generic over any type T

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.size(), 0);
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack = Stack::new();
        assert_eq!(stack.peek(), None);

        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
        assert_eq!(stack.peek(), Some(&1)); // Peek doesn't remove

        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn test_size() {
        let mut stack = Stack::new();
        assert_eq!(stack.size(), 0);

        stack.push(1);
        assert_eq!(stack.size(), 1);

        stack.push(2);
        stack.push(3);
        assert_eq!(stack.size(), 3);

        stack.pop();
        assert_eq!(stack.size(), 2);
    }

    #[test]
    fn test_generic_types() {
        let mut string_stack = Stack::new();
        string_stack.push("hello".to_string());
        string_stack.push("world".to_string());
        assert_eq!(string_stack.pop(), Some("world".to_string()));
    }
}
```

#### Hints
1. Use a Vec<T> internally to store items
2. `push` corresponds to `Vec::push`
3. `pop` corresponds to `Vec::pop`
4. `peek` uses `Vec::last()`
5. `is_empty` checks if Vec length is 0

---

### 4.2 Queue Implementation

**Difficulty**: 5 kyu
**Concepts**: Structs, Collections, VecDeque
**Prerequisites**: VecDeque usage, generic types, Option handling

#### Problem Statement
Implement a generic queue data structure with basic operations: enqueue, dequeue, peek, is_empty, and size. The queue should follow First-In-First-Out (FIFO) principle.

#### Signature
```rust
use std::collections::VecDeque;

struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self
    fn enqueue(&mut self, item: T)
    fn dequeue(&mut self) -> Option<T>
    fn peek(&self) -> Option<&T>
    fn is_empty(&self) -> bool
    fn size(&self) -> usize
}
```

#### Input/Output Specification
- **Operations**:
  - `new()`: Create empty queue
  - `enqueue(item)`: Add item to back of queue
  - `dequeue()`: Remove and return front item (None if empty)
  - `peek()`: View front item without removing (None if empty)
  - `is_empty()`: Check if queue is empty
  - `size()`: Return number of items
- **Type**: Generic over any type T

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_queue_is_empty() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_enqueue_and_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_peek() {
        let mut queue = Queue::new();
        assert_eq!(queue.peek(), None);

        queue.enqueue(1);
        assert_eq!(queue.peek(), Some(&1));

        queue.enqueue(2);
        assert_eq!(queue.peek(), Some(&1)); // Still first item
    }

    #[test]
    fn test_fifo_order() {
        let mut queue = Queue::new();
        queue.enqueue("first");
        queue.enqueue("second");
        queue.enqueue("third");

        assert_eq!(queue.dequeue(), Some("first"));
        queue.enqueue("fourth");
        assert_eq!(queue.dequeue(), Some("second"));
        assert_eq!(queue.dequeue(), Some("third"));
        assert_eq!(queue.dequeue(), Some("fourth"));
    }

    #[test]
    fn test_size() {
        let mut queue = Queue::new();
        assert_eq!(queue.size(), 0);

        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.size(), 2);

        queue.dequeue();
        assert_eq!(queue.size(), 1);
    }
}
```

#### Hints
1. Use VecDeque for efficient front and back operations
2. `enqueue` uses `push_back()`
3. `dequeue` uses `pop_front()`
4. `peek` uses `front()`
5. VecDeque provides O(1) operations at both ends

---

### 4.3 HashMap Usage - Most Frequent Element

**Difficulty**: 4 kyu
**Concepts**: HashMap, Iteration, Comparison
**Prerequisites**: HashMap operations, iterator methods, Option handling

#### Problem Statement
Given an array of elements, find the most frequent element. If there's a tie, return any of the tied elements. Use HashMap to efficiently count occurrences.

#### Signature
```rust
use std::collections::HashMap;

fn most_frequent<T: Eq + std::hash::Hash + Clone>(items: &[T]) -> Option<T>
```

#### Input/Output Specification
- **Input**: Slice of any hashable and equatable type
- **Output**: `Option<T>` containing the most frequent element, or None if input is empty
- **Edge Cases**:
  - Empty array returns None
  - Single element returns that element
  - Tie: return any of the tied elements
  - Works with any type that implements Hash + Eq

#### Example Test Cases
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_frequency() {
        let nums = vec![1, 2, 2, 3, 3, 3, 4];
        assert_eq!(most_frequent(&nums), Some(3));
    }

    #[test]
    fn test_strings() {
        let words = vec!["apple", "banana", "apple", "cherry", "apple"];
        assert_eq!(most_frequent(&words), Some("apple"));
    }

    #[test]
    fn test_all_same_frequency() {
        let nums = vec![1, 2, 3, 4];
        let result = most_frequent(&nums);
        assert!(result.is_some());
        assert!([1, 2, 3, 4].contains(&result.unwrap()));
    }

    #[test]
    fn test_edge_cases() {
        let empty: Vec<i32> = vec![];
        assert_eq!(most_frequent(&empty), None);

        let single = vec![42];
        assert_eq!(most_frequent(&single), Some(42));
    }

    #[test]
    fn test_tie() {
        let nums = vec![1, 1, 2, 2, 3];
        let result = most_frequent(&nums);
        assert!(result == Some(1) || result == Some(2));
    }

    #[test]
    fn test_with_owned_strings() {
        let words = vec![
            "rust".to_string(),
            "go".to_string(),
            "rust".to_string(),
            "python".to_string(),
            "rust".to_string(),
        ];
        assert_eq!(most_frequent(&words), Some("rust".to_string()));
    }
}
```

#### Hints
1. Create HashMap to count occurrences
2. Use `entry().or_insert(0)` pattern for counting
3. Iterate through HashMap to find maximum count
4. Keep track of both max count and corresponding element
5. Generic type T must implement Eq, Hash, and Clone

---

## Learning Path

### Recommended Order
1. **Start with Strings** (1.1 → 1.2 → 1.3)
   - Build understanding of string manipulation
   - Learn HashMap basics with word frequency

2. **Move to Arrays** (2.1 → 2.2 → 2.3)
   - Master HashMap with Two Sum
   - Practice Vec operations with rotation
   - Combine concepts with grouping

3. **Number Theory** (3.2 → 3.1 → 3.3)
   - Start with Fibonacci (simpler)
   - Move to GCD/LCM (recursion)
   - Challenge yourself with Roman numerals

4. **Data Structures** (4.1 → 4.2 → 4.3)
   - Implement Stack first
   - Then Queue (similar but FIFO)
   - Apply HashMap knowledge to find most frequent

### Tips for Success
1. **Read the tests first** - They show exactly what's expected
2. **Start simple** - Get basic cases working before edge cases
3. **Use the hints** - They point you in the right direction
4. **Test incrementally** - Run tests after each implementation step
5. **Refactor** - First make it work, then make it clean
6. **Learn from failures** - Test failures teach you about Rust

### Common Pitfalls
- **Ownership issues**: Remember to clone when needed
- **Off-by-one errors**: Pay attention to inclusive/exclusive ranges
- **Empty input**: Always handle edge cases
- **Type confusion**: Be explicit about type annotations
- **Borrowing**: Understand when to use `&` vs ownership

---

## Next Steps

After completing Level 2 challenges:
1. **Review Solutions** - Compare your approach with optimal solutions
2. **Measure Performance** - Use benchmarks to compare implementations
3. **Advance to Level 3** - Tackle 3-4 kyu problems (Intermediate)
4. **Join Community** - Share solutions and learn from others
5. **Build Projects** - Apply these patterns in real projects

---

## Resources

### Rust Documentation
- [The Rust Book - Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Rust by Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
- [std::collections Documentation](https://doc.rust-lang.org/std/collections/)

### Algorithm Resources
- [Leetcode](https://leetcode.com/) - Similar problems for practice
- [Codewars](https://www.codewars.com/) - Kyu-ranked challenges
- [Exercism Rust Track](https://exercism.org/tracks/rust) - Mentored learning

### Community
- [Rust Users Forum](https://users.rust-lang.org/)
- [r/rust Subreddit](https://www.reddit.com/r/rust/)
- [Rust Discord](https://discord.gg/rust-lang)

---

## Contributing

Found an issue or want to add more challenges? Contributions are welcome! Please ensure:
- Clear problem statements
- Comprehensive test cases
- Appropriate difficulty level
- Educational value

---

**Happy Coding! 🦀**
