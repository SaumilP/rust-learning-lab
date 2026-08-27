# Key Takeaways: Control Flow

## Quick Reference

### if/else Expression
```rust
let x = 5;

if x > 3 {
    println!("greater");
} else {
    println!("not greater");
}

let result = if x > 3 { 5 } else { 6 };
```

### match Expression
```rust
match value {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"),
}

let description = match value {
    1 => "one",
    _ => "other",
};
```

### Loops
```rust
loop { break; }               // Infinite loop
while condition { }           // While loop
for i in 0..5 { }            // For loop
for item in collection { }   // Iterate
```

## Essential Concepts

### 1. if is an Expression
```rust
// Returns a value
let x = if condition { 5 } else { 6 };

// Must return same type
let x = if condition { 5 } else { 6 };      // ✅ OK
let x = if condition { 5 } else { "five" }; // ❌ Error
```

### 2. match is Exhaustive
```rust
match x {
    1 => {},
    2 => {},
    _ => {},  // MUST cover all cases or use _
}
```

### 3. Loop Types

| Loop | Use | Example |
|------|-----|---------|
| `loop` | Infinite until break | `loop { break; }` |
| `while` | Condition true | `while x < 10 { }` |
| `for` | Iterate | `for i in 0..5 { }` |

### 4. match Patterns
```rust
match x {
    1 => {},                    // Exact match
    1 | 2 => {},               // OR
    1..=5 => {},               // Range (inclusive)
    1..5 => {},                // Range (exclusive)
    _ => {},                   // Catch-all
}
```

### 5. Loop Control
```rust
break;      // Exit loop
continue;   // Next iteration
break value; // Return value from loop
```

## Common Patterns

### Pattern 1: Simple if/else
```rust
if age >= 18 {
    println!("adult");
} else {
    println!("minor");
}
```

### Pattern 2: match with multiple cases
```rust
match day {
    1 => println!("Monday"),
    2 => println!("Tuesday"),
    _ => println!("Other"),
}
```

### Pattern 3: Range matching
```rust
match score {
    90..=100 => println!("A"),
    80..=89 => println!("B"),
    _ => println!("F"),
}
```

### Pattern 4: for loop over range
```rust
for i in 0..5 {
    println!("{}", i);  // 0, 1, 2, 3, 4
}
```

### Pattern 5: Early return with break
```rust
let result = loop {
    if condition {
        break value;
    }
};
```

### Pattern 6: Nested loops
```rust
for i in 0..3 {
    for j in 0..3 {
        println!("{}, {}", i, j);
    }
}
```

## Checklist

For if/else:
- [ ] Does condition return bool?
- [ ] Are both branches same type?
- [ ] Is condition clear and readable?

For match:
- [ ] Are all cases covered (or `_`)?
- [ ] Is each arm clear?
- [ ] Could this be simpler with if?

For loops:
- [ ] Which loop type is best?
- [ ] Is exit condition clear?
- [ ] Is variable scope correct?

## Error Prevention

### ❌ DON'T: Forget => in match
```rust
match x {
    1 println!("one"),  // Missing =>
}
```

### ✅ DO: Include =>
```rust
match x {
    1 => println!("one"),
}
```

### ❌ DON'T: Different types in if
```rust
let x = if true { 5 } else { "six" };
```

### ✅ DO: Same types
```rust
let x = if true { 5 } else { 6 };
```

### ❌ DON'T: Incomplete match
```rust
match x {
    1 => {},
    2 => {},
    // Missing case!
}
```

### ✅ DO: Exhaustive match
```rust
match x {
    1 => {},
    2 => {},
    _ => {},
}
```

## Practice Questions

1. When is `if` used as expression vs statement?
2. Why must all `if` branches return same type?
3. What does `_` mean in match?
4. Which loop type would you use for a range?
5. When would you use `continue` vs `break`?

## if vs match

Use `if` when:
- Simple boolean condition
- Two alternatives

Use `match` when:
- Multiple specific values
- Pattern matching
- Exhaustive coverage needed

## Loop Selection

| Need | Use |
|------|-----|
| Infinite until break | `loop` |
| While condition true | `while` |
| Iterate over range | `for` |
| Iterate over collection | `for item in` |

## Related Concepts

- **Before**: Variables, Data Types, Functions
- **After**: Ownership, Collections

## Time Estimates

- Reading this concept: 30-40 minutes
- Working through examples: 40-50 minutes
- Practice exercises: 30-60 minutes
- Total: 1.5-2 hours

## Resources

- [Rust Book: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust by Example: Flow Control](https://doc.rust-lang.org/rust-by-example/flow_control.html)

---

**Status**: Core concept
**Importance**: ⭐⭐⭐⭐⭐ (Critical)
**Difficulty**: Beginner
