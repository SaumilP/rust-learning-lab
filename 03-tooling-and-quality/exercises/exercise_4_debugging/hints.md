# Hints for Exercise 4: Debug Output Statements to Find Bug

## Stuck? Here are some hints:

### Using println! for debugging:

Add print statements to see values at each step:
```rust
println!("DEBUG: subtotal = {}", subtotal);
println!("DEBUG: after_discount = {}", after_quantity_discount);
println!("DEBUG: after_coupon = {}", after_coupon);
println!("DEBUG: tax = {}", tax);
println!("DEBUG: final_total = {}", final_total);
```

### Using dbg! macro:

The `dbg!` macro is even more convenient:
```rust
let subtotal = dbg!(unit_price * quantity as f64);
// Output: [src/main.rs:25] unit_price * quantity as f64 = 50.0
```

Benefits of `dbg!`:
- Shows file name and line number
- Shows the expression being evaluated
- Shows the resulting value
- Returns the value (can be used inline)

### Tracing the calculation:

Follow the money through the calculation:

1. **Subtotal**: $10.00 x 5 = $50.00 (correct)
2. **Quantity discount**: $50.00 - 10% = $45.00 (correct)
3. **Coupon discount**: $45.00 - 15% = $38.25 (correct)
4. **Tax**: $38.25 x 8% = $3.06 (correct)
5. **Final**: Should be $38.25 + $3.06 = $41.31

Now look at what the program outputs:
- TOTAL: $38.25 (that's the after-coupon amount, not the final!)

### Finding the bug:

Look at this code:
```rust
let tax = after_coupon * 0.08;
println!("Tax (8%): ${:.2}", tax);

let final_total = after_coupon;  // <-- BUG HERE!
```

The tax is calculated, but never added to the total!

### The fix:

Change line 55 from:
```rust
let final_total = after_coupon;
```

To:
```rust
let final_total = after_coupon + tax;
```

### Debugging workflow:

1. **Observe**: Run program, see wrong output ($38.25 instead of $41.31)
2. **Hypothesize**: The tax ($3.06) is missing from the total
3. **Test**: Add dbg! to trace values
4. **Identify**: Find where tax should be added but isn't
5. **Fix**: Add tax to final_total
6. **Verify**: Run again, confirm correct output

### Example debugging session:

```rust
fn calculate_total(...) -> f64 {
    let subtotal = dbg!(unit_price * quantity as f64);
    // [src/main.rs:25] = 50.0

    let after_quantity_discount = dbg!(if quantity >= 3 {
        subtotal * 0.90
    } else {
        subtotal
    });
    // [src/main.rs:27] = 45.0

    let after_coupon = dbg!(after_quantity_discount * 0.85);
    // [src/main.rs:35] = 38.25

    let tax = dbg!(after_coupon * 0.08);
    // [src/main.rs:45] = 3.06

    let final_total = dbg!(after_coupon);  // AHA! Tax not included!
    // [src/main.rs:48] = 38.25  <-- Should be 41.31!

    final_total
}
```

### Other common debugging techniques:

```rust
// Conditional debug output
#[cfg(debug_assertions)]
println!("Debug: value = {}", value);

// Pretty-print complex structures
println!("{:#?}", complex_struct);

// Assertions for sanity checks
assert!(final_total > subtotal * 0.5, "Total seems too low");
debug_assert!(tax > 0.0, "Tax should be positive");
```

### After fixing:

1. Remove debug println! statements
2. Keep dbg! calls commented or remove them
3. Run program to verify correct output
4. Consider adding unit tests for the calculation

The fix is a single line change!

## Testing Checklist

- [ ] Identified the bug using debug output
- [ ] Fixed the calculation
- [ ] Output shows correct total ($41.31)
- [ ] Removed debugging statements
- [ ] Program runs cleanly
