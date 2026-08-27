# Exercise 10: Documentation and Doc Tests

## Difficulty: Medium
## Concepts Tested: Doc Comments, Doc Tests, Markdown in Docs, cargo doc
## Prerequisites: Module 03 - Documentation, Module 01-02 Foundations

---

## Problem Statement

Write a Rust library with comprehensive documentation including doc comments and doc tests. The program should:

1. Create well-documented functions with examples
2. Write documentation in markdown format
3. Include executable doc tests
4. Create module-level documentation
5. Document parameters and return values
6. Generate HTML documentation with cargo doc

## Requirements

- Write doc comments for module, types, and functions
- Use proper markdown formatting in docs
- Include code examples in doc comments
- Create doc tests that serve as documentation and tests
- Document panic conditions when applicable
- Document Examples section for each function
- Test doc examples with `cargo test --doc`
- Generate and view documentation with `cargo doc --open`

## Functions to Document

### Function 1: gcd(a: u32, b: u32) -> u32
- Calculate greatest common divisor
- Document using Euclidean algorithm
- Include example showing usage
- Document why it works

### Function 2: is_valid_email(email: &str) -> bool
- Simple email validation
- Document validation rules
- Show examples of valid/invalid emails
- Note: Not production-grade

### Function 3: calculate_average(numbers: &[f64]) -> Option<f64>
- Calculate average of numbers
- Handle empty slice with Option
- Show doc test examples
- Document error case (empty)

## Expected Output

### Doc Generation
```
cargo doc --open

Should generate:
- HTML docs for module and functions
- Examples section in each function's documentation
- Links between related items
```

### Doc Tests
```
cargo test --doc

Should show:
- Each doc test runs and passes
- Examples execute correctly
- Output matches documentation
```

## Hints

1. **Hint 1**: Use `///` for item documentation
2. **Hint 2**: Use `//!` for module/crate documentation
3. **Hint 3**: Doc tests are code blocks in doc comments with ` ```rust` marker
4. **Hint 4**: Doc tests are verified to compile and run
5. **Hint 5**: Use markdown formatting: **bold**, `code`, lists
6. **Hint 6**: The broken code has missing or incorrect documentation

## Testing

Run doc tests:
```bash
cargo test --doc
```

Generate documentation:
```bash
cargo doc --open
```

## Learning Objectives

After completing this exercise, you should understand:
- Writing doc comments in Rust
- Markdown formatting in documentation
- Creating executable doc tests
- Documentation best practices
- Documenting parameters and returns
- Using examples in documentation
- Generating and viewing docs
- Doc tests as living documentation

