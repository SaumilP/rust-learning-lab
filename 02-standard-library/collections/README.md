# Concept: Collections

## Overview

Collections are data structures that hold multiple values. While arrays and tuples hold a fixed number of values, collections can grow and shrink dynamically. Rust's standard library provides several useful collection types, each with different performance characteristics and use cases.

## Learning Objectives

By the end of this concept, you will understand:
- `Vec<T>` - dynamic arrays
- `HashMap<K, V>` - key-value maps
- `HashSet<T>` - unique value sets
- `VecDeque<T>` - double-ended queues
- When to use each collection type
- Basic operations on each collection

## Theory

### Vec<T> - Vector

A vector is a growable array stored on the heap. It's the most commonly used collection in Rust.

**Creating vectors**:

```rust
let v: Vec<i32> = Vec::new();  // Empty vector
let v = vec![1, 2, 3];          // Macro syntax with values
let mut v = Vec::new();
v.push(1);  // Need mut to modify
```

**Properties**:
- Generic over type `T`
- Contiguous heap allocation
- O(1) amortized push
- O(n) insert/remove in middle
- Indexable: `v[0]`, `v.get(0)`

**Common operations**:

```rust
let mut v = vec![1, 2, 3];
v.push(4);              // Add to end
v.pop();                // Remove from end
v.len();                // Length
v.is_empty();           // Check empty
v[0];                   // Index access (panics if out of bounds)
v.get(0);               // Safe access returns Option
v.remove(0);            // Remove at index
v.clear();              // Remove all
```

**Iteration**:

```rust
for item in &v {              // Borrow each item
    println!("{}", item);
}

for item in &mut v {          // Mutable borrow
    *item *= 2;
}

for item in v {               // Take ownership
    println!("{}", item);     // v no longer usable
}

v.iter()    // Borrow iterator
v.iter_mut() // Mutable borrow iterator
v.into_iter() // Take ownership iterator
```

### HashMap<K, V>

A hash map stores key-value pairs. It uses hashing to find values quickly by key.

**Creating hashmaps**:

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("name", "Alice");
map.insert("age", "30");

// From iterator
let pairs = vec![("a", 1), ("b", 2)];
let map: HashMap<_, _> = pairs.into_iter().collect();
```

**Properties**:
- Fast lookups: O(1) average
- No ordering guarantee
- Generic over K and V types
- K must implement Hash and Eq traits

**Common operations**:

```rust
let mut map = HashMap::new();
map.insert("key", "value");     // Insert/update
map.get("key");                 // Returns Option
map.get_mut("key");             // Mutable reference
map.remove("key");              // Remove and return
map.contains_key("key");        // Check existence
map.len();                       // Number of pairs
map.is_empty();                  // Check empty
map.clear();                     // Remove all
```

**Iteration**:

```rust
for (key, value) in &map {
    println!("{}: {}", key, value);
}

for (k, v) in map.iter_mut() {
    *v *= 2;
}
```

**Entry API** (efficient update):

```rust
map.entry("key")
    .or_insert(0)     // Insert if not present
    .add_assign(1);   // Modify value

// Count occurrences
for word in words {
    *map.entry(word).or_insert(0) += 1;
}
```

### HashSet<T>

A set of unique values. Uses hashing for fast membership testing.

**Creating sets**:

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(1);
set.insert(2);
set.insert(1);  // Duplicate, no effect

let set: HashSet<_> = vec![1, 2, 2, 3].into_iter().collect();
```

**Properties**:
- Only unique values
- No ordering guarantee
- O(1) insertion and lookup
- T must implement Hash and Eq

**Common operations**:

```rust
let mut set = HashSet::new();
set.insert(1);          // Returns true if new
set.contains(&1);       // Check membership
set.remove(&1);         // Remove
set.len();              // Number of elements
set.is_empty();         // Check empty

// Set operations
let s1: HashSet<_> = vec![1, 2, 3].into_iter().collect();
let s2: HashSet<_> = vec![2, 3, 4].into_iter().collect();

s1.union(&s2);          // All elements
s1.intersection(&s2);   // Common elements
s1.difference(&s2);     // In s1 but not s2
```

### VecDeque<T>

