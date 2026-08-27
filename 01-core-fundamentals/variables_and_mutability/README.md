# Concept: Variables & Mutability

## Overview

Variables are the foundation of any program. In Rust, variables are immutable by default, which encourages you to think carefully about what data might change and what should stay constant. This unique approach helps prevent entire classes of bugs.

## Learning Objectives

By the end of this concept, you will understand:
- How to declare variables with `let`
- Mutability and the `mut` keyword
- Variable shadowing and why it's useful
- Differences between variables and constants
- Rust's philosophy on immutability

## Theory

### Variable Declaration

In Rust, you declare variables using the `let` keyword:

```rust
let x = 5;
```

This creates a binding between the name `x` and the value `5`. The variable `x` is **immutable by default**, meaning you cannot change its value once bound.

### Mutability

If you need to change a variable's value, you must explicitly mark it as mutable using the `mut` keyword:

```rust
let mut y = 5;
y = 6;  // This is allowed because y is mutable
```

Without `mut`, trying to reassign a variable will result in a compile-time error:

```rust
let x = 5;
x = 6;  // ERROR: cannot assign twice to immutable variable `x`
```

### Why Immutability by Default?

Rust's immutability-by-default philosophy:
1. **Prevents accidental changes** - Compiler catches reassignments you didn't intend
2. **Makes code clearer** - Readers know which variables change and which don't
3. **Enables optimizations** - Compiler can optimize immutable data more aggressively
4. **Improves thread safety** - Immutable data is inherently thread-safe

### Variable Shadowing

Shadowing occurs when you declare a new variable with the same name as an existing variable. The new variable "shadows" (hides) the previous one:

```rust
let x = 5;
let x = x + 1;  // Shadows previous x
let x = x * 2;  // Shadows again
println!("{}", x);  // Prints 12
```

**Key difference from mutation**: Shadowing creates a new variable, while mutation changes the existing variable. This is useful when you want to transform data through multiple steps.

### Shadowing vs Mutation

```rust
// Mutation: changing the value of existing variable
let mut x = 5;
x = x + 1;

// Shadowing: creating a new variable with same name
let x = 5;
let x = x + 1;

// Shadowing allows type changes!
let spaces = "   ";
let spaces = spaces.len();  // spaces is now usize, not str
```

### Constants

Constants are different from variables:

```rust
const MAX_POINTS: u32 = 100_000;
```

**Differences from variables**:
- Always immutable (cannot use `mut` with constants)
- Must have explicit type annotation
- Can only be assigned constant expressions (known at compile time)
- Conventionally use SCREAMING_SNAKE_CASE names
- Have a specific scope (entire program or module)

```rust
// Constants from the Rust Book example
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## Syntax

### Variable Declaration Patterns

```rust
// Immutable variable
let x = 5;
let x: i32 = 5;  // With explicit type

// Mutable variable
let mut y = 10;
let mut y: i32 = 10;  // With explicit type

// Constant
const MAX: u32 = 100;
const MAX: u32 = 100_000;  // Can use underscores for readability

// Shadowing
let x = 5;
let x = "hello";  // Different type!

// Multiple variables
let (a, b) = (5, 10);
let mut (x, y) = (1, 2);
```

### Common Patterns

**Declaring multiple related variables**:
```rust
let (x, y, z) = (1, 2, 3);
let mut (width, height) = (100, 200);
```

**Shadowing for transformation**:
```rust
let input = "  42  ";
let input = input.trim();      // Remove whitespace
let input: u32 = input.parse().unwrap();  // Parse to number
```

**Using mut for collection building**:
```rust
let mut numbers = vec![];
numbers.push(1);
numbers.push(2);
numbers.push(3);
```

## Common Mistakes

### Mistake 1: Forgetting `mut` when you need mutation

```rust
// ❌ ERROR: cannot assign twice
let x = 5;
x = 6;

// ✅ CORRECT: use mut
let mut x = 5;
x = 6;
```

**Fix**: Add `mut` when you plan to reassign a variable.

### Mistake 2: Confusing shadowing with mutation

```rust
// This creates a NEW variable (shadowing), not mutation
let x = 5;
let x = x + 1;

