# Hints for Exercise 2: String Manipulation Challenge

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: Trying to Mutate a &str**
- You can't call `.push_str()` on a `&str` - it's immutable
- The variable `original` is bound to a `&str` (string slice from the vector)
- You need to convert it to a `String` first
- Use `let mut original = test_str.to_string();` or `String::from(test_str)`
- Then you can call `.push_str()` on the `String`
- Example:
  ```rust
  let mut original = test_str.to_string();
  original.push_str(" World");  // Now this works
  ```

**Bug 2: Comparing char with &str**
- You're using `"l"` (a string literal, which is `&str`)
- But `.chars()` iterates over `char` types
- In the comparison `c == target_char`, you're trying to compare `char` with `&str`
- Fix: Use a single-quoted character literal: `'l'` instead of `"l"`
- Example:
  ```rust
  let target_char = 'l';  // char, not &str
  let count = original.chars().filter(|c| c == &target_char).count();
  ```

**Bug 3: Iterator Not Collected**
- `split(' ')` returns an iterator, not a Vec
- When you print an iterator with `{:?}`, it shows the iterator type, not the values
- You need to convert it to a collection using `.collect()`
- Use `.collect::<Vec<&str>>()` to get a `Vec` of string slices
- Example:
  ```rust
  let words: Vec<&str> = original.split(' ').collect();
  println!("Words: {:?}", words);  // Now prints ["Hello", "Rust"]
  ```

### Testing your fix:

After fixing the bugs, run:
```bash
cargo run
```

You should see:
- No compilation errors about borrowed values
- No type mismatch errors between char and &str
- Three string transformations
- All string methods working correctly
- Proper word lists displayed

### Debugging tips:

1. Read the compiler error messages carefully
2. Look for "cannot borrow as mutable" errors - usually means &str vs String issue
3. Look for "mismatched types" errors - could be char vs &str comparison
4. Look for "debug assertions" - could be iterator not collected
5. Test with strings of different lengths to verify character counting

### Key concepts to remember:

- `&str` is a string slice - immutable, borrowed reference
- `String` is an owned, mutable string
- Convert with `.to_string()`, `String::from()`, or `to_owned()`
- `char` uses single quotes: `'a'`
- `&str` uses double quotes: `"hello"`
- `.chars()` returns an iterator of `char` values
- `.split()` returns an iterator that must be collected
- Method chaining works: `.chars().filter(...).collect()`

### If still stuck:

1. Check the compiler error line numbers first
2. If it says "cannot call `.push_str()` on `&str`" → convert to String
3. If it says "mismatched types" involving char → check quotes (single vs double)
4. If printing shows `Split { ... }` → you need to `.collect()` the iterator
5. Remember: iterators are lazy - they don't do work until consumed

The fix is usually 3-4 line changes!

### Related concepts:

- **String vs &str**: String is owned, mutable; &str is a slice, immutable
- **Ownership**: Moving vs borrowing applies to strings too
- **Iterators**: Many string methods return iterators that need collecting
- **Character iteration**: `.chars()` gives you individual characters to work with
- **Method chaining**: You can chain string operations for efficiency

