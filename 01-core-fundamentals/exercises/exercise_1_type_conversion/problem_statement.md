# Exercise 1: Type Conversion Challenge

## Difficulty: Easy
## Concepts Tested: Data Types, Type Casting, Type Conversion
## Prerequisites: Module 01 - Data Types, Variables

---

## Problem Statement

Write a Rust program that converts between different numeric types and demonstrates type safety. The program should:

1. Read a string input representing a temperature value
2. Parse it to an i32 (whole number temperature)
3. Convert it to different numeric types (i64, f64)
4. Perform calculations with different types
5. Display results showing the differences between types

## Requirements

- Accept user input or use hardcoded test values
- Parse string to integer safely (handle invalid input)
- Convert integer to i64 using `as` keyword
- Convert integer to f64 for calculations
- Demonstrate why type conversion matters (precision, range)
- Show final temperature in multiple formats
- Handle at least 3 different input scenarios

## Expected Input/Output

### Test Case 1:
```
Input temperature: 32
Output:
32°F (i32)
32 (i64)
32.0°C (f64 from conversion)
Converted to Celsius: 0.0°C
```

### Test Case 2:
```
Input temperature: 98
Output:
98°F (i32)
98 (i64)
98.0°C (f64 from conversion)
Converted to Celsius: 36.7°C
```

### Test Case 3:
```
Input temperature: 212
Output:
212°F (i32)
212 (i64)
212.0°C (f64 from conversion)
Converted to Celsius: 100.0°C
```

## Hints

1. **Hint 1**: Use `.parse::<i32>()` to convert String to i32. It returns a `Result` type.
2. **Hint 2**: The `as` keyword is used for type casting: `value as i64` or `value as f64`
3. **Hint 3**: String temperature values might have trailing whitespace - use `.trim()`
4. **Hint 4**: Temperature formula: Celsius = (Fahrenheit - 32) * 5/9
5. **Hint 5**: When converting i32 to f64, use: `value as f64` and the result will have decimal point
6. **Hint 6**: The broken code has type mismatches and incorrect conversions - check the types carefully

## Testing

Run your program with these test values:
```bash
cargo run
# Try: 32, 98, 212
```

Expected behavior:
- Accepts integer input
- Displays value in multiple formats
- Calculates Celsius correctly
- No compile errors about type mismatches

## Learning Objectives

After completing this exercise, you should understand:
- How to parse strings to numbers
- Type casting with `as` keyword
- The difference between integer and floating-point types
- Why type safety matters in Rust
- Converting between types for calculations
