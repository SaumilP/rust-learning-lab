# Key Takeaways: Collections

## Quick Reference

### Vec<T>

```rust
let v: Vec<i32> = Vec::new();  // Empty vector
let v = vec![1, 2, 3];          // With values
let mut v = Vec::new();
v.push(4);                       // Add to end
v.pop();                         // Remove from end
v[0];                            // Index access
v.get(0);                        // Safe access (Option)
```

### HashMap<K, V>

```rust
use std::collections::HashMap;

let mut m = HashMap::new();
m.insert("key", "value");        // Insert
m.get("key");                     // Get (Option)
m.remove("key");                  // Remove
m.contains_key("key");            // Check

// Efficient update pattern
*m.entry("key").or_insert(0) += 1;
```

### HashSet<T>

```rust
use std::collections::HashSet;

let mut s = HashSet::new();
s.insert(1);                      // Insert
s.contains(&1);                   // Contains
s.remove(&1);                     // Remove

// Set operations
s.union(&other);                  // All elements
s.intersection(&other);           // Common
s.difference(&other);             // In s but not other
```

### VecDeque<T>

```rust
use std::collections::VecDeque;

let mut dq = VecDeque::new();
dq.push_back(1);                  // Add to end
dq.push_front(0);                 // Add to front
dq.pop_back();                    // Remove from end
dq.pop_front();                   // Remove from front
```

## Essential Concepts

### 1. Collection Choice

| Collection | Use Case | Time Complexity |
|-----------|----------|-----------------|
| `Vec<T>` | Sequential data, indexing | O(1) access, O(1)* push, O(n) insert |
| `HashMap<K,V>` | Key-value lookups | O(1)* lookup, insert, remove |
| `HashSet<T>` | Unique values, membership | O(1)* insert, lookup, remove |
| `VecDeque<T>` | Queue/deque operations | O(1) front/back push/pop |

*Amortized average case, can be worse in worst case

### 2. Vector Iteration

```rust
// Borrow iterator - doesn't move
for item in &v { }

// Mutable borrow iterator
for item in &mut v {
    *item *= 2;
}

// Take ownership iterator
for item in v { }
```

### 3. HashMap Entry API (Key Pattern!)

```rust
// Inefficient: two lookups
if !map.contains_key(&key) {
    map.insert(key, 0);
}
map.get_mut(&key).unwrap() += 1;

// Efficient: one lookup
*map.entry(key).or_insert(0) += 1;
```

### 4. Type Requirements

- **HashMap keys** must implement `Hash` and `Eq`
- **HashSet values** must implement `Hash` and `Eq`
- Primitives (i32, String, etc.) already implement these
- Vec and String can't be HashMap keys!

### 5. Common Ownership Patterns

```rust
// Convert to vector from iterator
let v: Vec<i32> = (1..=5).collect();

// Convert to HashMap
let map: HashMap<_, _> = pairs.into_iter().collect();

// Convert to HashSet (deduplication)
let unique: HashSet<_> = vec.into_iter().collect();
```

## Common Patterns

### Pattern 1: Counting with HashMap
```rust
let mut counts = HashMap::new();
for word in words {
    *counts.entry(word).or_insert(0) += 1;
}
```

### Pattern 2: Deduplication with HashSet
```rust
let unique: HashSet<_> = numbers.into_iter().collect();
```

### Pattern 3: Building a lookup
```rust
let lookup: HashMap<String, Value> =
    items.into_iter()
         .map(|i| (i.key, i.value))
         .collect();
```

### Pattern 4: Queue with VecDeque
```rust
let mut queue = VecDeque::new();
queue.push_back(1);
while let Some(item) = queue.pop_front() {
    // process item
}
```

### Pattern 5: Collecting transformations
```rust
let doubled: Vec<i32> =
    numbers.iter()
           .map(|x| x * 2)
           .collect();
```

## Checklist: Which Collection?

**Use Vec when:**
- [ ] Need sequential access
- [ ] Need indexing by position
- [ ] Growing/shrinking list
- [ ] Cache-friendly iteration

**Use HashMap when:**
- [ ] Need key-value associations
- [ ] Fast lookup by key
- [ ] Flexible key types
- [ ] No ordering needed

**Use HashSet when:**
- [ ] Need unique values only
- [ ] Membership testing needed
- [ ] No key-value needed
- [ ] Deduplication required

**Use VecDeque when:**
- [ ] Need queue operations
- [ ] Efficient at both ends
- [ ] Push/pop from both sides
- [ ] FIFO/LIFO patterns

## Error Prevention

### ❌ DON'T: Index out of bounds
```rust
let v = vec![1, 2, 3];
let item = v[10];  // Panics!
```

