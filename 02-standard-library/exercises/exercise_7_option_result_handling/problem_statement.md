# Exercise 7: Option and Result Handling

## Difficulty: Medium
## Concepts Tested: Option, Result, Pattern Matching, Error Handling
## Prerequisites: Module 02 - Error Handling Basics

---

## Problem Statement

Write a Rust program that demonstrates proper Option and Result handling patterns. The program should:

1. Parse strings to numbers safely using Result
2. Handle optional values with Option
3. Use pattern matching on Option and Result
4. Implement custom validation logic
5. Demonstrate error propagation with the `?` operator

## Requirements

- Implement string to integer parsing
- Handle cases where parsing fails
- Use Option to represent potentially missing values
- Create custom Result types for validation
- Demonstrate multiple error handling strategies
- Use pattern matching to handle cases
- Show proper error messages
- Implement safe data access from collections

## Scenarios to Handle

### Scenario 1: Number Parsing
- Parse strings to integers
- Handle parse errors
- Provide informative error messages
- Show successful vs failed parsing

### Scenario 2: Safe Collection Access
- Access vector elements safely
- Handle out-of-bounds cases
- Get first/last elements optionally
- Find elements with predicates

### Scenario 3: Validation
- Create validation functions that return Result
- Check multiple conditions
- Chain validation operations
- Show error reasons

## Expected Input/Output

### Test Case 1: Number Parsing
```
Parsing "42":
  Ok: 42
Parsing "abc":
  Err: invalid digit found in string

Success: 42, 87, -15
Failed: ["abc", "xyz"], total failures: 2
```

### Test Case 2: Collection Access
```
Vector: [10, 20, 30, 40, 50]
First: Some(10)
Last: Some(50)
At index 2: Some(30)
At index 10: None
Find > 25: Some(30)
```

### Test Case 3: Validation
```
Validate (12, "Alice"): Ok, valid age and name
Validate (150, "Alice"): Err: age out of range
Validate (25, "A"): Err: name too short
Validate (25, "Bob"): Ok, valid age and name
```

## Hints

1. **Hint 1**: Use `.parse::<i32>()` to convert strings - returns `Result<i32, ParseIntError>`
2. **Hint 2**: `Vec::get(index)` returns `Option<&T>` safely
3. **Hint 3**: Use `match` or `if let` to handle Option and Result
4. **Hint 4**: `.unwrap_or()` provides default value for Option
5. **Hint 5**: The `?` operator propagates errors in functions returning Result
6. **Hint 6**: The broken code has issues with error handling, pattern matching, or type conversion

## Testing

Run your program:
```bash
cargo run
```

You should see:
- Successful and failed parsing attempts
- Safe collection access results
- Validation results with error messages
- No panics or unwrap failures

## Learning Objectives

After completing this exercise, you should understand:
- Option type and its methods
- Result type and its methods
- Pattern matching on Option and Result
- Error propagation with `?` operator
- Unwrap vs safe alternatives
- Custom validation functions
- Combining multiple Option/Result operations
- Creating descriptive error types

