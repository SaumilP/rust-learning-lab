# Hints for Exercise 10: Documentation and Doc Tests

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: Missing Documentation for gcd() Function**
- The `gcd` function has no documentation at all
- Every public function should have doc comments
- Documentation should explain what the function does
- Should include Examples section
- Should include doc tests
- Add above the function:
  ```rust
  /// Calculates the greatest common divisor of two numbers.
  ///
  /// Uses Euclidean algorithm for efficient computation.
  ///
  /// # Examples
  ///
  /// ```
  /// assert_eq!(gcd(48, 18), 6);
  /// assert_eq!(gcd(100, 50), 50);
  /// ```
  ```
- This creates documentation AND doc tests

**Bug 2: Incomplete Doc Test**
- The `is_valid_email` doc test is inside the doc comment
- The issue is that doc tests in this context need to be complete
- The function reference might need import in the doctest
- Actually, in library code, simple examples usually work
- This one should work, but if it doesn't, ensure:
  ```rust
  /// # Examples
  ///
  /// ```
  /// assert!(is_valid_email("user@example.com"));
  /// assert!(!is_valid_email("notanemail"));
  /// ```
  ```
- The test automatically has access to items in the same module

**Bug 3: Wrong Expected Value in Doc Test**
- The code: `assert_eq!(avg, Some(2.5));` in comment after wrong calculation
- Average of [1.0, 2.0, 3.0, 4.0] is: (1+2+3+4)/4 = 10/4 = 2.5
- But if the example shows [1.0, 2.0, 3.0], that's: (1+2+3)/3 = 6/3 = 2.0
- The doc comment shows [1.0, 2.0, 3.0, 4.0] but asserts Some(2.5)
- Wait - 2.5 IS correct for that set. The comment might be confusing.
- Check: [1, 2, 3, 4] → (1+2+3+4)/4 = 10/4 = 2.5 ✓
- So the assertion is correct! The comment explaining it might just be unclear

### Doc Comment Structure:

```rust
/// One-line summary of what this does
///
/// More detailed explanation if needed. Can span multiple lines.
/// Markdown formatting works here.
///
/// # Arguments
///
/// * `param1` - Description
/// * `param2` - Description
///
/// # Returns
///
/// Description of return value
///
/// # Panics
///
/// Describe any conditions that cause panic
///
/// # Examples
///
/// ```
/// // Code that will be tested as doc test
/// assert_eq!(function(input), expected);
/// ```
pub fn function(param1: Type, param2: Type) -> ReturnType {
    // ...
}
```

### Module Documentation:

```rust
//! # Crate or Module Name
//!
//! One-line description of what this module provides
//!
//! ## Features
//!
//! - Feature 1
//! - Feature 2
//! - Feature 3
//!
//! ## Examples
//!
//! ```
//! use this_module::function;
//! // Example code
//! ```
```

### Doc Test Execution:

```rust
/// # Examples
///
/// ```
/// // This code runs as a test
/// assert_eq!(add(2, 3), 5);
/// ```
///
/// # Panics
///
/// ```should_panic
/// // This code should panic
/// panic_function();
/// ```
///
/// # Ignored
///
/// ```ignore
/// // This code won't run - useful for non-runnable examples
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

### Testing your fix:

After fixing the bugs, run:
```bash
cargo test --doc
```

You should see:
- Doc tests for each documented function
- All tests pass (green checkmarks)
- Output shows which doc tests ran

### Generating documentation:

```bash
cargo doc --no-deps --open
```

This generates and opens HTML documentation showing:
- Module overview
- Function signatures with doc comments
- Examples sections
- Links between items

### Debugging tips:

1. Doc tests won't compile if there are syntax errors in examples
2. Doc tests automatically have access to items in current module
3. Use `#` to hide setup lines in examples: `let hidden = true;`
4. Use `ignore` directive to skip examples that shouldn't run
5. Use `should_panic` to test code that should panic

### Key concepts to remember:

- **`///` comments**: Document items (functions, types, etc.)
- **`//!` comments**: Document containing module or crate
- **Doc tests**: Code in examples is compiled and run as tests
- **Markdown support**: Use formatting for clarity
- **Auto-detection**: Functions without public doc comments are private
- **Visibility**: Only public items appear in documentation

### Doc Comment Best Practices:

```rust
// DON'T - No documentation
fn bad_function() {}

// BETTER - Basic documentation
/// Does something important
fn okay_function() {}

// BEST - Complete documentation
/// Performs mathematical operation on two numbers.
///
/// Adds the two arguments and returns the result.
/// Works with any numeric types that implement Add.
///
/// # Arguments
///
/// * `a` - First number
/// * `b` - Second number
///
/// # Returns
///
/// The sum of a and b
///
/// # Examples
///
/// ```
/// assert_eq!(add(2, 3), 5);
/// assert_eq!(add(-1, 1), 0);
/// ```
pub fn good_function(a: i32, b: i32) -> i32 { a + b }
```

### Common Doc Sections:

```
# Panics     - When function panics
# Errors     - Return error conditions (for Result)
# Safety     - Unsafe behavior and invariants required
# Examples   - Runnable code examples
# Arguments  - Parameter documentation
# Returns    - Return value documentation
```

### If still stuck:

1. **No doc tests running**: Make sure `/// # Examples` section exists
2. **Doc test fails to compile**: Check example code syntax
3. **Missing documentation**: Add `///` comments before public items
4. **Assertion wrong**: Calculate expected value carefully
5. **Can't generate docs**: Run `cargo doc --no-deps`

The fixes are usually 5-10 line changes per function!

### Related concepts:

- **Rustdoc**: The tool that generates documentation
- **Markdown rendering**: HTML output from doc comments
- **Doc comment inheritance**: Parent modules documented first
- **Cross-references**: Links to related functions in docs
- **Documentation linting**: clippy checks for missing docs
- **Test coverage**: Doc tests contribute to code coverage

