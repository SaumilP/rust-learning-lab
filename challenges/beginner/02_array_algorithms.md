# Beginner Level: Array Algorithm Problems

## Problem 1: Two Sum

**Difficulty**: Beginner-Intermediate
**Concepts**: Iteration, HashMap, algorithm design
**Prerequisites**: 02-standard-library (Collections, HashMap)

### Problem Statement

Given an array of integers and a target sum, find two numbers that add up to the target and return their indices.

### Requirements
- Accept array slice and target sum
- Return indices of two different elements that sum to target
- Return Option with tuple of indices
- Return None if no pair exists
- Each element can be used only once

### Function Signature
```rust
pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    // Your implementation here
}
```

### Test Cases
```
Input: nums=[2, 7, 11, 15], target=9
Output: Some((0, 1))

Input: nums=[3, 2, 4], target=6
Output: Some((1, 2))

Input: nums=[3, 3], target=6
Output: Some((0, 1))

Input: nums=[1, 2, 3], target=10
Output: None

Input: nums=[], target=5
Output: None

Input: nums=[5], target=10
Output: None
```

### Hints
- Use HashMap to store value -> index mapping
- For each number, check if (target - number) is in HashMap
- This gives O(n) solution vs O(n²) nested loop

### Algorithm Approach (Optimized)
1. Create HashMap to store seen numbers
2. For each number:
   - Calculate complement = target - number
   - If complement in HashMap, return indices
   - Otherwise, add current number to HashMap
3. Return None if no pair found

### Related Concepts
- HashMap lookups
- Algorithm optimization
- Index tracking

---

## Problem 2: Rotate Array

**Difficulty**: Beginner-Intermediate
**Concepts**: Array manipulation, slicing
**Prerequisites**: 02-standard-library (Collections, Slicing)

### Problem Statement

Rotate array elements to the right by k steps.

### Requirements
- Accept array slice and rotation amount k
- Return rotated vector
- Handle k larger than array length
- Preserve element order within rotation
- Handle empty arrays

### Function Signature
```rust
pub fn rotate_array(arr: &[i32], k: usize) -> Vec<i32> {
    // Your implementation here
}
```

### Test Cases
```
Input: arr=[1, 2, 3, 4, 5], k=2
Output: [4, 5, 1, 2, 3]

Input: arr=[1, 2, 3], k=1
Output: [3, 1, 2]

Input: arr=[1, 2, 3, 4], k=5
Output: [4, 1, 2, 3] (k mod length = 1)

Input: arr=[1], k=0
Output: [1]

Input: arr=[1, 2], k=3
Output: [2, 1]

Input: arr=[], k=1
Output: []
```

### Hints
- Effective rotation = k % array.len()
- Split array at rotation point
- Reorder pieces to get rotated result
- Or use reverse trick: reverse all, reverse first k, reverse rest

### Algorithm Approach (Split)
1. Calculate effective_k = k % arr.len()
2. Split array: [...] -> [last effective_k] + [first rest]
3. Concatenate rotated parts

Algorithm Approach (Reverse)
1. Reverse entire array
2. Reverse first k elements
3. Reverse remaining elements

### Related Concepts
- Array slicing and splitting
- Index calculations with modulo
- Rotation algorithms

---

## Problem 3: Group Array Elements

**Difficulty**: Beginner-Intermediate
**Concepts**: Grouping, HashMap, custom structures
**Prerequisites**: 02-standard-library (Collections, HashMap)

### Problem Statement

Group array elements by a property (e.g., even/odd or by value itself).

### Requirements
- Accept array and grouping function/property
- Return HashMap with groups
- Handle empty arrays
- Preserve element values

### Function Signature (Simplified)
```rust
pub fn group_by_even_odd(arr: &[i32]) -> (Vec<i32>, Vec<i32>) {
    // Return (even_numbers, odd_numbers)
}
```

### Test Cases
```
Input: [1, 2, 3, 4, 5, 6]
Output: ([2, 4, 6], [1, 3, 5])

Input: [1, 3, 5]
Output: ([], [1, 3, 5])

Input: [2, 4, 6]
Output: ([2, 4, 6], [])

Input: []
Output: ([], [])

Input: [0]
Output: ([0], [])
```

### Hints
- Use separate vectors or HashMap for grouping
- For each element, determine its group
- Add to appropriate collection
- Or use filter for each group

### Algorithm Approach
1. Create collections for each group
2. Iterate through array
3. For each element, add to appropriate group based on property
4. Return groups

### Related Concepts
- Conditional grouping
- HashMap or Vec usage
- Pattern matching on properties

---

## Problem 4: Find Element Pairs with Sum

**Difficulty**: Beginner-Intermediate
**Concepts**: Nested iteration, pair finding
**Prerequisites**: 02-standard-library (Collections)

### Problem Statement

Find all unique pairs in an array that sum to a target value.

### Requirements
- Accept array and target sum
- Return vector of pairs that sum to target
- Each pair should be (smaller, larger)
- No duplicate pairs
- Each element can be used only once

### Function Signature
```rust
pub fn find_pairs_with_sum(arr: &[i32], target: i32) -> Vec<(i32, i32)> {
    // Your implementation here
}
```

### Test Cases
```
Input: arr=[1, 2, 3, 4, 5], target=6
Output: [(1, 5), (2, 4)]

Input: arr=[1, 1, 2, 3], target=4
Output: [(1, 3)]

Input: arr=[0, 0], target=0
Output: [(0, 0)]

Input: arr=[1, 2, 3], target=10
Output: []

Input: arr=[], target=5
Output: []
```

### Hints
- Use HashSet to track seen values
- For each number, check if complement exists
- Ensure pairs aren't duplicated (store as ordered tuples)
- Or sort and use two-pointer approach

### Algorithm Approach (HashSet)
1. Create HashSet for tracking seen numbers
2. For each number:
   - Calculate complement = target - number
   - If complement in set and complement <= number (to avoid duplicates)
     - Add pair to result
   - Add number to set
3. Return pairs

### Related Concepts
- Pair finding algorithms
- Duplicate avoidance
- HashSet for O(1) lookup

---

## Summary of Beginner Level: Array Algorithms

These problems practice:
- Advanced iteration techniques
- Algorithm design and optimization
- Working with indices
- Grouping and organizing data

**Recommended Order**: Complete problems 1-4 sequentially
**Time Estimate**: 60-90 minutes total
**Difficulty Progression**: 1→2→3→4 (gradual increase)

---

## Prerequisites Before Starting

Before attempting these problems, ensure you understand:
- Array/slice operations
- HashMap and HashSet
- Iterators and filtering
- Index manipulation
- Tuple types for returning multiple values
