# Hints for Exercise 3: FizzBuzz with Match Expressions

## Stuck? Here are some hints:

### About the bugs:

**Bug 1: Duplicate Pattern in Match**
- Look at the `fizzbuzz_simple` function
- You have the pattern `(false, false)` appearing **twice**
- One of these should be `(true, true)` for the FizzBuzz case
- The Rust compiler will warn you about this: "pattern is unreachable"
- The pattern should be: `(true, true) => println!("FizzBuzz")`
- Think about what each tuple combination means:
  - `(false, false)` → not divisible by 3 or 5 → print number
  - `(false, true)` → divisible by 5 only → "Buzz"
  - `(true, false)` → divisible by 3 only → "Fizz"
  - `(true, true)` → divisible by both → "FizzBuzz"

**Bug 2: Logic Error from Bug 1**
- Because Bug 1 has the wrong pattern, the FizzBuzz case is never handled
- When `i = 15`, we have `is_three = true` and `is_five = true`
- The match will look for a pattern, but `(true, true)` doesn't exist
- This actually causes the compiler error about non-exhaustive patterns
- Fix: Add the correct `(true, true)` pattern

**Bug 3: Non-Exhaustive Patterns**
- In `fizzbuzz_with_description`, the match is missing the `(true, true)` case
- The Rust compiler requires all patterns to be handled in a match
- You'll get: "error[E0004]: non-exhaustive patterns: `(true, true)` not covered"
- Fix: Add this arm: `(true, true) => println!("Number: {} → FizzBuzz", i),`

### Testing your fix:

After fixing the bugs, run:
```bash
cargo run
```

You should see:
- No compiler errors about non-exhaustive patterns
- No unreachable pattern warnings
- Three complete FizzBuzz sequences (1-15, 1-30, 1-15 with descriptions)
- Correct output: numbers, Fizz, Buzz, and FizzBuzz in right places

### Debugging tips:

1. Read the compiler error messages - they tell you exactly which patterns are missing
2. Count your match arms - should have 4 for two boolean values (2^2 = 4 combinations)
3. Test mentally: what happens when i=3? (true, false) → Fizz ✓
4. Test mentally: what happens when i=15? (true, true) → should be FizzBuzz
5. Use a systematic approach: list all combinations, then write patterns

### Key concepts to remember:

- **Tuple patterns**: `(bool1, bool2)` creates a two-element tuple
- **Pattern matching**: Match must cover all possible cases
- **Modulo operator**: `n % 3 == 0` checks if n is divisible by 3
- **Exhaustive matching**: Rust requires all cases handled (no default)
- **Logical combinations**: With 2 booleans, you have 2² = 4 combinations
- **Order in patterns**: Doesn't matter - `(true, false)` and `(false, true)` are different

### Pattern coverage checklist:

- [ ] `(false, false)` → Number
- [ ] `(false, true)` → Buzz
- [ ] `(true, false)` → Fizz
- [ ] `(true, true)` → FizzBuzz

### If still stuck:

1. Look for compiler error about "non-exhaustive patterns" - shows missing case
2. Look for "unreachable pattern" warning - means you have a duplicate
3. Count: do you have 4 pattern arms for 2 booleans?
4. Check the values: is `(true, true)` handled?
5. Remember: FizzBuzz only appears at multiples of 15 (3×5)

The fix is usually 2 line changes!

### Related concepts:

- **Boolean logic**: Understanding true/false combinations
- **Tuple patterns**: Matching structured data with multiple values
- **Modulo arithmetic**: Using % operator for divisibility
- **Pattern guards**: Could use `if` guards instead of tuples
- **Alternative patterns**: Could use `|` for OR patterns

### Alternative approach (without tuples):

You could also write this without tuples:
```rust
match (i % 3 == 0, i % 5 == 0) {
    (true, true) => println!("FizzBuzz"),
    (true, _) => println!("Fizz"),
    (_, true) => println!("Buzz"),
    _ => println!("{}", i),
}
```

This uses wildcard `_` which is more concise!

