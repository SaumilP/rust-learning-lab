# Exercise 3: FizzBuzz with Match Expressions

## Difficulty: Easy to Medium
## Concepts Tested: Control Flow, Pattern Matching, Modulo Operator, Loops
## Prerequisites: Module 01 - Control Flow, Functions

---

## Problem Statement

Write a Rust program that implements the classic FizzBuzz algorithm using match expressions and pattern matching. The program should:

1. Iterate through a range of numbers (1 to N)
2. Apply FizzBuzz rules using pattern matching
3. Print results according to divisibility rules
4. Demonstrate effective use of match expressions
5. Use range patterns for cleaner code

## Requirements

- Use a `for` loop with range syntax
- Implement FizzBuzz using `match` expression with tuples
- Check divisibility with modulo operator (%)
- Use pattern matching with logical operators
- Print appropriate output based on rules
- Handle at least 2 different range sizes (15, 30)
- Demonstrate multiple approaches if possible

## FizzBuzz Rules

- If divisible by both 3 and 5 → print "FizzBuzz"
- If divisible by 3 only → print "Fizz"
- If divisible by 5 only → print "Buzz"
- Otherwise → print the number

## Expected Input/Output

### Test Case 1: Range 1-15
```
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```

### Test Case 2: Range 1-30
```
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
16
17
Fizz
19
Buzz
Fizz
22
23
Fizz
Buzz
26
Fizz
28
29
FizzBuzz
```

### Test Case 3: Using Descriptive Output
```
Number: 1 → 1
Number: 2 → 2
Number: 3 → Fizz
Number: 4 → 4
Number: 5 → Buzz
Number: 6 → Fizz
Number: 7 → 7
Number: 8 → 8
Number: 9 → Fizz
Number: 10 → Buzz
Number: 11 → 11
Number: 12 → Fizz
Number: 13 → 13
Number: 14 → 14
Number: 15 → FizzBuzz
```

## Hints

1. **Hint 1**: Use modulo operator `%` to check divisibility: `n % 3 == 0` means divisible by 3
2. **Hint 2**: Create a tuple with divisibility checks: `(n % 3 == 0, n % 5 == 0)`
3. **Hint 3**: Use match on the tuple: `match (is_three, is_five) { ... }`
4. **Hint 4**: Pattern combinations in match: `(true, true)`, `(true, false)`, `(false, true)`, `(false, false)`
5. **Hint 5**: Use `for i in 1..=15` for inclusive range, `1..15` for exclusive
6. **Hint 6**: The broken code has issues with logic conditions or match patterns

## Testing

Run your program:
```bash
cargo run
```

You should see:
- FizzBuzz output for range 1-15 (first 15 lines)
- FizzBuzz output for range 1-30
- Possibly descriptive output with "Number: X → Y" format

## Learning Objectives

After completing this exercise, you should understand:
- How to use `for` loops with ranges
- Pattern matching with tuples in `match` expressions
- The modulo operator for divisibility checking
- Logical operations in pattern matching
- Writing cleaner conditional logic with match
- Different approaches to the same problem
- Testing edge cases (multiples of 3, 5, and 15)

