# Hints for Exercise 1: Collection Manipulation

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: Vector Mutability**
- Vectors need to be mutable (`mut`) to modify them
- Operations like `push()`, `pop()`, and `remove()` require `&mut self`
- Fix: Change `let numbers = ...` to `let mut numbers = ...`
- Example:
  ```rust
  let mut numbers = vec![1, 2, 3];
  numbers.push(4);  // Now this works
  ```

**Bug 2: Filter Closure Reference**
- When you call `.iter()` on a Vec<i32>, you get an iterator over &i32
- In the filter closure `|x|`, x is actually `&&i32` (reference to reference)
- You need to dereference to do arithmetic: `|x| *x % 2 == 0`
- Or use pattern matching: `|&&x| x % 2 == 0`
- Also, `.collect()` needs proper type annotation or will collect references
- Fix: `let even: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();`

**Bug 3: Entry API Counting**
- `entry().or_insert(0)` returns `&mut V` (mutable reference to value)
- The expression `count + 1` creates a new value but doesn't modify count
- You need to use `*count += 1` to modify the value through the reference
- Example:
  ```rust
  let count = word_counts.entry(word).or_insert(0);
  *count += 1;  // Dereference and increment
  ```

**Bug 4: Handling Option from max_by_key**
- `max_by_key()` returns `Option<(&K, &V)>`, not the tuple directly
- Accessing `.0` on an Option will cause a compiler error
- Use `.unwrap()` to get the value (if you're sure it exists)
- Or use `if let Some((word, count)) = most_common { ... }`
- Example:
  ```rust
  let most_common = word_counts.iter().max_by_key(|(_, count)| *count).unwrap();
  println!("Most common: {} ({} times)", most_common.0, most_common.1);
  ```

### Testing your fix:

After fixing the bugs, run:
```bash
cargo run
```

You should see:
- Vector operations completing without errors
- Word counts correctly showing 3 for "the"
- Statistics calculated properly
- No compilation errors about mutability or types

### Debugging tips:

1. Read the compiler error messages - they often suggest the fix
2. "cannot borrow as mutable" means you need `mut`
3. "expected type ... found type" often means reference issues
4. "no method named ... on type Option" means you need to unwrap

### Key concepts to remember:

- `Vec::push/pop/remove` require `&mut self`
- `.iter()` gives you references; use `.iter().cloned()` for owned values
- HashMap's entry API returns a mutable reference
- Many iterator methods return Option for potentially empty collections
- Type annotations help the compiler understand what you want to collect

### If still stuck:

1. Fix mutability issues first (add `mut`)
2. Fix reference issues in filter (dereference with `*` or `**`)
3. Fix the entry API (use `*count += 1`)
4. Fix Option handling (add `.unwrap()`)

The fix is usually 4-5 line changes!

## Testing Checklist

- [ ] Program compiles without warnings
- [ ] Vector operations work correctly
- [ ] Word frequency counts are accurate
- [ ] Statistics are calculated properly
- [ ] Output matches expected exactly
