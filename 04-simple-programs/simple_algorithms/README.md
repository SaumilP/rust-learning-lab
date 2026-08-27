# Simple Algorithms

## Overview

Algorithms are step-by-step procedures to solve problems. Common simple algorithms include sorting, searching, and filtering. These form the foundation for more complex problem-solving. Rust's collections and iterators make implementing algorithms concise and efficient.

## Theory

### Core Algorithm Concepts

1. **Sorting** - Arranging data in order (ascending/descending)
2. **Searching** - Finding specific elements or patterns
3. **Filtering** - Selecting elements matching criteria
4. **Transformation** - Converting data from one form to another
5. **Aggregation** - Combining data to produce a single result

### Algorithm Complexity

Algorithms have different efficiency levels:
- **Time complexity** - How execution time grows with input size
- **Space complexity** - How memory usage grows with input size
- Simple algorithms: O(n), O(n²) are acceptable for small data

### Why Simple Algorithms Matter

- Foundation for complex systems
- Teach problem-solving approaches
- Demonstrate language features
- Essential for technical interviews

## Syntax

### Linear Search

```rust
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(i);
        }
    }
    None
}
```

### Binary Search

```rust
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None
}
```

### Bubble Sort

```rust
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
```

### Finding Maximum

```rust
fn find_max(arr: &[i32]) -> Option<i32> {
    arr.iter().copied().max()
}

// Or manually
fn find_max_manual(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() {
        None
    } else {
        let mut max = arr[0];
        for &value in &arr[1..] {
            if value > max {
                max = value;
            }
        }
        Some(max)
    }
}
```

## Common Patterns

### Pattern 1: Simple Linear Search

```rust
fn contains(arr: &[i32], target: i32) -> bool {
    for &value in arr {
        if value == target {
            return true;
        }
    }
    false
}

// Or using iterator
fn contains_iter(arr: &[i32], target: i32) -> bool {
    arr.iter().any(|&x| x == target)
}
```

### Pattern 2: Sorting with Custom Logic

```rust
fn sort_descending(arr: &mut [i32]) {
    arr.sort_by(|a, b| b.cmp(a));
}

fn sort_by_length(words: &mut [String]) {
    words.sort_by_key(|w| w.len());
}
```

### Pattern 3: Filtering and Collecting

```rust
fn even_numbers(arr: &[i32]) -> Vec<i32> {
    arr.iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect()
}
```

### Pattern 4: Counting Occurrences

```rust
fn count_occurrences(arr: &[i32], target: i32) -> usize {
    arr.iter().filter(|&&x| x == target).count()
}
```

### Pattern 5: Partition Array

```rust
fn partition(arr: &[i32]) -> (Vec<i32>, Vec<i32>) {
    arr.iter().partition(|&&x| x % 2 == 0)
}
```

## Common Mistakes

### Mistake 1: Off-By-One Errors in Loops

```rust
// ❌ WRONG - Excludes last element
for i in 0..arr.len() - 1 {
    // process arr[i]
}

// ✅ CORRECT - Includes all elements
for i in 0..arr.len() {
    // process arr[i]
}
```

### Mistake 2: Not Handling Empty Collections

```rust
// ❌ WRONG - Panics on empty array
let max = arr[0];

// ✅ CORRECT - Returns Option
fn find_max(arr: &[i32]) -> Option<i32> {
    if arr.is_empty() { None } else { Some(*arr.iter().max()?) }
}
```

### Mistake 3: Inefficient Repeated Searches

```rust
// ❌ WRONG - O(n²) complexity
for item in collection {
    if collection.contains(&item) { }
}

// ✅ CORRECT - Use HashSet for O(1) lookup
use std::collections::HashSet;
let set: HashSet<_> = collection.iter().collect();
for item in collection {
    if set.contains(&item) { }
}
```

### Mistake 4: Forgetting to Handle Unsorted Arrays

```rust
// ❌ WRONG - Binary search requires sorted array
fn search_sorted(arr: &[i32], target: i32) -> Option<usize> {
    // Uses binary search
    // Works only if arr is sorted!
}

// ✅ CORRECT - Sort first or use linear search
let mut arr = vec![3, 1, 4, 1, 5];
arr.sort();
binary_search(&arr, 4);
```

### Mistake 5: Inefficient String Comparisons

```rust
// ❌ WRONG - Case-sensitive comparison
if word.contains("hello") { }

// ✅ CORRECT - Case-insensitive when appropriate
if word.to_lowercase().contains("hello") { }
```

## Real-World Examples

### Example 1: Word Frequency Counter

```rust
use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut frequencies = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *frequencies.entry(word).or_insert(0) += 1;
    }
    frequencies
}

fn main() {
    let text = "rust is great rust is fun";
    let freq = word_frequency(text);
    for (word, count) in freq.iter() {
        println!("{}: {}", word, count);
    }
}
```

### Example 2: Fibonacci Generator

```rust
fn fibonacci(n: usize) -> Vec<u32> {
    if n == 0 { return vec![]; }
    if n == 1 { return vec![1]; }

    let mut fib = vec![1, 1];
    for _ in 2..n {
        let next = fib[fib.len() - 1] + fib[fib.len() - 2];
        fib.push(next);
    }
    fib
}

fn main() {
    println!("{:?}", fibonacci(10));
}
```

### Example 3: Anagram Checker

```rust
fn are_anagrams(word1: &str, word2: &str) -> bool {
    let mut chars1: Vec<_> = word1.chars().collect();
    let mut chars2: Vec<_> = word2.chars().collect();

    chars1.sort();
    chars2.sort();

    chars1 == chars2
}

fn main() {
    println!("{}", are_anagrams("listen", "silent")); // true
    println!("{}", are_anagrams("hello", "world"));   // false
}
```

## Related Concepts

### Prerequisites
- Module 01: Functions, Control Flow
- Module 02: Collections, Iterators
- Module 04: CLI Arguments, File I/O

### Follow-ups
- Text Processing (applying algorithms to text)
- Advanced Algorithms (sorting variants, searching strategies)
- Data Structures (implementing custom algorithms)
- Performance Optimization (algorithm analysis)

## Best Practices

1. **Understand the problem** - Before implementing, understand what you're solving
2. **Start simple** - Use iterator methods before manual loops
3. **Handle edge cases** - Empty collections, single elements, etc.
4. **Choose right algorithm** - O(n) is better than O(n²) for large data
5. **Test with examples** - Verify with different inputs
6. **Document complexity** - Add comments about algorithm efficiency
7. **Use built-in methods** - Rust provides optimized implementations

## Summary

Simple algorithms demonstrate fundamental problem-solving approaches. Rust's collections and iterator methods make algorithms concise. Understanding common patterns like searching, sorting, and filtering builds foundation for more complex systems.

## Practice Exercise Ideas

1. Implement multiple sorting algorithms (bubble, selection, insertion)
2. Create search functions (linear, binary)
3. Build a palindrome checker
4. Implement GCD/LCM calculators
5. Create a duplicate finder in arrays

