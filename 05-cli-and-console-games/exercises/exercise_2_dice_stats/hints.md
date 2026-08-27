# Hints for Exercise 2: Dice Statistics

## Bug 1: Missing Input Validation

**Location**: After parsing sides and rolls

**Issue**: No check to ensure valid input (sides must be >= 2, rolls must be >= 1)

**Hint**:
```rust
if sides < 2 {
    println!("Dice must have at least 2 sides");
    return;  // Exit early
}

if rolls < 1 {
    println!("Must roll at least once");
    return;
}
```

## Bug 2: Bar Chart Calculation

**Location**: Bar length calculation

**Issue**: `bar_length` is always 20, should be proportional to count

**Hint**: Calculate as ratio of count to rolls:
```rust
let bar_length = (count as u32 * 20) / rolls;  // Proportional to percentage
let bar = "█".repeat(bar_length as usize) + &"░".repeat(20 - bar_length as usize);
```

## Bug 3: Expected Value Formula

**Location**: Expected value calculation

**Issue**: Formula is `(sides + 1)` but should be `(sides + 1) / 2`

**Hint**: For a fair die with sides 1 to N, expected value is (N + 1) / 2:
- d6: (6 + 1) / 2 = 3.5
- d20: (20 + 1) / 2 = 10.5
- d100: (100 + 1) / 2 = 50.5

```rust
let expected = (sides as f64 + 1.0) / 2.0;
```

## Testing

Test with:
- Sides: 6, Rolls: 100 (standard dice)
- Sides: 20, Rolls: 1000 (more rolls for statistics)
- Invalid: 1 side (should be rejected)
- Invalid: 0 rolls (should be rejected)

