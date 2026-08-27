# Hints for Exercise 1: Guessing Game

## Bug 1: Random Number Range

**Location**: `simple_random()` function

**Issue**: The modulo operation `% max` returns values from 0 to max-1, but we need 1 to 100.

**Hint**: You need to add 1 to shift the range. Think about: if modulo gives 0-99, how do you get 1-100?

**Pattern**:
```rust
// Wrong: 0 to max-1
let value = number % max;

// Right: 1 to max (inclusive)
let value = (number % max) + 1;
```

## Bug 2: Secret Number Generation

**Location**: Line where `secret` is assigned

**Issue**: This is actually a consequence of Bug 1. If `simple_random()` can return 0, the secret number could be 0, which breaks the game logic.

**Hint**: Once you fix Bug 1, this is automatically fixed. Test with values 1-100 to ensure it works.

## Bug 3: Input Validation

**Location**: Input parsing section

**Issue**: The code checks if input is a valid number but doesn't validate the range (1-100).

**Hint**: After parsing the number successfully, add another check:
```rust
// Parse number
let guess: u32 = input.trim().parse()?;

// Check range - what's missing here?
if guess < 1 || guess > 100 {
    println!("Please enter a number between 1 and 100");
    continue;  // Skip this iteration and ask again
}
```

## Summary of Changes

1. **Fix `simple_random()`**: Add 1 to the modulo result to get range 1 to max
2. **Test secret number**: After fixing simple_random, verify it's in correct range
3. **Add range validation**: Check that parsed guess is between 1 and 100

## Testing Your Solution

After fixing the bugs, test with:
- Valid guesses: 50, 75, 25 (should work)
- Invalid number: "abc" (should show error and ask again)
- Out of range: 0 or 101 (should show range error)
- Boundary values: 1 and 100 (should work correctly)

## Full Solution Pattern

```rust
fn simple_random(max: u32) -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    (duration.as_nanos() as u32) % max + 1  // FIX: Add + 1
}

// In main loop:
let guess = match input.trim().parse::<u32>() {
    Ok(n) => n,
    Err(_) => {
        println!("Invalid input!");
        continue;
    }
};

// FIX: Add this validation
if guess < 1 || guess > 100 {
    println!("Please enter a number between 1 and 100");
    continue;
}

// Rest of comparison logic...
```

