# Exercise 4: Debug Output Statements to Find Bug

## Difficulty: Easy
## Concepts Tested: Debugging, println!, dbg! macro
## Prerequisites: Module 03 - Debugging techniques

---

## Problem Statement

You are given a program that compiles and runs, but produces incorrect output. Your task is to use `println!` and `dbg!` macros to trace the program's execution, identify the bug, and fix it. This exercise teaches practical debugging skills.

## Requirements

- Run the program and observe the wrong output
- Add strategic `println!` and `dbg!` statements
- Trace variable values through execution
- Identify where the logic goes wrong
- Fix the bug
- Verify the output is now correct

## The Program

The program implements a simple shopping cart calculator that should:
1. Apply quantity discounts (10% off for 3+ items)
2. Apply a coupon code discount (15% off)
3. Calculate tax (8%)
4. Return the final total

## Current (Wrong) Output

```
Shopping Cart Calculator
========================
Item: Widget at $10.00 x 5
Subtotal: $50.00
After quantity discount: $45.00
After coupon (SAVE15): $38.25
Tax (8%): $3.06
-----------
TOTAL: $38.25    <-- WRONG! Should include tax
========================
```

## Expected (Correct) Output

```
Shopping Cart Calculator
========================
Item: Widget at $10.00 x 5
Subtotal: $50.00
After quantity discount: $45.00
After coupon (SAVE15): $38.25
Tax (8%): $3.06
-----------
TOTAL: $41.31    <-- Correct: $38.25 + $3.06
========================
```

## Debugging Workflow

1. Add `println!` statements to trace values:
   ```rust
   println!("DEBUG: variable = {}", variable);
   ```

2. Use `dbg!` macro for quick debugging:
   ```rust
   let result = dbg!(some_calculation());
   ```

3. Trace the flow through each calculation step

4. Find where the final total goes wrong

5. Fix the bug and remove debug statements

## Notes

- `dbg!()` prints file, line number, expression, and value
- `dbg!()` returns the value, so it can be used inline
- Start debugging at the end (wrong output) and work backwards
- The bug is a logic error, not a syntax error
