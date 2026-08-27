# Hints for Exercise 3: Error Handler

## Bug 1: Inverted Range Logic

**Issue**: Returns Ok for out-of-range values

**Fix**:
```rust
if n < 0 || n > 1000 {
    Err(CustomError::OutOfRange)
} else {
    Ok(n)
}
```

## Bug 2: Non-Exhaustive Pattern Match

**Issue**: Missing Empty and OutOfRange cases

**Fix**:
```rust
match parse_number(val) {
    Ok(n) => println!("Ok({})", n),
    Err(CustomError::InvalidNumber) => println!("InvalidNumber"),
    Err(CustomError::Empty) => println!("Empty"),
    Err(CustomError::OutOfRange) => println!("OutOfRange"),
    Err(_) => println!("Other error"),
}
```

## Bug 3: Unused Error Information

**Issue**: Catches error but doesn't use it

**Fix**:
```rust
match divide(10, 0) {
    Ok(r) => println!("Result: {}", r),
    Err(CustomError::DivisionByZero) => {
        println!("Error: Cannot divide by zero");
    }
    Err(e) => println!("Error: {:?}", e),
}
```

