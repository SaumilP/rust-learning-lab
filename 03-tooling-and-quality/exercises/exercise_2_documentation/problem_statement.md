# Exercise 2: Add Documentation and Pass Doc Tests

## Difficulty: Medium
## Concepts Tested: Documentation, Doc comments, Doc tests
## Prerequisites: Module 03 - Documentation practices

---

## Problem Statement

You are given a Rust module with functions that lack proper documentation. Your task is to add comprehensive documentation including doc comments with examples that serve as doc tests. The examples in your documentation must compile and pass when running `cargo test --doc`.

## Requirements

- Add doc comments (`///`) to all functions
- Include a description of what each function does
- Document parameters and return values
- Add code examples in doc comments
- Ensure examples compile and pass as doc tests
- Create module-level documentation (`//!`)

## Functions to Document

### Function 1: `gcd(a: u32, b: u32) -> u32`
- Calculates the greatest common divisor using Euclidean algorithm
- Document the algorithm used
- Provide examples with different inputs

### Function 2: `is_prime(n: u32) -> bool`
- Checks if a number is prime
- Document edge cases (0, 1, 2)
- Show examples of prime and non-prime numbers

### Function 3: `fizzbuzz(n: u32) -> String`
- Returns "Fizz", "Buzz", "FizzBuzz", or the number as string
- Document the rules
- Show examples for each case

### Function 4: `celsius_to_fahrenheit(celsius: f64) -> f64`
- Converts temperature from Celsius to Fahrenheit
- Document the formula used
- Show example conversions

## Expected Output

### When running `cargo test --doc`:
```
running 8 doc-tests exercise_2_documentation
test src/lib.rs - celsius_to_fahrenheit (line 85) ... ok
test src/lib.rs - celsius_to_fahrenheit (line 91) ... ok
test src/lib.rs - fizzbuzz (line 58) ... ok
test src/lib.rs - fizzbuzz (line 65) ... ok
test src/lib.rs - gcd (line 15) ... ok
test src/lib.rs - gcd (line 21) ... ok
test src/lib.rs - is_prime (line 38) ... ok
test src/lib.rs - is_prime (line 44) ... ok

test result: ok. 8 passed
```

### When running `cargo doc --open`:
- HTML documentation should be generated
- Each function should have:
  - Description
  - Parameters section (if documented)
  - Returns section
  - Examples section with runnable code

## Notes

- Doc comments use `///` for items, `//!` for modules
- Code blocks in doc comments become doc tests
- Use `# ` to hide lines in doc test output but still run them
- Mark examples that should panic with `should_panic`
- Mark examples that won't compile with `ignore` or `compile_fail`
