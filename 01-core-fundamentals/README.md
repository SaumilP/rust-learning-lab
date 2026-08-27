# Module 01: Core Fundamentals

Welcome to Core Fundamentals! This module covers the essential building blocks of Rust programming that you'll use in every program you write.

## Learning Objectives

By completing this module, you will understand:
- How Rust's ownership system works and why it matters
- Variable declaration and mutability semantics
- Rust's type system and type inference
- Function definition and control flow
- Pattern matching and exhaustive match expressions

## Module Structure

### 1. Variables & Mutability
**Concepts**:
- Variable declaration with `let`
- Mutability and `mut` keyword
- Variable shadowing
- Constants vs variables

**Time**: 1-2 hours
**Prerequisite**: Basic programming knowledge

**Key Files**:
- `variables_and_mutability/README.md` - Detailed explanation
- `variables_and_mutability/examples/` - Working code examples
- `variables_and_mutability/key_takeaways.md` - Quick reference

---

### 2. Data Types
**Concepts**:
- Scalar types (integers, floats, booleans, characters)
- Compound types (tuples, arrays)
- String types (`String` vs `&str`)
- Type casting and type inference

**Time**: 1-2 hours
**Prerequisite**: Variables & Mutability

**Key Files**:
- `data_types/README.md` - Detailed explanation
- `data_types/examples/` - Working code examples
- `data_types/key_takeaways.md` - Quick reference

---

### 3. Functions
**Concepts**:
- Function declaration with `fn`
- Parameters and return types
- Expressions vs statements
- Early returns and implicit returns
- Function documentation

**Time**: 1-2 hours
**Prerequisite**: Variables & Mutability, Data Types

**Key Files**:
- `functions/README.md` - Detailed explanation
- `functions/examples/` - Working code examples
- `functions/key_takeaways.md` - Quick reference

---

### 4. Control Flow
**Concepts**:
- `if`, `else if`, `else` expressions
- `match` expressions and pattern matching
- Loops: `loop`, `while`, `for`
- Loop control: `break`, `continue`
- Ternary-like expressions with `if`

**Time**: 1.5-2 hours
**Prerequisite**: All previous concepts

**Key Files**:
- `control_flow/README.md` - Detailed explanation
- `control_flow/examples/` - Working code examples
- `control_flow/key_takeaways.md` - Quick reference

---

## Exercises

Practice what you've learned with hands-on exercises:

### Exercise 1: Type Conversion
**Concepts Tested**: Data Types, Functions, Type Casting
**Difficulty**: Easy

Practice converting between different types and handling conversion errors.

### Exercise 2: String Manipulation
**Concepts Tested**: Strings, Functions, Loops
**Difficulty**: Easy

Build string processing functions and work with different string methods.

### Exercise 3: FizzBuzz
**Concepts Tested**: Loops, Control Flow, Pattern Matching
**Difficulty**: Easy-Medium

Classic programming challenge using `match` and loops.

### Exercise 4: Pattern Generation
**Concepts Tested**: Loops, Control Flow, Functions
**Difficulty**: Medium

Create patterns and sequences using nested loops and control flow.

---

## Learning Path

### Recommended Order
1. Start with **Variables & Mutability** (foundation)
2. Learn **Data Types** (what you work with)
3. Study **Functions** (how to organize code)
4. Master **Control Flow** (how to make decisions)
5. Complete exercises in order

### Time Estimate
- Concepts: 4-8 hours
- Exercises: 2-3 hours
- Total: 6-11 hours for complete mastery

### Progression Tips
- Don't rush - understand ownership early
- Run all examples and modify them
- Complete exercises without looking at hints first
- Practice writing your own functions

---

## Prerequisites

Before starting, ensure you have:
- Rust installed (download from [rustup.rs](https://rustup.rs/))
- A text editor or IDE (VS Code with Rust Analyzer recommended)
- Familiarity with basic programming concepts (variables, functions)
- Command line comfort

### Quick Setup
```bash
# Verify Rust installation
rustc --version
cargo --version

# Create a new project to test
cargo new hello_rust
cd hello_rust
cargo run
```

---

## Related Concepts

### What You'll Use This For
- **All Programs**: Variables, functions, and control flow are in every program
- **Module 02**: Collections and iterators build on these basics
- **Module 03**: Testing requires understanding functions
- **Module 06**: Ownership is crucial for advanced concepts

### What Comes Next
After mastering core fundamentals:
- Move to Module 02 (Standard Library)
- Explore Module 06 (Intermediate Rust - Traits, Generics)
- Practice with challenges in `/challenges/kata/`

---

## Common Mistakes & Solutions

### Mistake 1: Confusing `String` and `&str`
**Problem**: Using wrong string type
**Solution**: Read data_types section carefully; practice with both types

### Mistake 2: Forgetting `mut` for mutations
**Problem**: Trying to change immutable variable
**Solution**: Rust compiler will tell you; add `mut` keyword

### Mistake 3: Not matching all patterns in `match`
**Problem**: Match statement doesn't compile
**Solution**: Use `_` catch-all or make match exhaustive

### Mistake 4: Ownership confusion
**Problem**: Trying to use value after move
**Solution**: This is covered in later modules; reference variables for now

---

## Resources

### Official Documentation
- [The Rust Book - Chapter 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Standard Library Documentation](https://doc.rust-lang.org/std/)

### External Learning
- [Rustlings - Exercises](https://github.com/rust-lang/rustlings)
- [Exercism.org - Rust Track](https://exercism.org/tracks/rust)
- [YouTube: Rust Crash Course](https://www.youtube.com/results?search_query=rust+crash+course)

### Community
- [Rust Forum](https://users.rust-lang.org/)
- [r/rust](https://reddit.com/r/rust)
- [Rust Discord](https://discord.gg/rust-lang)

---

## Quick Reference

### Variables
```rust
let x = 5;           // immutable
let mut y = 5;       // mutable
const MAX: i32 = 42; // constant
```

### Data Types
```rust
let integer = 42i32;
let float = 3.14f64;
let boolean = true;
let character = 'a';
let tuple = (1, "hello", 3.14);
let array = [1, 2, 3, 4, 5];
```

### Functions
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Control Flow
```rust
if x > 0 { } else if x < 0 { } else { }
match value { pattern => result, _ => default }
for i in 0..10 { }
while condition { }
loop { break; }
```

---

## Module Status

-  Structure established
- ó Concept documentation in progress
- ó Code examples needed
- ó Exercises pending

## Next Steps

1. Read through each concept README
2. Run and modify all examples
3. Complete exercises without looking at solutions
4. Revisit difficult concepts
5. Move to Module 02

---

**Last Updated**: 2026-08-27
**Estimated Completion**: Phase 3
**Questions?** Refer to the Rust Book or community resources above.
