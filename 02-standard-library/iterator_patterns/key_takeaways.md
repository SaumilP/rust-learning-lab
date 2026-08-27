# Key Takeaways: Iterator Patterns

## Quick Reference

### Creating Iterators

```rust
v.iter()          // Borrow: &T
v.iter_mut()      // Mutable borrow: &mut T
v.into_iter()     // Take ownership: T
```

### Common Adapters (return new iterator)

```rust
.map(|x| f(x))              // Transform each
.filter(|x| predicate(x))   // Keep matching
.take(n)                    // First n elements
.skip(n)                    // Skip first n
.enumerate()                // Index + value
.zip(other)                 // Pair with other
.rev()                      // Reverse
.flatten()                  // Flatten nested
.flat_map(|x| f(x))         // Map + flatten
```

### Common Consumers (execute immediately)

```rust
.collect()                  // Gather into collection
.sum()                      // Sum all
.product()                  // Multiply all
.count()                    // Count elements
.any(predicate)             // Any match?
.all(predicate)             // All match?
.find(predicate)            // First match
.fold(init, |a, x| f(a, x)) // Accumulate
.for_each(|x| f(x))         // Do for each
```

## Essential Concepts

### 1. Three Iterator Types

| Type | Borrows | Mutable | Consumes | Example |
|------|---------|---------|----------|---------|
| `iter()` | Yes | No | No | `v.iter()` |
| `iter_mut()` | Yes | Yes | No | `v.iter_mut()` |
| `into_iter()` | No | N/A | Yes | `v.into_iter()` |

**When to use:**
- `iter()` - Default, when you just need to read
- `iter_mut()` - Need to modify elements
- `into_iter()` - Need ownership or consuming loop

### 2. Adapters vs Consumers

```rust
// ADAPTER - returns new iterator (lazy, doesn't execute)
v.iter().map(|x| x * 2)

// CONSUMER - executes immediately (returns final value)
v.iter().map(|x| x * 2).collect()
```

**Key insight**: Chain adapters, end with consumer

```rust
v.iter()                // Create iterator
  .map(|x| x * 2)      // Adapter
  .filter(|x| x > &4)  // Adapter
  .collect()           // Consumer (now executes)
```

### 3. Lazy Evaluation

```rust
// This line does NOTHING
let iter = vec![1,2,3].iter().map(|x| {
    println!("processing"); // Never prints
    x * 2
});

// This line executes the chain
for item in iter {
    println!("{}", item);  // Now prints
}
```

**Benefits:**
- Only compute needed elements
- Compose operations efficiently
- Avoid unnecessary allocations

### 4. Closure Syntax in Iterators

```rust
// Single line, no braces
.map(|x| x * 2)

// Multiple lines, needs braces
.map(|x| {
    let doubled = x * 2;
    doubled
})

// With condition
.filter(|x| x > &5)
.filter(|x| x % 2 == 0)
```

### 5. Common Consumer Patterns

```rust
// Gather into collection
let v: Vec<i32> = iter.collect();

// Sum/Product
let sum: i32 = iter.sum();
let product: i32 = iter.product();

// Count/Find
let count = iter.count();
let first = iter.find(|x| x > &5);  // Returns Option

// Conditions
let has_even = iter.any(|x| x % 2 == 0);
let all_positive = iter.all(|x| x > &0);

// Accumulate
let sum = iter.fold(0, |acc, x| acc + x);
```

## Common Patterns

### Pattern 1: Transform (map)
```rust
let doubled: Vec<i32> = v.iter()
    .map(|x| x * 2)
    .collect();
```

### Pattern 2: Filter + Transform
```rust
let result: Vec<i32> = v.iter()
    .filter(|x| x > &2)
    .map(|x| x * x)
    .collect();
```

### Pattern 3: Accumulate (fold)
```rust
let sum = v.iter()
    .fold(0, |acc, x| acc + x);
```

### Pattern 4: Find
```rust
let first_even = v.iter()
    .find(|x| x % 2 == 0)
    .copied();  // Convert Option<&i32> to Option<i32>
```

### Pattern 5: Conditional check
```rust
let has_even = v.iter().any(|x| x % 2 == 0);
let all_positive = v.iter().all(|x| x > &0);
```

### Pattern 6: With enumerate
```rust
for (i, item) in v.iter().enumerate() {
    println!("{}: {}", i, item);
}
```

### Pattern 7: Multiple operations
```rust
let result: i32 = v.iter()
    .skip(1)
    .take(3)
    .map(|x| x * x)
    .sum();
```

## Checklist: Iterator Decisions

**Use iter():**
- [ ] Only need to read elements
- [ ] Want to keep collection usable
- [ ] Don't need ownership

**Use iter_mut():**
- [ ] Need to modify elements
- [ ] Keep collection afterward
- [ ] Don't need ownership

**Use into_iter():**
- [ ] Need ownership of elements
- [ ] Collection not needed after
- [ ] Discarding collection

**Chain adapters:**
- [ ] Start with iterator creation
- [ ] Add adapters (map, filter, etc.)
- [ ] End with consumer

## Error Prevention

