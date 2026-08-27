# Hints for Exercise 4: Ownership and Borrowing

## Bug 1: Ownership Transfer

**Issue**: Function parameter takes ownership with `Vec<T>`, but vector needed later

**Fix**: Change function signature to use reference:
```rust
// Wrong: Takes ownership
fn calculate_sum(nums: Vec<i32>) -> i32 { }

// Right: Borrows reference
fn calculate_sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

// Then use:
let sum = calculate_sum(&data);
println!("Average: {}", sum as f64 / data.len() as f64);  // OK: data still owned
```

## Bug 2: Multiple Mutable Borrows

**Issue**: Code tries to create two mutable references at once

**Fix**: Only one mutable reference at a time:
```rust
// Wrong:
let ref1 = &mut data;
let ref2 = &mut data;  // ERROR

// Right:
let ref1 = &mut data;
ref1.push(10);  // Use first reference
// ref1 scope ends here

let ref2 = &mut data;  // Now OK
ref2.push(20);
```

## Bug 3: Dangling Reference

**Issue**: Reference to temp_value that drops at end of block

**Fix**: Ensure value outlives reference:
```rust
// Wrong:
let reference;
{
    let temp = vec![1, 2, 3];
    reference = &temp;  // Dangling when block ends
}

// Right:
let temp = vec![1, 2, 3];  // Declared outside block
let reference = &temp;
// Both must be in same scope
```

## Testing

Try compiling with `cargo build` to see compiler errors and fix them step by step.

