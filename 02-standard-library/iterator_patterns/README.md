# Concept: Iterator Patterns

## Overview

Iterators are a core abstraction in Rust for working with sequences of elements. They provide a functional, composable way to transform and filter data without explicitly writing loops. Understanding iterators is essential for writing idiomatic, expressive Rust code. This concept covers iterator creation, common methods, adapter chains, and patterns for effective use.

## Learning Objectives

By the end of this concept, you will understand:
- What iterators are and why they're useful
- Iterator adapters (map, filter, etc.) and consumers
- Method chaining patterns
- Lazy evaluation and performance benefits
- Common iterator patterns and idioms
- Iterator trait and implementation
- When to use iterators vs loops

## Theory

### What is an Iterator?

An iterator is a value that implements the `Iterator` trait, which allows you to iterate over sequences. The key insight is that iterators are **lazy** - they don't execute until you consume them.

```rust
let v = vec![1, 2, 3, 4, 5];

// Iterator - does nothing yet
let iter = v.iter();  // Type: std::slice::Iter

// Consuming - actually processes
for item in iter {
    println!("{}", item);
}
```

### Three Types of Iterators

Rust provides three ways to iterate, depending on what you need:

**`iter()` - Borrowing Iterator**

Borrows each element immutably:

```rust
let v = vec![1, 2, 3];

for item in v.iter() {
    println!("{}", item);  // item is &i32
}

println!("{:?}", v);  // v still usable (not moved)
```

**`iter_mut()` - Mutable Iterator**

Borrows each element mutably:

```rust
let mut v = vec![1, 2, 3];

for item in v.iter_mut() {
    *item *= 2;  // Modify each element
}

println!("{:?}", v);  // [2, 4, 6]
```

**`into_iter()` - Consuming Iterator**

Takes ownership of each element:

```rust
let v = vec![1, 2, 3];

for item in v.into_iter() {
    println!("{}", item);  // item is i32
}

// println!("{:?}", v);  // ERROR: v moved
```

**Comparison Table**

| Iterator | Borrows | Mutable | Consumes | Use When |
|----------|---------|---------|----------|----------|
| `iter()` | Yes | No | No | Read-only access |
| `iter_mut()` | Yes | Yes | No | Need to modify |
| `into_iter()` | No | N/A | Yes | Need ownership |

### Iterator Adapters and Consumers

Iterators have two types of methods:

**Adapters** - Return a new iterator (lazy):

```rust
let v = vec![1, 2, 3, 4, 5];

// These don't execute yet
v.iter()
    .map(|x| x * 2)          // Adapter: transform
    .filter(|x| x > &4)      // Adapter: filter
    .take(2)                 // Adapter: limit

// Code above does NOTHING until consumed
```

**Consumers** - Execute and return final value:

```rust
let v = vec![1, 2, 3, 4, 5];

// Consumers actually execute
let sum: i32 = v.iter().sum();              // Consumer
let product: i32 = v.iter().product();      // Consumer
let count: usize = v.iter().count();        // Consumer
let vec: Vec<i32> = v.iter().map(|x| x * 2).collect();  // Consumer
```

### Common Iterator Adapters

**map()**

Transforms each element:

```rust
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
// [2, 4, 6]
```

**filter()**

Keeps elements matching a predicate:

```rust
let v = vec![1, 2, 3, 4, 5];
let evens: Vec<i32> = v.iter()
    .filter(|x| x % 2 == 0)
    .copied()
    .collect();
// [2, 4]
```

**take()**

Takes first n elements:

```rust
let v = vec![1, 2, 3, 4, 5];
let first_three: Vec<i32> = v.iter()
    .take(3)
    .copied()
    .collect();
// [1, 2, 3]
```

**skip()**

Skips first n elements:

```rust
let v = vec![1, 2, 3, 4, 5];
let rest: Vec<i32> = v.iter()
    .skip(2)
    .copied()
    .collect();
// [3, 4, 5]
```

**enumerate()**

Gives index and element:

```rust
let v = vec!["a", "b", "c"];
for (i, item) in v.iter().enumerate() {
    println!("{}: {}", i, item);
}
// 0: a
// 1: b
// 2: c
```

**zip()**

Combines two iterators:

```rust
let a = vec![1, 2, 3];
let b = vec!["a", "b", "c"];
for (num, letter) in a.iter().zip(b.iter()) {
    println!("{}: {}", num, letter);
}
// 1: a, 2: b, 3: c
```

**fold()**

Accumulates a value:

```rust
let v = vec![1, 2, 3, 4];
let sum = v.iter().fold(0, |acc, x| acc + x);
// 10

let product = v.iter().fold(1, |acc, x| acc * x);
// 24
```

**rev()**

Reverses iterator:

```rust
let v = vec![1, 2, 3];
let reversed: Vec<i32> = v.iter()
    .rev()
    .copied()
    .collect();
// [3, 2, 1]
```

### Common Iterator Consumers

**collect()**

Gathers into a collection:

```rust
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.iter()
    .map(|x| x * 2)
    .collect();
```

**sum() and product()**

Sum or multiply all elements:

```rust
let v = vec![1, 2, 3, 4];
let sum: i32 = v.iter().sum();       // 10
let product: i32 = v.iter().product();  // 24
```

**count()**

Counts elements:

```rust
let v = vec![1, 2, 3, 4, 5];
let count = v.iter().filter(|x| x > &2).count();
// 3
```

**any() and all()**

Check if any/all match:

```rust
let v = vec![1, 2, 3, 4, 5];
let has_even = v.iter().any(|x| x % 2 == 0);  // true
let all_positive = v.iter().all(|x| x > &0);  // true
```

**find()**

Returns first matching element:

```rust
let v = vec![1, 2, 3, 4, 5];
let first_even = v.iter().find(|x| x % 2 == 0);
// Some(&2)
```

**for_each()**

Performs closure on each element:

```rust
let v = vec![1, 2, 3];
v.iter().for_each(|x| println!("{}", x));
```

### Lazy Evaluation

A key feature of iterators is that they're lazy - computations don't happen until you consume:

```rust
let v = vec![1, 2, 3, 4, 5];

// This line does NOTHING
let iter = v.iter()
    .map(|x| {
        println!("mapping {}", x);  // Never prints
        x * 2
    })
    .filter(|x| x > &4);            // Never evaluates

// Now it executes:
let result: Vec<_> = iter.collect();
// Prints: mapping 1, mapping 2, mapping 3, mapping 4, mapping 5
```

This laziness is powerful because:
1. **Performance** - Only compute what's needed
2. **Chaining** - Compose operations efficiently
3. **Expressiveness** - Read like a pipeline

### Method Chaining Patterns

**Basic chain**

```rust
let result: Vec<i32> = vec![1, 2, 3, 4, 5]
    .iter()
    .map(|x| x * 2)
    .filter(|x| x > &4)
    .copied()
    .collect();
```

**Complex chain**

```rust
let result: Vec<String> = data
    .iter()
    .filter(|item| item.is_valid())
    .map(|item| item.format())
    .take(10)
    .collect();
```

**Using fold for accumulation**

```rust
let sum: i32 = vec![1, 2, 3, 4, 5]
    .iter()
    .fold(0, |acc, x| acc + x);
```

## Syntax

### Creating Iterators

```rust
v.iter()      // Borrow each element
v.iter_mut()  // Mutable borrow
v.into_iter() // Take ownership
```

### Adapter Methods

```rust
.map(|x| f(x))          // Transform
.filter(|x| predicate(x)) // Filter
.take(n)                // First n elements
.skip(n)                // Skip first n
.enumerate()            // With index
.zip(other)             // Pair with other
.rev()                  // Reverse
.fold(init, |a, x| f(a, x))  // Accumulate
.flat_map(f)            // Map then flatten
```

### Consumer Methods

```rust
.collect()              // Gather into collection
.sum()                  // Sum all
.product()              // Product all
.count()                // Count elements
.any(predicate)         // Any match?
.all(predicate)         // All match?
.find(predicate)        // First match
.for_each(f)            // Do for each
```

