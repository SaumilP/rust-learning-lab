# Concept: Control Flow

## Overview

Control flow is the order in which statements and expressions are executed in your code. Rust has several control flow constructs that allow you to decide whether to run certain code based on conditions or repeat code multiple times.

## Learning Objectives

By the end of this concept, you will understand:
- How to use `if`, `else if`, and `else` expressions
- Pattern matching with `match` expressions
- Different loop types: `loop`, `while`, `for`
- Loop control with `break` and `continue`
- Using control flow in real programs

## Theory

### if Expressions

The most basic control flow construct is `if`:

```rust
let x = 5;

if x > 3 {
    println!("x is greater than 3");
} else {
    println!("x is not greater than 3");
}
```

**Important**: Rust uses `if` not `if ()` - parentheses optional (but common):

```rust
if x > 3 {  // No parentheses needed
    // ...
}

if (x > 3) {  // Parentheses OK too
    // ...
}
```

### else if

For multiple conditions:

```rust
let x = 5;

if x < 0 {
    println!("negative");
} else if x == 0 {
    println!("zero");
} else if x > 0 && x < 10 {
    println!("positive single digit");
} else {
    println!("large positive");
}
```

### if as Expression

In Rust, `if` is an expression (returns a value):

```rust
let condition = true;
let number = if condition { 5 } else { 6 };
println!("{}", number);  // Prints: 5
```

**Important**: Branches must return same type:

```rust
// ❌ ERROR: different types
let number = if condition { 5 } else { "six" };

// ✅ CORRECT: same types
let number = if condition { 5 } else { 6 };
```

### match Expression

`match` is powerful for handling multiple possibilities with pattern matching:

```rust
let number = 3;

match number {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("something else"),
}
```

**Important points**:
- `=>` points to the code to run
- Must cover all possibilities (or use `_` catch-all)
- Like a switch statement, but more powerful

**match with multiple statements**:

```rust
match number {
    1 => {
        println!("one");
        println!("uno");
    }
    2 => {
        println!("two");
        println!("dos");
    }
    _ => println!("other"),
}
```

**match as expression**:

```rust
let description = match number {
    1 => "one",
    2 => "two",
    3 => "three",
    _ => "other",
};
println!("{}", description);  // Prints: three
```

**Pattern matching**:

```rust
let x = 5;

match x {
    1 | 2 => println!("one or two"),       // OR pattern
    3..=5 => println!("three to five"),    // Range
    _ => println!("something else"),
}
```

### Loops

Rust has three types of loops:

#### loop

An infinite loop that continues until explicitly broken:

```rust
loop {
    println!("Again!");
    break;  // Exits the loop
}
```

With a return value:

```rust
let result = loop {
    x += 1;
    if x == 10 {
        break x * 2;  // Breaks and returns value
    }
};
println!("{}", result);  // 20
```

#### while

Loops while a condition is true:

```rust
let mut x = 0;

while x < 5 {
    println!("{}", x);
    x += 1;
}
```

#### for

Iterates over a collection:

```rust
let arr = [1, 2, 3, 4, 5];

for element in arr.iter() {
    println!("{}", element);
}

for i in 0..5 {
    println!("{}", i);  // 0, 1, 2, 3, 4
}

for i in 1..=5 {
    println!("{}", i);  // 1, 2, 3, 4, 5
}
```

### Loop Control

**break** - Exits the loop:

```rust
for i in 0..10 {
    if i == 5 {
        break;  // Exits loop when i == 5
    }
    println!("{}", i);
}
```

**continue** - Skips to next iteration:

```rust
for i in 0..10 {
    if i == 5 {
        continue;  // Skips println! for i == 5
    }
    println!("{}", i);  // Prints 0,1,2,3,4,6,7,8,9
}
```

## Syntax

### if/else Expression

```rust
if condition {
    // code
} else if other_condition {
    // code
} else {
    // code
}
```

### match Expression

```rust
match value {
    pattern1 => expression,
    pattern2 => expression,
    _ => default_expression,
}
```

### Loops

```rust
loop {
    // infinite loop
    break;
}

while condition {
    // loop while condition is true
}

for item in collection {
    // iterate over collection
}
```

## Common Patterns

### Pattern 1: Simple validation

```rust
fn validate_age(age: u32) -> bool {
    if age >= 18 && age <= 120 {
        true
    } else {
        false
    }
}
```

### Pattern 2: Pattern matching

```rust
fn describe_number(n: i32) {
    match n {
        0 => println!("zero"),
        1 | 2 | 3 => println!("small"),
        4..=10 => println!("medium"),
        _ => println!("large"),
    }
}
```

