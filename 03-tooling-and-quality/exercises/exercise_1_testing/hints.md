# Hints for Exercise 1: Write Failing Tests Then Fix Code

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: is_palindrome - Case and Character Handling**
- The test `is_palindrome("RaceCar")` fails because comparison is case-sensitive
- The test `is_palindrome("A man a plan...")` fails because spaces aren't ignored
- Solution approach:
  1. Convert the string to lowercase
  2. Filter to keep only alphanumeric characters
  3. Then compare with reversed version
- Fix:
  ```rust
  pub fn is_palindrome(s: &str) -> bool {
      let cleaned: String = s.to_lowercase()
          .chars()
          .filter(|c| c.is_alphanumeric())
          .collect();
      let reversed: String = cleaned.chars().rev().collect();
      cleaned == reversed
  }
  ```

**Bug 2: fibonacci - Wrong Base Case**
- The Fibonacci sequence is: 0, 1, 1, 2, 3, 5, 8...
- F(0) = 0, F(1) = 1
- Current code returns 1 for both F(0) and F(1)
- Fix: Simply return `n` instead of `1` for the base case
  ```rust
  pub fn fibonacci(n: u32) -> u32 {
      if n <= 1 {
          return n;  // F(0)=0, F(1)=1
      }
      fibonacci(n - 1) + fibonacci(n - 2)
  }
  ```

**Bug 3: find_max - Empty Slice and Initial Value**
- Two bugs here:
  1. Returns `Some(0)` for empty slice, should be `None`
  2. Initializes max to 0, which fails for all-negative slices
- Fix:
  ```rust
  pub fn find_max(slice: &[i32]) -> Option<i32> {
      if slice.is_empty() {
          return None;
      }
      let mut max = slice[0];  // Start with first element
      for &val in slice.iter().skip(1) {
          if val > max {
              max = val;
          }
      }
      Some(max)
  }
  ```
- Alternative using iterator:
  ```rust
  pub fn find_max(slice: &[i32]) -> Option<i32> {
      slice.iter().cloned().max()
  }
  ```

**Bug 4: count_vowels - Case Sensitivity**
- Only lowercase vowels are checked
- Test with "HELLO" expects 2 but gets 0
- Fix Option 1: Add uppercase vowels to the check string
  ```rust
  let vowels = "aeiouAEIOU";
  ```
- Fix Option 2: Convert input character to lowercase before checking
  ```rust
  pub fn count_vowels(s: &str) -> usize {
      let vowels = "aeiou";
      s.chars()
          .filter(|c| vowels.contains(c.to_ascii_lowercase()))
          .count()
  }
  ```

### Testing your fixes:

Run tests with:
```bash
cargo test
```

Expected output:
```
running 12 tests
test tests::test_is_palindrome_simple ... ok
test tests::test_is_palindrome_with_spaces ... ok
... (all tests pass)
test result: ok. 12 passed
```

To see more details:
```bash
cargo test -- --nocapture
```

### Debugging tips:

1. Run tests one at a time to isolate issues:
   ```bash
   cargo test test_is_palindrome
   ```

2. Add debug output in functions:
   ```rust
   dbg!(&cleaned, &reversed);
   ```

3. Read test assertions carefully - they show expected values:
   ```rust
   assert_eq!(fibonacci(0), 0);  // Expected: 0
   ```

4. Test edge cases mentally before fixing

### Key testing concepts:

- `assert!(condition)` - passes if condition is true
- `assert_eq!(left, right)` - passes if left equals right
- `#[test]` marks a function as a test
- `#[cfg(test)]` marks a module as test-only
- Tests should be independent and repeatable

### Common patterns:

```rust
// Testing with Option
assert_eq!(func(), Some(value));
assert_eq!(func(), None);

// Testing with Result
assert!(result.is_ok());
assert!(result.is_err());

// Testing panic (if needed)
#[test]
#[should_panic]
fn test_panic() {
    panic_function();
}
```

### If still stuck:

1. Focus on one failing test at a time
2. Read the test assertion to understand expected behavior
3. Add println! to see actual values
4. Compare expected vs actual to identify the bug
5. Make minimal changes to fix each bug

The fix is usually 4-6 line changes across the functions!

## Testing Checklist

- [ ] All 12 tests pass
- [ ] No modifications to test functions
- [ ] Code handles edge cases correctly
- [ ] Functions work as documented