## Common Patterns

### Pattern 1: Transform data

```rust
let numbers = vec![1, 2, 3, 4, 5];
let doubled: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .collect();
```

### Pattern 2: Filter and transform

```rust
let numbers = vec![1, 2, 3, 4, 5];
let large_doubled: Vec<i32> = numbers
    .iter()
    .filter(|x| x > &2)
    .map(|x| x * 2)
    .collect();
```

### Pattern 3: Accumulate value

```rust
let numbers = vec![1, 2, 3, 4];
let sum = numbers.iter().fold(0, |acc, x| acc + x);
```

### Pattern 4: Find element

```rust
let numbers = vec![1, 2, 3, 4, 5];
let first_even = numbers
    .iter()
    .find(|x| x % 2 == 0)
    .copied();
```

### Pattern 5: Conditional checks

```rust
let numbers = vec![1, 2, 3, 4, 5];
let has_even = numbers.iter().any(|x| x % 2 == 0);
let all_positive = numbers.iter().all(|x| x > &0);
```

### Pattern 6: Group and process

```rust
let data = vec!["a1", "a2", "b1", "b2"];
let grouped: Vec<Vec<_>> = data
    .iter()
    .group_by(|item| item.chars().next())
    .into_iter()
    .collect();
```

### Pattern 7: Chaining with enumerate

```rust
let items = vec!["apple", "banana", "cherry"];
for (i, item) in items.iter().enumerate() {
    println!("{}: {}", i, item);
}
```

### Pattern 8: Multiple operations

```rust
let result: i32 = vec![1, 2, 3, 4, 5]
    .iter()
    .skip(1)
    .take(3)
    .map(|x| x * x)
    .sum();
// Sums squares of middle 3 elements
```

## Common Mistakes

### Mistake 1: Forgetting to consume

```rust
// ❌ Does nothing (iterator not consumed)
vec![1, 2, 3].iter().map(|x| x * 2);

// ✅ Actually executes
let v: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect();
```

### Mistake 2: Type confusion with references

```rust
// ❌ Type mismatch
let v = vec![1, 2, 3];
let sum: i32 = v.iter().sum();  // Error: expects &i32, not i32

// ✅ Use into_iter or copied
let sum: i32 = v.into_iter().sum();
// or
let sum: i32 = v.iter().copied().sum();
```

### Mistake 3: Ownership issues

```rust
// ❌ Error: v moved by into_iter
let v = vec![1, 2, 3];
let iter = v.into_iter();
println!("{:?}", v);  // Error

// ✅ Use iter to borrow
let iter = v.iter();
println!("{:?}", v);  // OK
```

### Mistake 4: Wrong iterator choice

```rust
// ❌ Inefficient: collecting when not needed
for x in vec![1, 2, 3].iter().map(|x| x * 2).collect::<Vec<_>>() {
    println!("{}", x);
}

// ✅ Efficient: iterator directly
for x in vec![1, 2, 3].iter().map(|x| x * 2) {
    println!("{}", x);
}
```

### Mistake 5: Closure variable capture

```rust
// ❌ May capture by reference unexpectedly
let threshold = 5;
let result: Vec<_> = vec![1, 2, 3, 4, 5]
    .iter()
    .filter(|x| x > &threshold)  // Captures by reference
    .copied()
    .collect();
```

## Real-World Examples

### Example 1: Data transformation pipeline

```rust
fn process_scores(scores: Vec<u32>) -> f64 {
    scores
        .iter()
        .filter(|s| s > &0)      // Remove invalid
        .map(|s| *s as f64)      // Convert to f64
        .fold(0.0, |acc, s| acc + s) / scores.len() as f64  // Average
}

let avg = process_scores(vec![85, 90, 78, 0, 92]);
```

### Example 2: Complex filtering

```rust
fn find_users(users: &[User], search: &str) -> Vec<String> {
    users
        .iter()
        .filter(|u| u.name.to_lowercase().contains(&search.to_lowercase()))
        .filter(|u| u.is_active)
        .map(|u| u.name.clone())
        .take(10)
        .collect()
}
```