### Pattern 3: Finding in loop

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}
```

### Pattern 4: Counting with loop

```rust
let mut count = 0;
for i in 0..100 {
    if is_even(i) {
        count += 1;
    }
}
println!("Count: {}", count);
```

### Pattern 5: Nested loops

```rust
for i in 1..=3 {
    for j in 1..=3 {
        println!("{} x {} = {}", i, j, i * j);
    }
}
```

## Common Mistakes

### Mistake 1: Forgetting `=>` in match

```rust
// ❌ ERROR: missing =>
match x {
    1 println!("one"),
    _ println!("other"),
}

// ✅ CORRECT
match x {
    1 => println!("one"),
    _ => println!("other"),
}
```

### Mistake 2: Not exhaustive match

```rust
// ❌ ERROR: doesn't cover all cases
match x {
    1 => println!("one"),
    2 => println!("two"),
    // What about all other numbers?
}

// ✅ CORRECT
match x {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"),
}
```

### Mistake 3: Different types in if branches

```rust
// ❌ ERROR: different types
let x = if condition { 5 } else { "five" };

// ✅ CORRECT
let x = if condition { 5 } else { 6 };
```

### Mistake 4: Infinite loop accidentally

```rust
// ❌ Infinite loop - condition never false
while true {
    println!("forever");
}

// ✅ Proper loop with exit
let mut x = 0;
while x < 5 {
    println!("{}", x);
    x += 1;
}
```

## Real-World Examples

### Example 1: Grade assignment

```rust
fn assign_grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

println!("{}", assign_grade(92));  // A
```

### Example 2: Day of week

```rust
fn day_name(day: u32) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid",
    }
}
```

### Example 3: Processing until condition

```rust
fn sum_until_zero(input: &[i32]) -> i32 {
    let mut sum = 0;
    for &n in input {
        if n == 0 {
            break;  // Stop processing
        }
        sum += n;
    }
    sum
}
```

### Example 4: Complex logic

```rust
fn categorize(age: u32, has_license: bool) -> &'static str {
    if age < 16 {
        "Too young"
    } else if age >= 16 && age < 18 {
        if has_license { "Teen driver" } else { "No license" }
    } else {
        "Adult driver"
    }
}
```

## Related Concepts

### Prerequisites
- **Variables & Mutability** - Used in conditions
- **Data Types** - Boolean type for conditions
- **Functions** - Control flow inside functions

### What comes next
- **Ownership** - Control flow interacts with ownership
- **Collections** - Loop over collections
- **Traits** - Pattern matching with traits

### Cross-references
- Module 01: Used in all remaining concepts
- Module 02: Iterators and functional programming
- Module 06: Advanced pattern matching

## Best Practices

### Prefer `match` over multiple `if` statements

```rust
// ✅ Better: cleaner with match
match status {
    "success" => println!("OK"),
    "error" => println!("Failed"),
    _ => println!("Unknown"),
}

// ⚠️ Works but less clean
if status == "success" {
    println!("OK");
} else if status == "error" {
    println!("Failed");
} else {
    println!("Unknown");
}
```

### Use `for` loops over `while` when possible

```rust
// ✅ Better: for is clearer
for i in 0..10 {
    println!("{}", i);
}

// ⚠️ Works but more verbose
let mut i = 0;
while i < 10 {
    println!("{}", i);
    i += 1;
}
```

### Make conditions readable

```rust
// ✅ Good: clear
if age >= 18 && has_license {
    can_drive = true;
}

// ❌ Less clear: complex nested
if !(age < 18 || !has_license) {
    can_drive = true;
}
```

## Summary

- **if/else** - Conditional execution
- **match** - Pattern matching (exhaustive)
- **loop** - Infinite loop with break
- **while** - Loop while condition true
- **for** - Iterate over collections
- **break** - Exit loop
- **continue** - Next iteration

## Key Takeaways

1. `if` is an expression that returns a value
2. `match` is for exhaustive pattern matching
3. All `if` branches must return same type
4. `for` is preferred over `while` for iteration
5. Use `match` for clarity over multiple `if`

## Practice Exercise Ideas

1. Write age validation with appropriate branch
2. Implement grade assignment with match
3. Create nested loops for pattern printing
4. Use continue/break in loops
5. Combine if/match for complex logic

---

**Time to complete this concept**: 1.5-2 hours
**Difficulty**: Beginner
**Prerequisite**: Variables & Mutability, Data Types, Functions
**Next concept**: None (end of Module 01)

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
