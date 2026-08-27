# Key Takeaways: Data Types

## Quick Reference

### Scalar Types
```rust
let x: i32 = 42;            // Signed integer
let y: u32 = 42;            // Unsigned integer
let z: f64 = 3.14;          // Float (default)
let b: bool = true;         // Boolean
let c: char = 'a';          // Single character
```

### Compound Types
```rust
let tup: (i32, f64) = (5, 3.14);  // Tuple
let arr: [i32; 3] = [1, 2, 3];    // Array
```

### String Types
```rust
let s1: &str = "hello";                    // String slice
let s2: String = String::from("hello");    // Owned string
```

## Type Quick Reference

| Type | Range | Use Case |
|------|-------|----------|
| i8 | -128 to 127 | Small signed |
| i32 | -2^31 to 2^31-1 | Default signed |
| u8 | 0 to 255 | Bytes, small unsigned |
| u32 | 0 to 2^32-1 | Default unsigned |
| f64 | IEEE 754 | Decimals (default) |
| bool | true/false | Logic |
| char | Unicode | Single character |

## Essential Concepts

### 1. Scalar vs Compound

**Scalar**: Single value
- Integers, floats, booleans, characters

**Compound**: Multiple values
- Tuples (different types OK)
- Arrays (same type required)

### 2. Signed vs Unsigned Integers

| Type | Can be negative? | Use when |
|------|-----------------|----------|
| i32 | Yes | Can be negative |
| u32 | No | Always positive |

### 3. String Types

| Type | Ownership | Mutability | Use when |
|------|-----------|-----------|----------|
| &str | Borrowed | No | Reading strings |
| String | Owned | Yes | Building/modifying |

### 4. Type Inference

Rust figures out types from context:
```rust
let x = 5;      // i32
let y = 3.14;   // f64
let s = "hi";   // &str
```

Provide explicit type when ambiguous:
```rust
let x: u32 = "42".parse().unwrap();  // Need type hint
```

## Common Patterns

### Pattern 1: Destructuring tuples
```rust
let person = ("Alice", 30);
let (name, age) = person;
```

### Pattern 2: String conversions
```rust
let num = 42;
let s = num.to_string();           // to String
let num: i32 = "42".parse().unwrap();  // to i32
```

### Pattern 3: Type casting
```rust
let x = 42u8;
let y = x as u32;  // u8 to u32
```

### Pattern 4: Array initialization
```rust
let arr = [0; 5];  // [0, 0, 0, 0, 0]
```

## Checklist

When choosing a type, consider:

- [ ] **Integer type**: Will it be negative? How large?
  - Negative? → `i32`
  - Always positive? → `u32`

- [ ] **Float type**: Need decimals?
  - Yes → `f64` (default)

- [ ] **String type**: Do I own it?
  - Own/modify? → `String`
  - Read only? → `&str`

- [ ] **Collection**: Different types?
  - Yes → Tuple
  - No → Array (or Vec for dynamic)

## Error Prevention

### ❌ DON'T: Confuse char and &str
```rust
let c = "a";  // This is &str, not char
```

### ✅ DO: Use correct quotes
```rust
let c = 'a';  // char with single quotes
let s = "a";  // &str with double quotes
```

### ❌ DON'T: Lose precision silently
```rust
let f = 3.99;
let i = f as i32;  // Results in 3!
```

### ✅ DO: Be aware of type casting
```rust
let f = 3.99;
let i = f as i32;  // Explicitly truncates to 3
```

## Practice Questions

1. What's the difference between `i32` and `u32`?
2. Why use `&str` instead of `String`?
3. How do you destructure a tuple?
4. What happens when you cast `f64` to `i32`?
5. When would you use an array vs tuple?

## Type Reference

### All Integer Types
```
Signed: i8, i16, i32, i64, i128, isize
Unsigned: u8, u16, u32, u64, u128, usize
Default (integer literal): i32
```

### All Float Types
```
f32 - 32-bit float
f64 - 64-bit float (default)
```

### Special Values
```rust
f64::INFINITY      // Positive infinity
f64::NEG_INFINITY  // Negative infinity
f64::NAN           // Not a number
```

## Common Conversions

```rust
// String to number
let num: i32 = "42".parse().unwrap();

// Number to string
let s = format!("{}", 42);
let s = (42).to_string();

// Types casting
let x = 42u8 as u32;
let x = 42i32 as f64;
let x = 65u8 as char;  // 'A'
```

## Related Concepts

- **Before**: Variables & Mutability
- **After**: Functions, Control Flow

## Time Estimates

- Reading this concept: 20-30 minutes
- Working through examples: 30-40 minutes
- Practice exercises: 30-60 minutes
- Total: 1-1.5 hours

## Resources

- [Rust Book: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust by Example: Types](https://doc.rust-lang.org/rust-by-example/types.html)

---

**Status**: Foundation concept
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Beginner