A double-ended queue - efficient insertion/removal at both ends.

**Creating deques**:

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
deque.push_back(1);     // Add to end
deque.push_front(0);    // Add to front
```

**Properties**:
- Efficient at both ends
- O(1) push_front/push_back
- O(1) pop_front/pop_back
- Good for queues and double-ended work

**Common operations**:

```rust
let mut deque = VecDeque::new();
deque.push_back(1);     // Add to back
deque.push_front(0);    // Add to front
deque.pop_back();       // Remove from back
deque.pop_front();      // Remove from front
deque.len();            // Size
deque.front();          // View front
deque.back();           // View back
```

## Syntax

### Vector Syntax

```rust
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3];
let v: Vec<i32> = (1..=3).collect();

v.push(4);
v.pop();
v[0];
v.get(0)?;
for item in v.iter() { }
```

### HashMap Syntax

```rust
use std::collections::HashMap;

let mut m = HashMap::new();
m.insert(key, value);
m.get(&key);
m.get_mut(&key);
m.remove(&key);

for (k, v) in &m { }
m.entry(k).or_insert(default);
```

### HashSet Syntax

```rust
use std::collections::HashSet;

let mut s = HashSet::new();
s.insert(value);
s.contains(&value);
s.remove(&value);

for item in s.iter() { }
s.union(&other);
```

### VecDeque Syntax

```rust
use std::collections::VecDeque;

let mut dq = VecDeque::new();
dq.push_front(val);
dq.push_back(val);
dq.pop_front();
dq.pop_back();
```

## Common Patterns

### Pattern 1: Counting occurrences

```rust
use std::collections::HashMap;

let words = vec!["hello", "world", "hello"];
let mut counts = HashMap::new();

for word in words {
    *counts.entry(word).or_insert(0) += 1;
}

println!("{:?}", counts);  // {"hello": 2, "world": 1}
```

### Pattern 2: Deduplication with HashSet

```rust
use std::collections::HashSet;

let numbers = vec![1, 2, 2, 3, 3, 3];
let unique: HashSet<_> = numbers.into_iter().collect();
let unique: Vec<_> = unique.into_iter().collect();

println!("{:?}", unique);  // [1, 2, 3]
```

### Pattern 3: Collecting into collection

```rust
let numbers: Vec<i32> = (1..=5).collect();
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

let chars: HashSet<char> = "hello".chars().collect();
```

### Pattern 4: Building a lookup

```rust
use std::collections::HashMap;

let people = vec![
    ("alice", 30),
    ("bob", 25),
];

let ages: HashMap<&str, u32> = people.into_iter().collect();

if let Some(age) = ages.get("alice") {
    println!("Alice is {}", age);
}
```

### Pattern 5: Queue operations with VecDeque

```rust
use std::collections::VecDeque;

let mut queue = VecDeque::new();
queue.push_back(1);
queue.push_back(2);
queue.push_back(3);

while let Some(item) = queue.pop_front() {
    println!("{}", item);  // Prints 1, 2, 3
}
```

## Common Mistakes

### Mistake 1: Forgetting imports

```rust
// ❌ ERROR: HashMap not in scope
let mut m = HashMap::new();

// ✅ CORRECT
use std::collections::HashMap;
let mut m = HashMap::new();
```

### Mistake 2: Index out of bounds panic

```rust
let v = vec![1, 2, 3];
let item = v[10];  // ❌ Panics!

let item = v.get(10);  // ✅ Returns None
```

### Mistake 3: Ownership with iteration

```rust
let v = vec![1, 2, 3];
for item in v {        // Takes ownership
    println!("{}", item);
}
println!("{:?}", v);   // ❌ ERROR: v moved

// ✅ CORRECT: borrow instead
for item in &v {
    println!("{}", item);
}
println!("{:?}", v);   // OK
```

### Mistake 4: HashMap key not hashable

```rust
use std::collections::HashMap;

let mut m: HashMap<Vec<i32>, i32> = HashMap::new();
m.insert(vec![1, 2], 10);  // Vec can't be key!

// ✅ Use hashable types or implement Hash
let mut m: HashMap<String, i32> = HashMap::new();
m.insert("key".to_string(), 10);  // OK
```

### Mistake 5: Mutable iteration

```rust
let v = vec![1, 2, 3];
for item in &v {
    *item = 0;  // ❌ ERROR: immutable borrow
}

