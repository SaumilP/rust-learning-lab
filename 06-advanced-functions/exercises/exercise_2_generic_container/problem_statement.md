# Exercise 2: Generic Container with Trait Bounds

## Difficulty: Medium
## Concepts Tested: Generics, Trait Bounds
## Prerequisites: Module 06 concepts

## Problem Statement

Implement a generic `Box<T>` that:

1. Stores any type of value
2. Provides get() that returns reference (requires Clone trait)
3. Provides map() that transforms value
4. Implements Display only for types that implement Display
5. Track type information

## Requirements

- Generic struct with type parameter
- Generic impl blocks with trait bounds
- Conditional impl (only when T: Display)
- Trait bound usage

## Expected Output

```
Box<i32>: 42
Box<String>: Hello

Display-only operations:
Integer formatted: 42
String formatted: Hello

Mapped operations:
i32 mapped to String: "42"
String mapped to uppercase: "HELLO"
```