// This modifies EXISTING variable (mutation)
let mut x = 5;
x = x + 1;
```

**Important**: Both result in the same value but work differently. Shadowing is preferred when transforming data.

### Mistake 3: Type mismatch with shadowing

```rust
// ❌ ERROR: wrong type
let x = 5;
x = "hello";  // Can't change from i32 to &str

// ✅ CORRECT: use shadowing
let x = 5;
let x = "hello";  // OK - creates new variable
```

**Fix**: Use shadowing (`let x = ...`) when you need to change types.

### Mistake 4: Trying to mutate constants

```rust
// ❌ ERROR: `const` cannot be `mut`
const mut X: i32 = 5;

// ✅ CORRECT: use let mut
let mut x = 5;
```

**Fix**: Constants are always immutable. Use variables if you need mutation.

## Real-World Examples

### Example 1: Processing user input

```rust
let input = "  42  ";
let input = input.trim();      // Remove whitespace
let input: u32 = input.parse().expect("not a number");
let input = input * 2;         // Double it

println!("{}", input);  // Prints: 84
```

Here, shadowing transforms the data through multiple steps.

### Example 2: Configuration and state

```rust
// Configuration (constant)
const DATABASE_TIMEOUT: u32 = 5000;

// Application state (mutable)
let mut active_connections = 0;
active_connections += 1;
active_connections += 1;
println!("Active: {}", active_connections);  // Prints: 2
```

### Example 3: Accumulating results

```rust
let mut sum = 0;
let numbers = vec![1, 2, 3, 4, 5];

for n in numbers {
    sum += n;
}

println!("Sum: {}", sum);  // Prints: 15
```

## Related Concepts

### Prerequisites
- None (this is foundational!)

### What comes next
- **Data Types** - Understanding what types of data variables hold
- **Functions** - Parameters are variables too
- **Control Flow** - Variables used in conditions

### Cross-references
- Module 01: All other concepts depend on understanding variables
- Module 02: Collections are variables that hold multiple values
- Module 03: Testing involves creating and mutating test state

## Best Practices

### Use immutable variables by default

```rust
// ✅ Good: start immutable
let x = 5;

// ❌ Avoid: unnecessary mutability
let mut x = 5;
```

### Prefer shadowing for transformations

```rust
// ✅ Better: clear transformation steps
let name = "  John  ";
let name = name.trim();
let name = name.to_uppercase();

// ❌ Less clear: mutating the same variable
let mut name = "  John  ";
name = name.trim();
name = name.to_uppercase();
```

### Use constants for configuration

```rust
// ✅ Good: constants for unchanging values
const MAX_RETRIES: u32 = 3;
const API_TIMEOUT: u32 = 5000;

// ❌ Avoid: magic numbers in code
let max_retries = 3;
let api_timeout = 5000;
```

### Use descriptive names

```rust
// ✅ Clear intent
let player_score = 0;
let mut total_attempts = 0;

// ❌ Unclear
let x = 0;
let mut m = 0;
```

## Summary

- **Variables** are bindings between names and values
- **Immutability by default** helps prevent bugs
- **`mut` keyword** explicitly marks variables as mutable
- **Shadowing** creates new variables, allowing type changes
- **Constants** are for values known at compile time
- **Rust's philosophy** prioritizes safety and clarity

## Key Takeaways

1. Variables are immutable by default in Rust
2. Use `let mut` for variables you need to change
3. Shadowing is useful for transforming data
4. Constants are compile-time known values
5. Think about whether data should change before writing code

## Practice Exercise Ideas

1. Create a program that uses shadowing to transform a number
2. Declare constants for a simple game configuration
3. Practice using `mut` for variables that need to change
4. Experiment with type changes using shadowing

---

**Time to complete this concept**: 1-1.5 hours
**Difficulty**: Beginner
**Prerequisite**: None
**Next concept**: Data Types

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