// ✅ CORRECT
for item in &mut v {
    *item = 0;  // OK
}
```

## Real-World Examples

### Example 1: Word frequency analysis

```rust
use std::collections::HashMap;

fn analyze_text(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();

    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(&word).or_insert(0) += 1;
    }

    counts
}

let analysis = analyze_text("hello world hello");
// {"hello": 2, "world": 1}
```

### Example 2: Grouping by category

```rust
use std::collections::HashMap;

#[derive(Clone)]
struct Item {
    name: String,
    category: String,
}

fn group_items(items: Vec<Item>) -> HashMap<String, Vec<String>> {
    let mut groups = HashMap::new();

    for item in items {
        groups.entry(item.category)
            .or_insert_with(Vec::new)
            .push(item.name);
    }

    groups
}
```

### Example 3: LRU cache concepts

```rust
use std::collections::HashMap;

struct Cache {
    data: HashMap<String, String>,
    max_size: usize,
}

impl Cache {
    fn new(max_size: usize) -> Self {
        Cache {
            data: HashMap::new(),
            max_size,
        }
    }

    fn set(&mut self, key: String, value: String) {
        if self.data.len() >= self.max_size {
            // Simple: just clear when full
            // Real LRU would track access order
            self.data.clear();
        }
        self.data.insert(key, value);
    }
}
```

## Related Concepts

### Prerequisites
- **Variables & Mutability** - Mut needed for collection modification
- **Data Types** - Generic types and type parameters
- **Functions** - Collection operations are method calls

### What comes next
- **Iterator Patterns** - Iterating and transforming collections
- **Error Handling** - Option/Result from collection operations
- **Ownership** - Deep understanding of borrowing with collections

### Cross-references
- Module 02: Iterators work on collections
- Module 03: Testing collection operations
- Challenge problems: Many use collections

## Best Practices

### Choose the right collection

```rust
// Use Vec when you need:
// - Sequential access
// - Efficient random access
// - Growing size
let mut v: Vec<i32> = vec![];

// Use HashMap when you need:
// - Key-value association
// - Fast lookup by key
let mut map: HashMap<String, i32> = HashMap::new();

// Use HashSet when you need:
// - Unique values only
// - Fast membership testing
let mut set: HashSet<i32> = HashSet::new();

// Use VecDeque when you need:
// - Queue/deque operations
// - Efficient operations at both ends
let mut dq: VecDeque<i32> = VecDeque::new();
```

### Use iterators instead of indexing

```rust
// ❌ Less efficient
for i in 0..v.len() {
    println!("{}", v[i]);
}

// ✅ Better
for item in &v {
    println!("{}", item);
}

// ✅ Even better with transformations
v.iter().filter(|x| x > &5).for_each(|x| println!("{}", x));
```

### Prefer entry API for updates

```rust
use std::collections::HashMap;

// ❌ Less efficient (two lookups)
if !map.contains_key(&key) {
    map.insert(key, 0);
}
map.get_mut(&key).unwrap() += 1;

// ✅ More efficient (one lookup)
*map.entry(key).or_insert(0) += 1;
```

## Summary

- **Vec<T>** - Dynamic array, most common
- **HashMap<K, V>** - Key-value storage
- **HashSet<T>** - Unique values
- **VecDeque<T>** - Double-ended queue
- Each has different performance characteristics
- Choose based on your access patterns
- Entry API for efficient HashMap updates

## Key Takeaways

1. Collections store multiple values dynamically
2. Vec is for sequential data
3. HashMap is for key-value lookups
4. HashSet removes duplicates automatically
5. VecDeque is for queue operations
6. Understand borrowing with collections
7. Iterators are preferred over indexing

## Practice Exercise Ideas

1. Count word frequencies with HashMap
2. Find duplicates using HashSet
3. Implement a simple cache
4. Queue simulation with VecDeque
5. Group data by category

---

**Time to complete this concept**: 1.5-2 hours
**Difficulty**: Beginner-Intermediate
**Prerequisite**: Module 01 concepts
**Next concept**: String Operations

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
