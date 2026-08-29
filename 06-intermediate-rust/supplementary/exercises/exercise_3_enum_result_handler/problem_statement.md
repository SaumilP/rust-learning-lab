# Exercise 3: Error Handling with Enums and Pattern Matching

## Difficulty: Medium
## Concepts Tested: Enums, Pattern Matching, Error Handling
## Prerequisites: Module 06 concepts

## Problem Statement

Create an error handler that:

1. Define custom error enum with variants
2. Return Result<T, CustomError>
3. Pattern match to handle all error cases
4. Provide meaningful error messages
5. Demonstrate error propagation

## Requirements

- Custom error enum with 3+ variants
- Result type with custom error
- Exhaustive pattern matching
- Error recovery or message display
- Chain operations that can fail

## Expected Output

```
Parsing "42": Ok(42)
Parsing "abc": Err(InvalidNumber)
Parsing "": Err(Empty)
Parsing "9999999": Err(OutOfRange)

Handling division by zero...
Error: DivisionByZero
Calculation failed, using default: 0

Chained operations...
Result: Success(100)
```

