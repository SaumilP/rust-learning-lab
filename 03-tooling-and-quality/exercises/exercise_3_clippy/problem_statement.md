# Exercise 3: Fix Clippy Warnings in Provided Code

## Difficulty: Easy
## Concepts Tested: Code quality, Clippy linting
## Prerequisites: Module 03 - Code quality tools

---

## Problem Statement

You are given Rust code that compiles and runs correctly, but contains several code quality issues that Clippy identifies. Your task is to refactor the code to satisfy all Clippy warnings while maintaining the same functionality.

## Requirements

- Run `cargo clippy` to identify all warnings
- Fix each warning following Clippy's suggestions
- Do not change the program's output or behavior
- Learn idiomatic Rust patterns from the fixes
- Final code should have zero Clippy warnings

## Issues to Fix

The provided code contains approximately 8-10 Clippy warnings including:

1. **Unnecessary `else` after `return`**
2. **Redundant clone**
3. **Manual implementation of standard functions**
4. **Inefficient string operations**
5. **Unnecessary variable binding**
6. **Comparison to boolean literal**
7. **Loop that could be iterator**
8. **Redundant pattern matching**

## Expected Workflow

1. Run `cargo clippy` to see all warnings
2. For each warning, understand what Clippy suggests
3. Apply the fix
4. Re-run `cargo clippy` to verify
5. Repeat until no warnings remain

## Expected Output

### Before fixing (cargo clippy):
```
warning: unneeded `return` statement
warning: redundant clone
warning: this `if` statement can be collapsed
... (8-10 warnings)
```

### After fixing (cargo clippy):
```
    Checking exercise_3 v0.1.0
    Finished dev [unoptimized + debuginfo] target(s)
```

### Program output (should be identical before and after):
```
Testing is_even:
  2 is even: true
  3 is even: false

Testing find_longest:
  Longest word: banana

Testing sum_positives:
  Sum of positives: 15

Testing format_greeting:
  Greeting: Hello, World!

All functions work correctly!
```

## Notes

- Clippy warnings include the specific line and suggested fix
- Some fixes are simple (remove a line), others require restructuring
- The goal is idiomatic Rust, not just "any working code"
- Read Clippy explanations to understand WHY each fix is better
