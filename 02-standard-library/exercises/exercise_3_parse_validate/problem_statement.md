# Exercise 3: Parse and Validate Input with Option/Result

## Difficulty: Medium
## Concepts Tested: Option, Result, Error handling, Parsing
## Prerequisites: Module 02 - Error Handling Basics

---

## Problem Statement

Write a Rust program that demonstrates proper use of Option and Result types for parsing and validating user input. This exercise teaches safe error handling patterns that are fundamental to writing robust Rust code.

## Requirements

- Parse string input to different types (integers, floats)
- Validate constraints (min/max values, format rules)
- Return Option or Result appropriately based on context
- Handle errors gracefully without panicking
- Process multiple inputs and report results

## Tasks

### Task 1: Number Parsing
- Parse strings to integers
- Handle parse errors with Result
- Provide informative error messages
- Show both successful and failed parsing

### Task 2: Safe Collection Access
- Access vector elements safely using Option
- Handle out-of-bounds cases
- Get first/last elements optionally
- Find elements matching predicates

### Task 3: Input Validation
- Validate age (must be 0-120)
- Validate name (must be at least 2 characters)
- Validate email format (must contain '@')
- Return structured error messages

## Expected Output

```
=== Number Parsing ===
Parsing "42": Ok(42)
Parsing "3.14": Ok(3.14)
Parsing "hello": Err(invalid digit found in string)
Parsing "999999999999": Err(number too large to fit in target type)

Valid numbers: [42, 100, -5]
Invalid inputs: ["abc", "12.34.56"]

=== Safe Collection Access ===
Numbers: [10, 20, 30, 40, 50]
First element: Some(10)
Last element: Some(50)
Element at index 2: Some(30)
Element at index 10: None
Find first > 25: Some(30)
Find first > 100: None

=== Input Validation ===
Validate age=25, name="Alice": Ok("Valid input")
Validate age=150, name="Alice": Err("Age must be between 0 and 120")
Validate age=25, name="A": Err("Name must be at least 2 characters")
Validate age=25, name="Bob", email="bob@email.com": Ok("Valid input")
Validate age=25, name="Bob", email="invalid": Err("Email must contain '@'")
```

## Notes

- Use `parse::<i32>()` for string to integer conversion
- `Vec::get(index)` returns `Option<&T>` instead of panicking
- Create custom Result types for validation functions
- The `?` operator can propagate errors in functions returning Result
