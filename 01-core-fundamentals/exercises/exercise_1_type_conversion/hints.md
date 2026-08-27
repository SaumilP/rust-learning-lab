# Hints for Exercise 1: Type Conversion Challenge

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: Result Type Handling**
- The `parse()` method returns `Result<i32, ParseIntError>`
- You need to extract the value from the Result
- Use `.unwrap()` to get the value (simpler) or match (more robust)
- Example: `let value: i32 = "42".parse().unwrap();`

**Bug 2: Wrong Type Assignment**
- Look at this line: `let fahrenheit_i64: i32 = fahrenheit as i64;`
- The variable is declared as `i32` but we're casting to `i64`
- The type annotation and cast don't match!
- Fix: Change the type annotation to `i64` or remove the `as i64` cast

**Bug 3: Type Mismatch in Arithmetic**
- You can't do arithmetic between `i32` and `f64` directly
- When you write `(fahrenheit - 32) * 5.0 / 9.0`
- `fahrenheit` is `i32`, but `5.0` is `f64`
- The expression `fahrenheit - 32` results in `i32`
- Multiplying `i32` by `f64` is not allowed
- Fix: Cast the entire expression to f64: `((fahrenheit - 32) as f64) * 5.0 / 9.0`

### Testing your fix:

After fixing the bugs, run:
```bash
cargo run
```

You should see:
- No compilation errors
- Three temperature conversions
- Accurate Celsius calculations
- Proper type conversions shown

### Debugging tips:

1. Read the compiler error messages carefully - they often point to the exact line
2. Look for "mismatched types" errors - that's usually a casting issue
3. Look for "expected Result" errors - that means parse() output needs handling
4. Test with edge cases like 32 (freezing), 98 (body temp), 212 (boiling)

### Key concepts to remember:

- `parse::<T>()` always returns `Result<T, ParseError>`
- `as` keyword is for type casting between compatible types
- Arithmetic operations require all operands to be the same type (or will be promoted)
- `i32` cannot be promoted to `f64` automatically - must use `as f64` explicitly

### If still stuck:

1. Look at the compiler error line numbers
2. Check if it's about `parse()` → likely needs `.unwrap()`
3. Check if it's about type annotations → likely type mismatch
4. Check if it's about arithmetic → likely casting needed

The fix is usually 2-3 line changes!
