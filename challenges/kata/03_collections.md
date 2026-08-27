# Kata: Collections Problems

## Problem 1: Find Duplicate

**Difficulty**: Beginner
**Concepts**: Vectors, iteration, early returns
**Prerequisites**: 02-standard-library (Collections, Vec<T>)

### Problem Statement

Write a function that finds and returns the first duplicate element in a vector.

### Requirements
- Accept a slice of integers
- Return the first value that appears more than once
- Return None if no duplicates exist
- Values can appear in any order

### Function Signature
```rust
pub fn find_duplicate(arr: &[i32]) -> Option<i32> {
    // Your implementation here
}
```

### Test Cases
```
Input: &[1, 2, 3, 2, 4]
Output: Some(2)

Input: &[5, 5, 1, 2, 3]
Output: Some(5)

Input: &[1, 2, 3, 4, 5]
Output: None

Input: &[]
Output: None

Input: &[1, 1, 1, 1]
Output: Some(1)

Input: &[3, 1, 4, 1, 5]
Output: Some(1)
```

### Hints
- Consider using a HashSet to track seen values
- Iterate through array and check if value already seen
- Return first duplicate found (early return with Some)
- Return None if loop completes without finding duplicate

### Related Concepts
- Option<T> type
- HashSet usage
- Early returns

---

## Problem 2: Remove Duplicates

**Difficulty**: Beginner
**Concepts**: Vectors, HashSet, collecting
**Prerequisites**: 02-standard-library (Collections, HashSet)

### Problem Statement

Write a function that returns a new vector containing unique elements from the input.

### Requirements
- Remove all duplicate values
- Return vector with unique elements
- Order doesn't need to be preserved
- Handle empty vectors

### Function Signature
```rust
pub fn remove_duplicates(arr: &[i32]) -> Vec<i32> {
    // Your implementation here
}
```

### Test Cases
```
Input: &[1, 2, 2, 3, 3, 3, 4]
Output: [1, 2, 3, 4] (order may vary)

Input: &[1, 1, 1, 1]
Output: [1]

Input: &[1, 2, 3, 4, 5]
Output: [1, 2, 3, 4, 5] (or any order with all elements)

Input: &[]
Output: []

Input: &[5, 4, 3, 2, 1, 1, 2, 3]
Output: [1, 2, 3, 4, 5] (order may vary)
```

### Hints
- Use HashSet to automatically handle uniqueness
- Convert slice to HashSet
- Convert HashSet back to Vec
- Order doesn't matter for this problem

### Related Concepts
- HashSet properties
- Type conversions
- Deduplication patterns

---

## Problem 3: Merge Sorted Arrays

**Difficulty**: Beginner
**Concepts**: Vectors, iteration, comparison
**Prerequisites**: 02-standard-library (Collections, Vec<T>)

### Problem Statement

Write a function that merges two sorted arrays into a single sorted array.

### Requirements
- Accept two sorted slices
- Return merged vector in sorted order
- Both input arrays are pre-sorted in ascending order
- Handle empty arrays

### Function Signature
```rust
pub fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    // Your implementation here
}
```

### Test Cases
```
Input: &[1, 3, 5], &[2, 4, 6]
Output: vec![1, 2, 3, 4, 5, 6]

Input: &[1, 2, 3], &[]
Output: vec![1, 2, 3]

Input: &[], &[1, 2, 3]
Output: vec![1, 2, 3]

Input: &[1, 2], &[1, 2]
Output: vec![1, 1, 2, 2]

Input: &[], &[]
Output: vec![]
```

### Hints
- Use two-pointer approach
- Compare elements from both arrays
- Add smaller element to result
- Handle remaining elements at end

### Related Concepts
- Two-pointer technique
- Vector building
- Algorithm efficiency

---

## Problem 4: Filter Type

**Difficulty**: Beginner
**Concepts**: Collections, type checking, filtering
**Prerequisites**: 02-standard-library (Collections, Type conversions)

### Problem Statement

Write a function that extracts only numbers from a mixed vector of values.

### Requirements
- Accept a vector reference (represent mixed types as enum)
- Filter to return only numeric values
- Preserve order of matching elements
- Handle empty vectors

### Note: Since Rust is statically typed, represent this as filtering i32 values from a vector that may have other types.

### Function Signature
```rust
pub fn filter_numbers(values: &[Value]) -> Vec<i32>
where
    Value = enum Value { Number(i32), Text(String), ... }
{
    // Your implementation here
}
```

### Alternative (simpler):
```rust
pub fn filter_even_numbers(arr: &[i32]) -> Vec<i32> {
    // Your implementation here - filter only even numbers
}
```

### Test Cases (for simpler version)
```
Input: &[1, 2, 3, 4, 5, 6]
Output: [2, 4, 6]

Input: &[1, 3, 5, 7]
Output: []

Input: &[2, 4, 6, 8]
Output: [2, 4, 6, 8]

Input: &[]
Output: []
```

### Hints
- Use `.filter()` to keep only matching elements
- For even check: `n % 2 == 0`
- Collect filtered results into new Vec
- Remember: filter creates iterator that returns only true values

### Related Concepts
- Iterator filtering
- Closures in filter
- Type matching

---

## Problem 5: Flatten One Level

**Difficulty**: Beginner
**Concepts**: Nested vectors, iteration
**Prerequisites**: 02-standard-library (Collections, Vec<T>)

### Problem Statement

Write a function that flattens a vector of vectors by one level.

### Requirements
- Accept a vector of vectors (Vec<Vec<i32>>)
- Return single-level vector with all elements
- Preserve order of elements
- Handle empty outer vector

### Function Signature
```rust
pub fn flatten_one_level(matrix: &[Vec<i32>]) -> Vec<i32> {
    // Your implementation here
}
```

### Test Cases
```
Input: &[vec![1, 2], vec![3, 4], vec![5]]
Output: vec![1, 2, 3, 4, 5]

Input: &[vec![1], vec![2], vec![3]]
Output: vec![1, 2, 3]

Input: &[]
Output: vec![]

Input: &[vec![]]
Output: vec![]

Input: &[vec![1, 2, 3, 4]]
Output: vec![1, 2, 3, 4]
```

### Hints
- Iterate through outer vector
- For each inner vector, add all elements to result
- Use `.iter().flat_map()` for elegant solution
- Or use nested loops

### Related Concepts
- Nested collections
- flatten() and flat_map() methods
- Iterator chaining

---

## Summary of Kata: Collections

These problems practice:
- Working with Vec<T>
- HashSet usage
- Filtering and transforming collections
- Merging and combining data
- Nested data structures

**Recommended Order**: Complete problems 1-5 sequentially
**Time Estimate**: 30-45 minutes total
**Difficulty Progression**: 1→2→3→4→5 (slight increase)
