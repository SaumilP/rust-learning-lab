# Hints for Exercise 3: Parse and Validate Input with Option/Result

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: parse() Returns Result**
- `str::parse()` returns `Result<T, ParseError>`, not the value directly
- You can't assign a Result to a variable typed as `i32`
- The variable needs to be typed as `Result<i32, _>` or inferred
- Fix:
  ```rust
  let result: Result<i32, _> = input.parse();
  // or use turbofish syntax
  let result = input.parse::<i32>();
  ```

**Bug 2: filter_map with Result**
- `.ok()` converts `Result<T, E>` to `Option<T>` (Some if Ok, None if Err)
- This is actually the correct pattern for filter_map
- The code `s.parse::<i32>().ok()` should work
- Make sure the type annotation is correct
- Example:
  ```rust
  let valid: Vec<i32> = numbers.iter()
      .filter_map(|s| s.parse::<i32>().ok())
      .collect();
  ```

**Bug 3: find() Closure Reference Levels**
- When you call `.iter()` on Vec<i32>, you get `Iterator<Item = &i32>`
- In `find()`, the closure receives `&&i32` (reference to the iterator item)
- Comparing `&&i32` with `i32` literal (like `25`) doesn't work directly
- Fix: Dereference in the pattern: `|&&x| x > 25`
- Or dereference explicitly: `|x| **x > 25`
- Example:
  ```rust
  let found = numbers.iter().find(|&&x| x > 25);
  ```

**Bug 4: Return Type Consistency**
- Functions returning `Result` must wrap returns in `Ok()` or `Err()`
- Plain string literals aren't valid Result returns
- Every return path must use the Result constructors
- Fix:
  ```rust
  fn validate_input(age: i32, name: &str) -> Result<&str, &str> {
      if age < 0 || age > 120 {
          return Err("Age must be between 0 and 120");
      }
      if name.len() < 2 {
          return Err("Name must be at least 2 characters");
      }
      Ok("Valid input")
  }
  ```

### Testing your fix:

After fixing the bugs, run:
```bash
cargo run
```

You should see:
- Parsing results with Ok/Err wrappers
- Collection access returning Some/None correctly
- Validation returning proper Result types
- No panic or unwrap failures

### Debugging tips:

1. Compiler errors about "expected Result, found &str" mean you need Ok/Err
2. Type mismatch errors often indicate reference level issues
3. Use turbofish (`::<Type>`) when the compiler can't infer types
4. The `?` operator only works in functions returning Result/Option

### Key concepts to remember:

**Option<T>**:
- Represents a value that might not exist
- `Some(value)` or `None`
- Methods: `unwrap()`, `unwrap_or()`, `map()`, `and_then()`

**Result<T, E>**:
- Represents an operation that might fail
- `Ok(value)` or `Err(error)`
- Methods: `unwrap()`, `ok()`, `map()`, `map_err()`, `?`

**Converting between them**:
- `result.ok()` -> Option (discards error)
- `result.err()` -> Option (discards success)
- `option.ok_or(err)` -> Result

### Common patterns:

```rust
// Parse with error handling
let num: i32 = "42".parse().unwrap_or(0);
let num: Result<i32, _> = "42".parse();

// Safe collection access
let first = vec.first();  // Option<&T>
let at_index = vec.get(5); // Option<&T>

// Find in iterator
let found = iter.find(|&&x| x > 10);

// Validation function
fn validate(x: i32) -> Result<i32, &'static str> {
    if x > 0 { Ok(x) } else { Err("must be positive") }
}
```

### If still stuck:

1. Add explicit type annotations to help the compiler
2. Use `dbg!()` to see intermediate values and types
3. Check that all return paths use Ok/Err
4. Count reference levels (&, &&) in closures

The fix is usually 4-5 line changes!

## Testing Checklist

- [ ] Program compiles without warnings
- [ ] Parse results show correct Ok/Err values
- [ ] Collection access returns Some/None correctly
- [ ] Validation functions return proper Results
- [ ] Output matches expected exactly
