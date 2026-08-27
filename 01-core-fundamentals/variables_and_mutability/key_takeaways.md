# Key Takeaways: Variables & Mutability

## Quick Reference

### Variable Declaration
```rust
let x = 5;              // Immutable
let mut y = 5;          // Mutable
const MAX: i32 = 100;   // Constant
```

### Mutating a Variable
```rust
let mut x = 5;
x = 10;  // ✅ OK - x is mutable
```

### Shadowing
```rust
let x = 5;
let x = x + 1;          // New variable x, shadows old x
let x = "hello";        // Can change type with shadowing
```

## Essential Concepts

### 1. Immutability by Default
- All variables are immutable unless marked `mut`
- Helps prevent accidental changes
- Compiler enforces immutability

### 2. Mutability (`mut` keyword)
- Marks a variable as mutable
- Only affects that specific variable
- Must be explicit

### 3. Shadowing vs Mutation

| Aspect | Shadowing | Mutation |
|--------|-----------|----------|
| Syntax | `let x = new_value` | `x = new_value` |
| Creates new variable? | Yes | No |
| Can change type? | Yes | No |
| Requires `mut`? | No | Yes |
| Use when | Transforming data | Updating state |

### 4. Constants
- Always immutable
- Declared with `const` (not `let`)
- Require explicit type annotation
- Compile-time known values only

## Common Patterns

### Pattern 1: Immutable by default
```rust
let name = "Alice";  // Immutable, safe
```

### Pattern 2: Mutable collection building
```rust
let mut numbers = Vec::new();
numbers.push(1);
numbers.push(2);
```

### Pattern 3: Data transformation
```rust
let x = 5;
let x = x * 2;       // Shadowing
let x = x as f64;    // Shadowing with type change
```

### Pattern 4: Configuration constants
```rust
const DATABASE_URL: &str = "postgresql://localhost";
const MAX_CONNECTIONS: u32 = 100;
```

## Checklist

When declaring a variable, ask yourself:

- [ ] Will this value change?
  - Yes → Use `let mut`
  - No → Use `let`

- [ ] Is this a compile-time constant?
  - Yes → Use `const`
  - No → Use `let` or `let mut`

- [ ] Am I transforming data through steps?
  - Yes → Consider shadowing
  - No → Use `let mut`

## Error Prevention

### ❌ DON'T: Forget `mut`
```rust
let x = 5;
x = 10;  // Compile error!
```

### ✅ DO: Add `mut` when needed
```rust
let mut x = 5;
x = 10;  // OK
```

### ❌ DON'T: Type-mutate without shadowing
```rust
let x = 5;
x = "hello";  // Error: wrong type
```

### ✅ DO: Use shadowing for type changes
```rust
let x = 5;
let x = "hello";  // OK - new variable
```

## Practice Questions

1. When would you use `const` instead of `let`?
2. What's the difference between `let x = y` and `let mut x = y`?
3. Can you use shadowing to change a variable's type?
4. Why does Rust make variables immutable by default?
5. When is shadowing better than using `mut`?

## Related Concepts

- **Before**: None (foundation)
- **After**: Data Types, Functions, Ownership

## Time Estimates

- Reading this concept: 20-30 minutes
- Working through examples: 30-40 minutes
- Practice exercises: 30-60 minutes
- Total: 1-1.5 hours

## Resources

- [Rust Book: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust by Example: Variables](https://doc.rust-lang.org/rust-by-example/variable_binding.html)

---

**Status**: Foundation concept
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Beginner
