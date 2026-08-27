# Simple Algorithms - Key Takeaways

## Core Algorithms

### Searching
```rust
// Linear search
arr.iter().position(|&x| x == target)

// Binary search (requires sorted array)
arr.binary_search(&target)

// Contains check
arr.iter().any(|&x| x == target)
```

### Sorting
```rust
// Ascending
arr.sort();

// Descending
arr.sort_by(|a, b| b.cmp(a));

// By key
arr.sort_by_key(|x| x.field);
```

### Filtering
```rust
// Filter to vector
arr.iter().filter(|&&x| x > 5).copied().collect()

// Partition into two groups
let (evens, odds): (Vec<_>, Vec<_>) = arr.iter()
    .partition(|&&x| x % 2 == 0);
```

### Finding
```rust
// First element
arr.first()

// Last element
arr.last()

// Maximum
arr.iter().max()

// Minimum
arr.iter().min()
```

## Common Patterns

| Task | Method |
|------|--------|
| Find element | `iter().position()` or `binary_search()` |
| Count matches | `iter().filter().count()` |
| Check if any match | `iter().any()` |
| Check if all match | `iter().all()` |
| Sum values | `iter().sum()` |
| Find max/min | `iter().max()` / `iter().min()` |

## Algorithm Complexity

| Algorithm | Time | When to Use |
|-----------|------|------------|
| Linear search | O(n) | Unsorted data |
| Binary search | O(log n) | Sorted data only |
| Bubble sort | O(n²) | Small datasets |
| Insertion sort | O(n²) | Small, nearly sorted |
| Merge sort | O(n log n) | General purpose |

## Built-in Methods

```rust
// All of these exist on iterators/collections
arr.iter().max()           // Maximum value
arr.iter().min()           // Minimum value
arr.iter().any(|x| ...)    // Any match condition
arr.iter().all(|x| ...)    // All match condition
arr.iter().find(|x| ...)   // First match
arr.iter().position(|x...) // First index
arr.iter().filter(|x| ...) // Keep matches
arr.iter().map(|x| ...)    // Transform
arr.iter().fold(0, |a,b|)  // Accumulate
```

## Important Concepts

✓ Use iterator methods instead of manual loops
✓ Handle empty collections (return Option)
✓ Binary search requires sorted array
✓ Iterator chains are lazy and efficient
✓ Use appropriate algorithm for data size
✓ Test edge cases (empty, single, duplicates)
✓ Understand time and space complexity

## Edge Cases to Handle

```rust
// Empty collection
if arr.is_empty() { return None; }

// Single element
if arr.len() == 1 { return Some(arr[0]); }

// Duplicate elements
arr.sort();  // Duplicates stay together

// Negative numbers
// Algorithm should still work
```

## Quick Reference

```rust
// Find element
arr.iter().find(|&&x| x == target)?

// Count matching
arr.iter().filter(|&&x| x == target).count()

// Sort descending
arr.sort_by(|a, b| b.cmp(a))

// Sum values
arr.iter().sum::<i32>()

// Any/all
arr.iter().any(|&x| x > 5)
arr.iter().all(|&x| x > 0)
```

## Common Mistakes to Avoid

1. ❌ Using binary search on unsorted array
2. ❌ Off-by-one errors in loop ranges
3. ❌ Not handling empty collections
4. ❌ Inefficient repeated searching
5. ❌ Wrong comparison operators in filters