### Example 3: Grouping and aggregation

```rust
use std::collections::HashMap;

fn group_by_category(items: Vec<Item>) -> HashMap<String, usize> {
    items
        .iter()
        .fold(HashMap::new(), |mut map, item| {
            *map.entry(item.category.clone()).or_insert(0) += 1;
            map
        })
}
```

### Example 4: Building a lookup

```rust
fn create_lookup(pairs: Vec<(String, i32)>) -> HashMap<String, i32> {
    pairs
        .into_iter()
        .fold(HashMap::new(), |mut map, (key, val)| {
            map.insert(key, val);
            map
        })
}
```

### Example 5: String processing

```rust
fn process_lines(text: &str) -> Vec<String> {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.to_uppercase())
        .collect()
}
```

## Related Concepts

### Prerequisites
- **Collections** - Iterators work on Vec, HashMap, etc.
- **Functions** - Closures in iterator methods
- **Ownership** - Different iterator types borrow differently

### What comes next
- **Traits** - Iterator trait implementation
- **Functional Programming** - Functional composition patterns
- **Error Handling** - Result types in iterators

### Cross-references
- Module 01: Closures (used in iterator methods)
- Module 02: Collections provide iterators
- Module 06: Trait implementations

## Best Practices

### Prefer iterators over explicit loops

```rust
// ✅ Idiomatic: use iterators
let doubled: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .collect();

// ❌ Less idiomatic: explicit loop
let mut doubled = Vec::new();
for x in &numbers {
    doubled.push(x * 2);
}
```

### Chain operations instead of intermediate collections

```rust
// ✅ Efficient: single pass
let result: i32 = data
    .iter()
    .filter(|x| x > &5)
    .map(|x| x * 2)
    .sum();

// ❌ Less efficient: intermediate Vec
let filtered: Vec<i32> = data
    .iter()
    .filter(|x| x > &5)
    .map(|x| x * 2)
    .collect();
let sum: i32 = filtered.iter().sum();
```

### Use appropriate consumer

```rust
// ✅ Use right consumer for the job
let count = data.iter().count();
let sum: i32 = data.iter().sum();
let has_match = data.iter().any(|x| x > &5);

// ❌ Unnecessary collect
let count = data.iter().collect::<Vec<_>>().len();
```

### Make iterator chains readable

```rust
// ✅ Clear pipeline
let result: Vec<i32> = numbers
    .iter()
    .filter(|x| x % 2 == 0)       // Even numbers
    .map(|x| x * x)               // Square them
    .take(5)                       // First 5
    .collected();

// ❌ Hard to follow
let result: Vec<i32> = numbers.iter().filter(|x| x%2==0).map(|x|x*x).take(5).collected();
```

## Summary

- **Iterators** are lazy sequences of elements
- **Three types**: `iter()` borrows, `iter_mut()` mutably borrows, `into_iter()` consumes
- **Adapters** transform (map, filter, etc.) and return new iterators
- **Consumers** execute and return final value (collect, sum, find, etc.)
- **Lazy evaluation** means computation only happens when consuming
- **Method chaining** creates expressive data transformation pipelines
- **Ownership**: iterators respect Rust's ownership rules

## Key Takeaways

1. Iterators enable functional, composable data processing
2. Lazy evaluation improves performance by avoiding unnecessary work
3. Method chaining creates readable transformation pipelines
4. Choose `iter()`, `iter_mut()`, or `into_iter()` based on needs
5. Always consume iterators (with collect, sum, for loop, etc.)
6. Iterators are more idiomatic than explicit loops in Rust
7. Closures in iterators capture their environment

## Practice Exercise Ideas

1. Filter and transform a Vec in one chain
2. Accumulate values using fold
3. Find first element matching a condition
4. Group elements by a property
5. Build a lookup table using iterators
6. Process text line by line
7. Combine multiple iterator operations

---

**Time to complete this concept**: 2-2.5 hours
**Difficulty**: Intermediate
**Prerequisite**: Collections, Closures, Ownership
**Next concept**: Common Traits

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
