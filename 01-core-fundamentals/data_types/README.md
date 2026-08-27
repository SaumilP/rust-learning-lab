# Concept: Data Types

## Overview

Every value in Rust is of a particular data type, which tells Rust what kind of data is being specified. Rust is a statically typed language, meaning type checking happens at compile time. However, Rust can often infer types, so you don't always need to declare them explicitly.

## Learning Objectives

By the end of this concept, you will understand:
- Rust's scalar types (integers, floats, booleans, characters)
- Compound types (tuples and arrays)
- String types and their differences
- Type inference and type annotations
- Type casting

## Theory

### Type System Overview

Rust's type system has two categories:

1. **Scalar types** - Single values
2. **Compound types** - Multiple values

### Scalar Types

#### Integers

Integers are whole numbers without a fractional component. Rust provides several integer types:

```
Signed:    i8, i16, i32, i64, i128, isize
Unsigned:  u8, u16, u32, u64, u128, usize
```

**Signed** vs **Unsigned**:
- Signed: can represent negative and positive numbers
- Unsigned: can only represent positive numbers

**Suffix notation**:
```rust
let x = 42u32;      // u32 suffix
let y = -5i32;      // i32 suffix
let z = 1_000_000;  // underscores for readability
```

**Integer overflow**:
- In debug mode: panics on overflow
- In release mode: wraps around
- Use checked/saturating methods for safety

#### Floating-Point Numbers

Floats represent numbers with decimal points. Rust has two floating-point types:

```rust
let x: f32 = 3.14;      // 32-bit float
let y: f64 = 2.71828;   // 64-bit float (default)
```

**Important**:
- `f64` is the default and more precise
- IEEE 754 standard floating-point numbers
- Can have special values: `f64::INFINITY`, `f64::NAN`

#### Booleans

The boolean type represents `true` or `false`:

```rust
let t = true;
let f: bool = false;
```

Used in conditionals:
```rust
if t {
    println!("true!");
}
```

#### Characters

The `char` type represents a single Unicode character:

```rust
let c = 'z';
let emoji = '😻';
let newline = '\n';
```

**Important**:
- Single quotes for `char`
- Double quotes for string slices (`&str`)
- Covers all Unicode scalar values
- 4 bytes in size (not 1!)

### Compound Types

#### Tuples

Tuples group multiple values of different types into one compound type:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Destructuring
let (x, y, z) = tup;
println!("y = {}", y);  // Prints: 6.4

// Accessing by index
println!("{}", tup.0);  // 500
println!("{}", tup.1);  // 6.4
```

**Properties**:
- Fixed length
- Can contain different types
- Access by position (0-indexed)

#### Arrays

Arrays are collections of the same type with a fixed size:

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let arr = [3; 5];  // [3, 3, 3, 3, 3]

// Access by index
println!("{}", arr[0]);  // 1

// Iteration
for item in arr.iter() {
    println!("{}", item);
}
```

**Properties**:
- Fixed length
- All same type
- Stack allocated
- 0-indexed

### String Types

Rust has two main string types:

#### String Slice (`&str`)

A reference to a string, known at compile time:

```rust
let s = "hello";  // &str - string literal
let s: &str = "world";
```

**Properties**:
- Immutable
- Fixed size (known at compile time)
- Stack allocated
- Often called "borrowed string"

#### String (`String`)

Owned, mutable string allocated on the heap:

```rust
let mut s = String::new();
s.push_str("hello");

let s = String::from("hello");
let s = "hello".to_string();
```

**Properties**:
- Mutable
- Dynamic size
- Heap allocated
- Can grow and shrink

**Key difference**:
```rust
let s1 = "hello";           // &str - string literal
let s2 = String::from("hello");  // String - owned
let s3 = s2.as_str();       // Convert String to &str
```

### Type Inference

Rust can infer types from context:

```rust
let x = 5;              // Inferred as i32
let y = 3.14;           // Inferred as f64
let z = true;           // Inferred as bool
let s = "hello";        // Inferred as &str
```

When ambiguous, provide explicit type:

```rust
let guess: u32 = "42".parse().expect("not a number");
```

### Type Casting

Converting between types using the `as` keyword:

```rust
let x = 5u8;
let y = x as u32;       // u8 to u32

let f = 3.14f64;
let i = f as i32;       // f64 to i32 (loses decimal)

let c = 65u8;
let chr = c as char;    // u8 to char - 'A'
```

**Important**:
- Can truncate or lose precision
- Use with caution
- Explicit casting shows intent

## Syntax

### Type Annotations

```rust
let x: i32 = 5;
let y: f64 = 3.14;
let flag: bool = true;
let ch: char = 'a';

let tup: (i32, f64) = (5, 3.14);
let arr: [i32; 3] = [1, 2, 3];

let s: &str = "hello";
let s: String = String::from("hello");
```