### ✅ DO: Use get() for safety
```rust
let item = v.get(10);  // Returns Option, safe
```

### ❌ DON'T: Forget imports
```rust
let map = HashMap::new();  // ERROR
```

### ✅ DO: Import collections
```rust
use std::collections::HashMap;
let map = HashMap::new();  // OK
```

### ❌ DON'T: Use Vec as HashMap key
```rust
let mut map: HashMap<Vec<i32>, String> = HashMap::new();
map.insert(vec![1, 2], "value");  // ERROR
```

### ✅ DO: Use hashable types
```rust
let mut map: HashMap<String, String> = HashMap::new();
map.insert("key".to_string(), "value");  // OK
```

### ❌ DON'T: Inefficient HashMap updates
```rust
if !map.contains_key(&key) {
    map.insert(key.clone(), 0);
}
let count = map.get_mut(&key).unwrap();
*count += 1;  // Multiple lookups!
```

### ✅ DO: Use entry API
```rust
*map.entry(key).or_insert(0) += 1;  // One lookup
```

### ❌ DON'T: Lose data with ownership
```rust
let v = vec![1, 2, 3];
for item in v {  // Takes ownership
    println!("{}", item);
}
println!("{:?}", v);  // ERROR: v moved
```

### ✅ DO: Borrow when iterating
```rust
let v = vec![1, 2, 3];
for item in &v {  // Borrow
    println!("{}", item);
}
println!("{:?}", v);  // OK
```

## Performance Tips

### Vec Performance
- `push()` is O(1) amortized (occasional reallocation)
- `insert(0, x)` is O(n) - avoid for large vectors
- Use `Vec::with_capacity()` if you know size in advance
- Iteration with `iter()` is preferred over indexing loop

### HashMap Performance
- Average case O(1), worst case O(n)
- Use `entry()` API to avoid repeated lookups
- `clone()` keys only when necessary
- Consider using `&str` keys instead of `String` when possible

### HashSet Performance
- Same as HashMap performance characteristics
- Good for membership testing
- Set operations (union, intersection) are O(n) where n is collection size

### VecDeque Performance
- Both ends: O(1) push/pop
- Better than Vec for queue operations
- Not as cache-friendly as Vec for single-end access

## Memory Considerations

### Vector Memory
- Heap-allocated, contiguous
- `len()` - number of elements
- `capacity()` - allocated space
- `push()` allocates when `len == capacity`

### HashMap Memory
- Heap-allocated, non-contiguous
- Iteration order undefined
- Each entry takes: key + value + overhead
- Larger memory overhead than Vec for same elements

### HashSet Memory
- Similar to HashMap (actually uses HashMap internally)
- Memory for each value + set overhead
- No ordering guarantee

### VecDeque Memory
- Heap-allocated, circular buffer
- More complex than Vec
- Good for queue patterns
- Slightly more overhead than Vec

## Related Concepts

- **Ownership** - How collections manage memory
- **Iterators** - Advanced collection manipulation
- **Traits** - Hash, Eq, Ord traits for collections
- **Error Handling** - Option/Result from collection operations
- **Generics** - Collection type parameters

## Time Estimates

- Reading this takeaway: 10-15 minutes
- Reviewing patterns: 10 minutes
- Practice drills: 20-30 minutes
- Total: 40-55 minutes

## Practice Questions

1. When would you use Vec vs HashMap vs HashSet?
2. Why is entry API more efficient than contains_key + insert?
3. What makes a good HashMap key type?
4. How do you iterate a collection without moving it?
5. What's the difference between Vec and VecDeque?
6. How do set operations work?
7. When would you use with_capacity?
8. What's the complexity of HashMap lookup?

## Quick Decision Tree

```
Need to store multiple items?
├─ Yes, with key-value pairs?
│  └─ HashMap<K, V>
├─ Yes, only unique values?
│  └─ HashSet<T>
├─ Yes, sequential data?
│  └─ Vec<T>
└─ Yes, queue operations?
   └─ VecDeque<T>
```

## Real-World Scenarios

| Problem | Collection | Pattern |
|---------|-----------|---------|
| Count word frequencies | HashMap | entry().or_insert(0) |
| Remove duplicates | HashSet | collect() |
| Cache lookups | HashMap | get() / insert() |
| Message queue | VecDeque | push_back / pop_front |
| Index data | HashMap | key is index |
| Track membership | HashSet | contains() |
| Process in order | Vec | iter() / for loop |
| Undo/Redo | VecDeque | push/pop both ends |

---

**Status**: Quick reference guide
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Beginner-Intermediate
**Part of**: Module 02 - Standard Library
