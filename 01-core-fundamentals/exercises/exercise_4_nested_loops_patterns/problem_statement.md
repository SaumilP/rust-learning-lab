# Exercise 4: Nested Loop Pattern Generation

## Difficulty: Medium
## Concepts Tested: Nested Loops, Control Flow, String Building, Variable Scope
## Prerequisites: Module 01 - Control Flow, Functions, Data Types

---

## Problem Statement

Write a Rust program that uses nested loops to generate various ASCII patterns. The program should:

1. Generate multiple patterns using nested loops
2. Handle different loop ranges and conditions
3. Build strings dynamically within loops
4. Demonstrate loop control with conditions
5. Show understanding of loop scoping and variables

## Requirements

- Implement at least 3 different patterns using nested loops
- Use appropriate loop ranges for each pattern
- Build output strings with proper spacing/formatting
- Use conditional logic within loops to control output
- Handle variable scope correctly
- Demonstrate pattern 1: Right triangle of asterisks
- Demonstrate pattern 2: Square with borders (hollow)
- Demonstrate pattern 3: Diamond shape
- Allow customizable pattern sizes (5, 6, 8)

## Patterns to Implement

### Pattern 1: Right Triangle
```
For size 5:
*
**
***
****
*****
```

### Pattern 2: Hollow Square
```
For size 5:
*****
*   *
*   *
*   *
*****
```

### Pattern 3: Diamond
```
For size 5:
    *
   ***
  *****
 *******
*********
 *******
  *****
   ***
    *
```

## Expected Input/Output

### Test Case 1: All Patterns with Size 5

```
=== Pattern 1: Right Triangle (Size 5) ===
*
**
***
****
*****

=== Pattern 2: Hollow Square (Size 5) ===
*****
*   *
*   *
*   *
*****

=== Pattern 3: Diamond (Size 5) ===
    *
   ***
  *****
 *******
*********
 *******
  *****
   ***
    *
```

## Hints

1. **Hint 1**: For right triangle, outer loop controls rows, inner loop controls columns
2. **Hint 2**: For hollow square, check if row/column is at border
3. **Hint 3**: For diamond, use absolute value to mirror the pattern
4. **Hint 4**: Use `String::new()` to create strings, then `push_str()` to build
5. **Hint 5**: Use conditions like `if i == 0` or `if j == n-1` to detect edges
6. **Hint 6**: For spacing in diamond, calculate `space_count = (size - i - 1)`
7. **Hint 7**: The broken code has issues with loop conditions, string building, or spacing

## Testing

Run your program:
```bash
cargo run
```

You should see:
- Three complete patterns
- Proper alignment and spacing
- Correct character placement
- No compile errors

## Learning Objectives

After completing this exercise, you should understand:
- How nested loops work in Rust
- Controlling outer and inner loop variables
- Building strings dynamically
- Using conditions within loops
- Calculating positions for pattern generation
- Proper indentation and formatting
- Loop scoping and variable lifetimes
- Combining multiple patterns in one program