### Type Suffixes

```rust
42i32           // i32
3.14f64         // f64
1u8             // u8
true            // bool
'a'             // char
```

## Common Patterns

### Pattern 1: Type-specific operations

```rust
let x: i32 = 42;
let y: f64 = 3.14;

println!("{} + {} = {}", x, y, x as f64 + y);
```

### Pattern 2: String conversions

```rust
let num = 42;
let s = num.to_string();        // i32 to String
let s = format!("{}", num);     // Using format! macro

let s = "42";
let num: i32 = s.parse().unwrap();  // String to i32
```

### Pattern 3: Working with tuples

```rust
let person = ("Alice", 30, 5.6);
let (name, age, height) = person;
println!("{} is {} years old", name, age);
```

## Common Mistakes

### Mistake 1: Confusing `char` and `&str`

```rust
// ❌ ERROR: char with double quotes
let c = "a";  // This is &str, not char

// ✅ CORRECT: char with single quotes
let c = 'a';
```

### Mistake 2: String slice vs String

```rust
// ❌ This can be confusing
let s1 = "hello";               // &str - immutable literal
let mut s2 = String::from("hello");  // String - mutable owned

// ✅ Know the difference
// Use &str when you don't need to own/modify
// Use String when you need to build/modify
```

### Mistake 3: Integer overflow

```rust
let x: u8 = 255;
let y = x + 1;  // Debug: panic! Release: wraps to 0

// ✅ Use checked methods
let y = x.checked_add(1);  // Returns Option
```

### Mistake 4: Type casting precision loss

```rust
let f = 3.99f64;
let i = f as i32;  // Results in 3, not 4!

// ✅ Be aware of precision loss
println!("{}", i);  // 3
```

## Real-World Examples

### Example 1: Collecting user data

```rust
let name = "Alice";              // &str
let age: u8 = 30;                // u8
let height: f64 = 5.6;           // f64
let is_admin: bool = true;       // bool

let user = (name, age, height, is_admin);
```

### Example 2: Converting types

```rust
let input = "42";
let number: u32 = input.parse().expect("not a number");
let text = format!("The answer is {}", number);
println!("{}", text);  // The answer is 42
```

### Example 3: Working with arrays

```rust
let scores: [u32; 3] = [85, 90, 95];
let mut sum: u32 = 0;

for score in scores.iter() {
    sum += score;
}

let average = sum as f64 / scores.len() as f64;
println!("Average: {}", average);  // 90
```

## Related Concepts

### Prerequisites
- **Variables & Mutability** - Variables hold data of specific types

### What comes next
- **Functions** - Return and parameter types
- **Control Flow** - Type comparisons
- **Collections** - Complex data structures

### Cross-references
- Module 01: Used in all remaining concepts
- Module 02: Collections extend basic types
- Module 06: Traits and generics work with types

## Best Practices

### Use explicit types when ambiguous

```rust
// ✅ Good: explicit when not obvious
let numbers: Vec<i32> = vec![1, 2, 3];
let result: u32 = "42".parse().unwrap();

// ✓ OK: obvious from context
let x = 5;  // clearly i32
```

### Prefer `String` for owned strings, `&str` for references

```rust
// ✅ Good: function takes &str
fn greet(name: &str) {
    println!("Hello, {}", name);
}

// Use String when building strings
let mut message = String::new();
message.push_str("Hello");
```

### Use `as` sparingly and deliberately

```rust
// ✅ Good: explicit casting is intentional
let bytes: u32 = text.len() as u32;

// ❌ Avoid: loss of precision without notice
let value = large_f64 as i32;  // Might lose data silently
```

## Summary

- **Scalar types**: i32, f64, bool, char
- **Compound types**: tuples, arrays
- **String types**: &str (borrowed), String (owned)
- **Type inference**: Rust often figures it out
- **Type casting**: Use `as` for conversions
- **Static typing**: Checked at compile time

## Key Takeaways

1. Rust is statically typed (compile-time checking)
2. Integers: signed (i*) and unsigned (u*)
3. Floats: f32 and f64 (use f64 by default)
4. Strings: `&str` (borrowed) vs `String` (owned)
5. Type inference works in most cases
6. Use explicit types when code is ambiguous

## Practice Exercise Ideas

1. Create variables of each scalar type
2. Practice destructuring tuples
3. Convert between different number types
4. Compare `String` and `&str` usage
5. Parse string input to numbers

---

**Time to complete this concept**: 1-1.5 hours
**Difficulty**: Beginner
**Prerequisite**: Variables & Mutability
**Next concept**: Functions

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
