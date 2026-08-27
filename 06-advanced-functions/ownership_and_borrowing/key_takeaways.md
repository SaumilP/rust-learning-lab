# Ownership and Borrowing - Key Takeaways

## Core Rules

1. Each value has **one owner**
2. Ownership can be **moved** to another variable
3. Values can be **borrowed** without transferring ownership
4. When owner goes out of scope, value is dropped

## Move vs Borrow vs Copy

```rust
// Move - ownership transferred
let s1 = String::from("hello");
let s2 = s1;  // Ownership moves to s2
// s1 is now invalid

// Borrow - temporary access, ownership stays
let s1 = String::from("hello");
let s2 = &s1;  // Borrow, s1 still owns

// Copy - automatic copy for cheap types
let x = 5;
let y = x;  // Automatic copy (i32 implements Copy)
// Both x and y valid
```

## Reference Types

```rust
&T           // Immutable reference (read-only)
&mut T       // Mutable reference (read/write)
&[T]         // Slice (reference to portion)
&str         // String slice (reference to String)
```

## Borrowing Rules (Borrow Checker)

✓ **Multiple immutable** borrows allowed
✓ **One mutable** borrow allowed
✗ Can't mix immutable and mutable in same scope
✗ Reference must outlive the borrow

```rust
let s = String::from("hello");

// OK: multiple immutable
let r1 = &s;
let r2 = &s;

// ERROR: mutable while immutable active
let r3 = &mut s;

// OK after immutable borrows end
let r3 = &mut s;
```

## Ownership Transfer Patterns

```rust
// Pattern 1: Function takes ownership
fn takes_ownership(s: String) { }
let s = String::from("hello");
takes_ownership(s);
// s invalid after this

// Pattern 2: Function borrows
fn borrows(s: &String) { }
let s = String::from("hello");
borrows(&s);
// s still valid

// Pattern 3: Function returns ownership
fn gives_ownership() -> String {
    String::from("hello")
}
let s = gives_ownership();
// s owns the returned value

// Pattern 4: Modify and return ownership
fn append_and_return(mut s: String) -> String {
    s.push_str(" world");
    s
}
```

## Mutable Borrowing

```rust
fn main() {
    let mut s = String::from("hello");
    modify(&mut s);
    println!("{}", s);
}

fn modify(s: &mut String) {
    s.push_str(" world");
}
```

## Lifetimes

```rust
// Lifetime annotations required when:
// - Reference returned from function
// - Struct contains references

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

struct Person<'a> {
    name: &'a str,
    age: u32,
}
```

## Common Patterns

| Task | Pattern |
|------|---------|
| Read data | `fn func(s: &String)` |
| Modify data | `fn func(s: &mut String)` |
| Transfer ownership | `fn func(s: String)` |
| Return modified | `fn func(mut s: String) -> String { s }` |
| Return reference | `fn func<'a>(s: &'a str) -> &'a str` |

## Important Notes

✓ Stack types (i32, bool, etc.) implement Copy
✓ Heap types (String, Vec) implement Move
✓ Clone() makes explicit copy
✓ Drop trait runs when value goes out of scope
✓ Ownership rules prevent memory leaks
✓ Borrowing allows sharing without transferring
✓ Lifetimes ensure references are valid

## Common Mistakes

1. ❌ Using value after move
   ```rust
   let s = String::from("hello");
   let s2 = s;
   println!("{}", s);  // ERROR
   ```

2. ❌ Multiple mutable borrows
   ```rust
   let mut s = String::from("hello");
   let r1 = &mut s;
   let r2 = &mut s;  // ERROR
   ```

3. ❌ Dangling reference
   ```rust
   fn dangle() -> &String {
       let s = String::from("hello");
       &s  // ERROR: reference outlives value
   }
   ```

4. ❌ Missing lifetime annotation
   ```rust
   fn longest(s1: &str, s2: &str) -> &str {  // ERROR: missing 'a
       if s1.len() > s2.len() { s1 } else { s2 }
   }
   ```

5. ❌ Borrow after move
   ```rust
   let s = String::from("hello");
   let s2 = s;
   let len = s.len();  // ERROR: s moved
   ```

## Quick Reference

```rust
use std::io::{self, Write};

// Ownership transfer
let s1 = String::from("hello");
let s2 = s1;  // Move

// Immutable borrow
let r1 = &s2;

// Mutable borrow
let mut s = String::from("hello");
let r = &mut s;
r.push_str(" world");

// Function with borrow
fn print_string(s: &String) {
    println!("{}", s);
}

// Function with mutable borrow
fn modify_string(s: &mut String) {
    s.push('!');
}

// Lifetime annotation
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// Struct with lifetime
struct Pair<'a> {
    first: &'a str,
    second: &'a str,
}
```

## Copy Types

Implement Copy (auto copy instead of move):
- Integers: i32, u64, etc.
- Floats: f32, f64
- Booleans: bool
- Characters: char
- Tuples: (i32, i32)
- Arrays: [i32; 3]

## Ownership Checklist

✓ Does function need to modify data? Use `&mut T`
✓ Does function only need to read? Use `&T`
✓ Does function need to own data? Use `T` (move)
✓ Need to return ownership? Return `T`
✓ Need to return reference? Add lifetime parameter
✓ Can't borrow if mutable borrow active? Wait for it to end

## Related Concepts

- Smart pointers (Box, Rc, RefCell)
- Traits with lifetime bounds
- Generic types with lifetime parameters
- Memory layout and alignment

