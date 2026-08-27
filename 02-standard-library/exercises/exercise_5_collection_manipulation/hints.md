# Hints for Exercise 5: Collection Manipulation

## Stuck? Here are some hints:

### About the bugs:

**Bug 1 & 2: Iterator Consumption with into_iter()**
- `into_iter()` consumes the collection - it takes ownership
- After `numbers.into_iter()`, the vector `numbers` no longer exists
- The next line tries to use `numbers` again, causing: "value used after move"
- Solution: Use `.iter()` instead of `.into_iter()` if you want to keep using the original
- Difference:
  - `.iter()` returns references, doesn't consume, can be called multiple times
  - `.into_iter()` consumes the vector, can only be called once
  - `.iter_mut()` returns mutable references, doesn't consume
- Fix: Change `numbers.into_iter()` to `numbers.iter()`
- Example:
  ```rust
  let evens: Vec<i32> = numbers.iter()  // Borrow, don't consume
      .filter(|&&n| n % 2 == 0)
      .collect();
  println!("Even numbers: {:?}", evens);

  // Now 'numbers' is still available
  let doubled: Vec<i32> = numbers.iter()  // Can use again
      .map(|n| n * 2)
      .collect();
  ```

**Bug 3: Inefficient HashMap Counting**
- The code works but is inefficient
- Using `.contains_key()` then indexing requires multiple lookups
- Better approach: Use the `.entry()` API for a single lookup
- The entry API looks up the key once and returns an Entry enum
- Use `.or_insert()` to set default if missing, then modify
- Fix: Replace the if/else with:
  ```rust
  *word_count.entry(word).or_insert(0) += 1;
  ```
- This is the idiomatic Rust pattern for counting
- The asterisk `*` dereferences the mutable reference to modify the value

**Bug 4: Partition Tuple Order**
- `partition()` takes a closure that returns bool
- Returns a tuple `(true_items, false_items)` - order depends on the closure
- In the code: `.partition(|&&n| n % 2 == 0)` returns (even, odd) because 0 is even
- But variables are named `(odd, even)` - they're backwards!
- When you iterate `odd`, you're actually iterating even numbers
- When you iterate `even`, you're actually iterating odd numbers
- Fix: Swap the variable names in the tuple unpacking:
  ```rust
  let (even, odd): (Vec<_>, Vec<_>) = numbers.iter()
      .partition(|&&n| n % 2 == 0);
  ```
- OR keep current names and flip the condition:
  ```rust
  let (odd, even): (Vec<_>, Vec<_>) = numbers.iter()
      .partition(|&&n| n % 2 != 0);  // n % 2 != 0 makes odd numbers true
  ```

### Testing your fix:

After fixing the bugs, run:
```bash
cargo run
```

You should see:
- Vector operations producing correct results
- All values used correctly without ownership errors
- Word counts matching expected frequencies
- Even and odd numbers separated correctly

### Debugging tips:

1. Look for "value used after move" - usually means into_iter() when you should use iter()
2. Look for "cannot index into map" - means key doesn't exist when expected
3. Count collections: Are even numbers [2, 4, 8]? Or wrong?
4. Check word counts: Should be 2 each for "rust" and "is"
5. Test with different numbers to verify grouping logic

### Key concepts to remember:

- **Iterators don't own**: `.iter()` borrows, `.into_iter()` owns
- **Partition returns tuple**: `(true_values, false_values)` based on closure condition
- **Entry API efficiency**: Single lookup instead of contains_key + indexing
- **Reference dereferencing**: `*entry` in `*entry.or_insert(0) += 1`
- **Vector vs Iterator**: Need `.collect()` to get Vec from Iterator chain
- **Method chaining**: Can chain filter, map, partition, collect

### Iterator consumption patterns:

```rust
// Pattern 1: Iterate but keep original
for item in collection.iter() { }  // Still have collection after loop

// Pattern 2: Transform into new collection
let new: Vec<_> = collection.iter().map(...).collect();  // Still have collection

// Pattern 3: Consume original (rarely needed)
for item in collection.into_iter() { }  // collection is gone after loop
```

### Entry API pattern:

```rust
// Instead of:
if map.contains_key(&key) {
    map.insert(key, map[&key] + 1);
} else {
    map.insert(key, 1);
}

// Use:
*map.entry(key).or_insert(0) += 1;
```

### If still stuck:

1. **Ownership error**: Try changing `into_iter()` to `.iter()`
2. **Wrong grouping**: Check if variable names match the condition
3. **Inefficient counting**: Use `.entry().or_insert()` pattern
4. **Type errors**: Ensure you're dereferencing when needed (`**` or `&n`)

The fixes are usually 2-4 line changes!

### Related concepts:

- **Ownership rules**: Critical for understanding iterator consumption
- **Partition algorithm**: Splits collections based on predicate
- **HashMap optimization**: Entry API vs direct access
- **Iterator adapters**: filter, map, partition all work on iterators
- **Closure types**: `|x|`, `|&x|`, `|&x, y|` have different meanings