### ❌ DON'T: Forget to consume
```rust
vec![1, 2, 3].iter().map(|x| x * 2);  // Does nothing!
```

### ✅ DO: End with consumer
```rust
let v: Vec<i32> = vec![1, 2, 3]
    .iter()
    .map(|x| x * 2)
    .collect();
```

### ❌ DON'T: Type confusion
```rust
let sum: i32 = vec![1, 2, 3].iter().sum();  // Type error!
```

### ✅ DO: Handle references
```rust
let sum: i32 = vec![1, 2, 3]
    .iter()
    .copied()      // Convert &i32 to i32
    .sum();
// OR
let sum: i32 = vec![1, 2, 3]
    .into_iter()   // Take ownership
    .sum();
```

### ❌ DON'T: Move when borrowing
```rust
let v = vec![1, 2, 3];
let iter = v.into_iter();
println!("{:?}", v);  // ERROR: v moved
```

### ✅ DO: Borrow when needed later
```rust
let v = vec![1, 2, 3];
let iter = v.iter();
println!("{:?}", v);  // OK: v borrowed
```

### ❌ DON'T: Unnecessary collect
```rust
// Only need to iterate, not collect
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
for item in doubled { }  // Extra allocation!
```

### ✅ DO: Iterator directly when possible
```rust
// No intermediate Vec needed
for item in v.iter().map(|x| x * 2) { }
```

## Method Quick Reference

| Method | Type | Returns | Purpose |
|--------|------|---------|---------|
| `map(f)` | Adapter | Iterator | Transform each |
| `filter(p)` | Adapter | Iterator | Keep matching |
| `take(n)` | Adapter | Iterator | First n |
| `skip(n)` | Adapter | Iterator | Skip first n |
| `enumerate()` | Adapter | Iterator | With index |
| `zip(other)` | Adapter | Iterator | Pair with other |
| `rev()` | Adapter | Iterator | Reverse |
| `flatten()` | Adapter | Iterator | Flatten nested |
| `flat_map(f)` | Adapter | Iterator | Map + flatten |
| `collect()` | Consumer | Collection | Gather into |
| `sum()` | Consumer | Value | Sum all |
| `product()` | Consumer | Value | Product all |
| `count()` | Consumer | usize | Count elements |
| `any(p)` | Consumer | bool | Any match? |
| `all(p)` | Consumer | bool | All match? |
| `find(p)` | Consumer | Option | First match |
| `fold(i, f)` | Consumer | Value | Accumulate |
| `for_each(f)` | Consumer | () | Do for each |

## Performance Tips

### Lazy Evaluation Benefit
```rust
// Only computes until find returns
let first_even = v.iter()
    .map(|x| expensive_fn(x))
    .find(|x| x % 2 == 0);
// Stops as soon as first even found
```

### Avoid Unnecessary Collect
```rust
// ✅ Good: no intermediate Vec
for item in v.iter().map(|x| x * 2).filter(|x| x > &4) { }

// ❌ Bad: extra allocation
let intermediate: Vec<i32> = v.iter().map(|x| x * 2).collect();
for item in intermediate.iter().filter(|x| x > &4) { }
```

### Use copied() or cloned()
```rust
// ✅ When working with &T in iterator
let v: Vec<i32> = v.iter().copied().collect();

// ❌ More verbose
let v: Vec<i32> = v.iter().map(|x| *x).collect();
```

## Related Concepts

- **Collections** - Provide iterators
- **Closures** - Used in iterator methods
- **Ownership** - Different iterator types
- **Traits** - Iterator trait implementation

## Time Estimates

- Reading this takeaway: 10-15 minutes
- Reviewing patterns: 10 minutes
- Practice drills: 20-30 minutes
- Total: 40-55 minutes

## Practice Questions

1. What's the difference between iter(), iter_mut(), and into_iter()?
2. What's an adapter vs a consumer?
3. Why are iterators lazy?
4. When would you use map vs filter?
5. How do you accumulate a value with an iterator?
6. What's the difference between find and filter?
7. When should you avoid using collect()?
8. How do you handle references in iterators?

## Cheat Sheet: Common Chains

**Double all elements:**
```rust
v.iter().map(|x| x * 2).collect()
```

**Keep only even numbers:**
```rust
v.iter().filter(|x| x % 2 == 0).copied().collect()
```

**Sum of squares:**
```rust
v.iter().map(|x| x * x).sum()
```

**Find first matching:**
```rust
v.iter().find(|x| x > &5).copied()
```

**Group and count:**
```rust
v.iter().fold(HashMap::new(), |mut m, x| {
    *m.entry(x).or_insert(0) += 1;
    m
})
```

**Process with index:**
```rust
for (i, item) in v.iter().enumerate() { }
```

**Skip, take, transform:**
```rust
v.iter().skip(2).take(3).map(|x| x * 2).collect()
```

**Check conditions:**
```rust
v.iter().any(|x| x > &5)  // Any greater than 5?
v.iter().all(|x| x > &0)  // All positive?
```

---

**Status**: Quick reference guide
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Intermediate
**Part of**: Module 02 - Standard Library
