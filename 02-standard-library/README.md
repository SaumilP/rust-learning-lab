# Module 02: Standard Library Essentials

Welcome to Standard Library Essentials! This module explores Rust's powerful standard library that provides collections, string operations, iterators, traits, and more.

## Learning Objectives

By completing this module, you will understand:
- Rust's collection types and when to use each
- String operations and differences between String and &str
- Iterator patterns and functional programming
- Common traits and how to use them
- Error handling with Option and Result

## Module Structure

### 1. Collections
**Concepts**:
- `Vec<T>` - Dynamic arrays
- `HashMap<K, V>` - Key-value storage
- `HashSet<T>` - Unique values
- `VecDeque<T>` - Double-ended queue
- Creating, inserting, removing, iterating

**Time**: 1.5-2 hours
**Prerequisite**: Module 01 (Core Fundamentals)

**Key Files**:
- `collections/README.md` - Detailed explanation
- `collections/examples/` - Working code examples
- `collections/key_takeaways.md` - Quick reference

---

### 2. String Operations
**Concepts**:
- String vs &str differences
- String methods and operations
- Parsing and formatting
- String slicing
- Pattern matching on strings

**Time**: 1-1.5 hours
**Prerequisite**: Module 01 (Data Types)

**Key Files**:
- `string_operations/README.md` - Detailed explanation
- `string_operations/examples/` - Working code examples
- `string_operations/key_takeaways.md` - Quick reference

---

### 3. Iterator Patterns
**Concepts**:
- `.iter()`, `.into_iter()`, `.iter_mut()`
- Iterator methods: `map()`, `filter()`, `fold()`
- Method chaining
- Collecting into collections
- Custom iterators basics

**Time**: 1.5-2 hours
**Prerequisite**: Collections, Module 01

**Key Files**:
- `iterator_patterns/README.md` - Detailed explanation
- `iterator_patterns/examples/` - Working code examples
- `iterator_patterns/key_takeaways.md` - Quick reference

---

### 4. Common Traits
**Concepts**:
- `Clone` and `Copy` traits
- `Default` trait
- `ToString` and `FromStr` traits
- `Debug` and `Display` traits
- Implementing common traits

**Time**: 1-1.5 hours
**Prerequisite**: Module 01

**Key Files**:
- `common_traits/README.md` - Detailed explanation
- `common_traits/examples/` - Working code examples
- `common_traits/key_takeaways.md` - Quick reference

---

### 5. Error Handling Basics
**Concepts**:
- `Option<T>` type and operations
- `Result<T, E>` type and operations
- `unwrap()`, `expect()`, `?` operator
- Pattern matching on Option/Result
- Custom error types basics

**Time**: 1.5-2 hours
**Prerequisite**: Module 01 (Control Flow)

**Key Files**:
- `error_handling_basics/README.md` - Detailed explanation
- `error_handling_basics/examples/` - Working code examples
- `error_handling_basics/key_takeaways.md` - Quick reference

---

## Exercises

Practice collections, iterators, and error handling:

### Exercise 1: Collection Manipulation
**Concepts Tested**: Collections, Insertion, Removal, Iteration
**Difficulty**: Easy

Work with Vec, HashMap, and HashSet; add, remove, and iterate elements.

### Exercise 2: Iterator Chain Puzzle
**Concepts Tested**: Iterators, `map()`, `filter()`, Functional Composition
**Difficulty**: Easy-Medium

Build iterator chains to transform and filter data elegantly.

### Exercise 3: Parse & Validate
**Concepts Tested**: Option, Result, Error Handling
**Difficulty**: Medium

Parse input and validate it, handling errors gracefully.

### Exercise 4: Find Duplicates
**Concepts Tested**: Collections, Iterators, Logic
**Difficulty**: Medium

Identify duplicate elements using appropriate data structures.

---

## Learning Path

### Recommended Order
1. Start with **Collections** (what to store)
2. Learn **String Operations** (working with text)
3. Study **Iterator Patterns** (processing data)
4. Master **Common Traits** (using library abstractions)
5. Understand **Error Handling Basics** (handling failures)
6. Complete exercises in order

### Time Estimate
- Concepts: 7-10 hours
- Exercises: 3-4 hours
- Total: 10-14 hours for complete mastery

### Progression Tips
- Experiment with different collection types
- Understand iterator adapters vs consumers
- Practice with Option and Result
- Use iterators instead of loops when possible

---

## Prerequisites

Before starting this module:
- Complete Module 01 (Core Fundamentals)
- Understand variables, functions, and control flow
- Be familiar with type systems basics

### Quick Review
```bash
# Test your Module 01 knowledge
cd challenges/kata/
# Solve a few string manipulation and number operation problems
```

---

## Related Concepts

### Foundation
- **Module 01**: Variables, functions, and control flow
- **Module 03**: Testing collections and error handling

### Next Steps
- **Module 04**: Using these tools in real programs
- **Module 06**: Advanced features with generics and traits
- **Challenges**: `/challenges/beginner/` for practice

---

## Common Patterns

### Working with Vec
```rust
let mut v = vec![1, 2, 3];
v.push(4);
v.iter().filter(|x| x > &2).map(|x| x * 2).collect::<Vec<_>>()
```

### Working with HashMap
```rust
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");
map.get("key")
```

### Error Handling
```rust
let result: Result<i32, _> = "42".parse();
match result {
    Ok(n) => println!("Parsed: {}", n),
    Err(e) => println!("Error: {}", e),
}
```

### Iterators
```rust
(1..=5)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .collect::<Vec<_>>()
```

---

## Cheat Sheet

### Collections
| Type | Use Case | Example |
|------|----------|---------|
| Vec<T> | Ordered, growable list | `vec![1, 2, 3]` |
| HashMap | Key-value pairs | `map.insert(key, val)` |
| HashSet | Unique values | `set.insert(value)` |
| VecDeque | Double-ended queue | `deque.push_front(val)` |

### String Operations
| Operation | Syntax | Returns |
|-----------|--------|---------|
| Length | `s.len()` | usize |
| Iterate chars | `s.chars()` | Iterator<char> |
| Split | `s.split(',')` | Iterator<&str> |
| To uppercase | `s.to_uppercase()` | String |
| Contains | `s.contains("str")` | bool |

### Error Handling
| Method | Purpose | When to Use |
|--------|---------|------------|
| `unwrap()` | Extract value or panic | Testing only |
| `expect(msg)` | Extract with message | Rare cases |
| `?` operator | Propagate error | Most common |
| Match pattern | Handle both cases | Explicit handling |

---

## Resources

### Official Documentation
- [The Rust Book - Chapter 8](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Rust by Example - Collections](https://doc.rust-lang.org/rust-by-example/std.html)
- [Standard Library Docs](https://doc.rust-lang.org/std/)

### Collections
- [Vec documentation](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [HashMap documentation](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Iterator documentation](https://doc.rust-lang.org/std/iter/)

### Learning Resources
- [Iterators blog post](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

## Module Status

-  Structure established
- ó Concept documentation in progress
- ó Code examples needed
- ó Exercises pending

## Next Steps

1. Read through each concept README
2. Run and modify all examples
3. Complete exercises
4. Move to Module 03 or practice with challenges

---

**Last Updated**: 2026-08-27
**Estimated Completion**: Phase 3-4
**Questions?** Refer to the official Rust Book or community resources.
