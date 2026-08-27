# Ownership and Borrowing

## Overview

Ownership and borrowing are the foundation of Rust's memory safety guarantees. Unlike languages with garbage collection, Rust uses a sophisticated ownership system to manage memory automatically without runtime overhead. Understanding ownership is essential for writing safe, efficient Rust code. This module covers how Rust manages memory through ownership transfer, borrowing patterns, and lifetimes - the features that make Rust both safe and fast.

## Theory

### The Ownership System

Rust manages memory through three core principles:

1. **Each value has one owner** - Exactly one variable holds the value
2. **Ownership can be transferred** - Move semantics pass ownership to new owner
3. **Borrowing allows temporary access** - References don't transfer ownership

### Why Ownership Matters

- **Memory Safety** - No dangling pointers, no use-after-free
- **No Garbage Collection** - Automatic cleanup with zero runtime cost
- **Thread Safety** - Ownership rules prevent data races
- **Performance** - Compiler optimizes based on ownership patterns

### The Stack vs The Heap

```
Stack: Fast, limited size, automatically freed
  - Primitive types (i32, bool, char)
  - Fixed-size structs
  - Function parameters

Heap: Flexible size, slower, must be managed
  - Strings (String, not &str)
  - Collections (Vec, HashMap)
  - Dynamic data
```

### Three Transfer Mechanisms

1. **Move** - Ownership transferred, original becomes invalid
2. **Clone** - Create independent copy (expensive)
3. **Copy** - Automatic copy for cheap types (primitives)

### Borrowing Mechanics

- **Immutable Borrow** (`&T`) - Read access only, multiple allowed
- **Mutable Borrow** (`&mut T`) - Write access, only one allowed
- **Lifetime** (`'a`) - How long borrow is valid

### Borrowing Rules

The Borrow Checker enforces:
1. One mutable reference XOR multiple immutable references
2. References must be valid for their lifetime
3. No mixing mutable and immutable borrows in same scope

## Syntax

### Basic Ownership Transfer (Move)

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // Ownership moved from s1 to s2

    // println!("{}", s1);  // ERROR: s1 no longer owns the value
    println!("{}", s2);     // OK: s2 owns the value
}
```

### Ownership Transfer via Function

```rust
fn takes_ownership(s: String) {
    println!("{}", s);  // s owns the value
    // Value dropped here when s goes out of scope
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);  // ERROR: ownership transferred
}
```

### Returning Ownership from Function

```rust
fn gives_ownership() -> String {
    String::from("hello")  // Ownership transferred to caller
}

fn main() {
    let s = gives_ownership();  // s now owns the value
    println!("{}", s);          // OK
}
```

### Borrowing (Immutable References)

```rust
fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);  // Borrow: s still owns the value
    println!("Length: {}", len);
    println!("String: {}", s);  // OK: s still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()  // Read access only
    // s dropped but String not dropped (still owned by caller)
}
```

### Borrowing (Mutable References)

```rust
fn main() {
    let mut s = String::from("hello");
    change_string(&mut s);  // Mutable borrow
    println!("{}", s);       // OK: s still valid and modified
}

fn change_string(s: &mut String) {
    s.push_str(" world");  // Write access
}
```

### Copy vs Clone

```rust
fn main() {
    // Primitives implement Copy - automatically copied
    let x = 5;
    let y = x;  // Copy (not move)
    println!("{}, {}", x, y);  // OK: both valid

    // Strings don't implement Copy - must clone or borrow
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Explicit copy
    println!("{}, {}", s1, s2);  // OK: both valid
}
```

### Lifetimes

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1  // Reference valid for 'a lifetime
    } else {
        s2  // Reference valid for 'a lifetime
    }
}

fn main() {
    let s1 = "hello";
    let s2 = "world";
    let result = longest(s1, s2);
    println!("{}", result);
}
```

### Mutable vs Immutable Mixing

```rust
fn main() {
    let mut x = 5;

    let r1 = &x;      // Immutable borrow
    let r2 = &x;      // Immutable borrow (allowed)
    // let r3 = &mut x;  // ERROR: can't mix mutable with immutable

    println!("{}, {}", r1, r2);

    let r3 = &mut x;  // OK: no immutable borrows active
    *r3 += 1;
    println!("{}", r3);
}
```

## Common Patterns

### Pattern 1: Ownership Transfer with Tuple

```rust
fn calculate_and_return_string(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)  // Return both ownership and result
}

fn main() {
    let s = String::from("hello");
    let (s, len) = calculate_and_return_string(s);
    println!("Length: {}, String: {}", len, s);
}
```

### Pattern 2: Borrowing to Avoid Ownership Transfer

```rust
fn process_slice(s: &[u8]) -> usize {
    s.len()  // Only need to read
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let len = process_slice(&data);  // Borrow, don't transfer
    println!("Length: {}, Vec: {:?}", len, data);  // data still valid
}
```

### Pattern 3: Mutable Borrowing for Modification

```rust
fn append_exclamation(s: &mut String) {
    s.push('!');
}

fn main() {
    let mut s = String::from("hello");
    append_exclamation(&mut s);
    println!("{}", s);  // "hello!"
}
```

### Pattern 4: Scoped Borrowing

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &s;  // Immutable borrow starts
        println!("{}", r1);
    }  // Immutable borrow ends

    let r2 = &mut s;  // Now OK: no active borrows
    r2.push_str(" world");
    println!("{}", r2);
}
```

### Pattern 5: Reference to Reference

```rust
fn main() {
    let x = 5;
    let r1 = &x;
    let r2 = &r1;      // Reference to reference
    let r3 = &r2;      // Reference to reference to reference

    println!("{}", ***r3);  // Dereference: 5
}
```

## Common Mistakes

### Mistake 1: Dangling Reference

```rust
// ❌ WRONG - Dangling reference
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ERROR: reference outlives value
}

// ✅ CORRECT - Transfer ownership or use lifetime
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // Ownership transferred to caller
}
```

### Mistake 2: Multiple Mutable Borrows

```rust
// ❌ WRONG - Multiple mutable borrows
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;  // ERROR: only one mutable borrow allowed
}

// ✅ CORRECT - One mutable borrow at a time
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(" world");
    // r1 ends here

    let r2 = &mut s;  // OK: previous borrow ended
    r2.push_str("!");
}
```

### Mistake 3: Mixing Mutable and Immutable Borrows

```rust
// ❌ WRONG - Immutable and mutable mixed
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;  // ERROR: can't borrow as mutable
}

// ✅ CORRECT - Separate scopes
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // r1 and r2 end here

    let r3 = &mut s;  // OK: previous borrows ended
}
```

### Mistake 4: Forgetting Ownership Transfer

```rust
// ❌ WRONG - Using after move
fn main() {
    let s = String::from("hello");
    let s2 = s;  // Ownership moved
    println!("{}", s);  // ERROR: s no longer owns value
}

// ✅ CORRECT - Borrow instead
fn main() {
    let s = String::from("hello");
    let len = s.len();  // Borrow with function
    println!("{}", s);  // OK: s still owns value
}
```

### Mistake 5: Lifetime Mismatches

```rust
// ❌ WRONG - Dangling reference
fn get_reference<'a>(x: &str, y: &'a str) -> &'a str {
    x  // ERROR: x's lifetime shorter than 'a
}

// ✅ CORRECT - Matching lifetimes
fn get_reference<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## Real-World Examples

### Example 1: String Manipulation with Borrowing

```rust
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn reverse_string(s: &mut String) {
    let bytes = unsafe { s.as_bytes_mut() };
    bytes.reverse();
}

fn main() {
    let mut text = String::from("hello world rust");

    // Borrow for reading
    let words = count_words(&text);
    println!("Words: {}", words);

    // text still valid
    println!("Original: {}", text);

    // Mutable borrow for modification
    reverse_string(&mut text);
    println!("Reversed: {}", text);
}
```

### Example 2: Vector Operations with Ownership

```rust
fn sum_vector(v: &[i32]) -> i32 {
    v.iter().sum()
}

fn append_value(v: &mut Vec<i32>, val: i32) {
    v.push(val);
}

fn process_and_return(mut v: Vec<i32>) -> Vec<i32> {
    v.push(100);
    v  // Return modified vector
}

fn main() {
    let mut numbers = vec![1, 2, 3];

    // Borrow for reading
    let total = sum_vector(&numbers);
    println!("Sum: {}", total);

    // Mutable borrow for modification
    append_value(&mut numbers, 4);
    println!("After append: {:?}", numbers);

    // Transfer ownership
    let numbers = process_and_return(numbers);
    println!("After process: {:?}", numbers);
}
```

### Example 3: Struct with Lifetime Parameters

```rust
struct Person<'a> {
    name: &'a str,
    age: u32,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, age: u32) -> Self {
        Person { name, age }
    }

    fn introduce(&self) {
        println!("I'm {} and I'm {} years old", self.name, self.age);
    }
}

fn main() {
    let name = String::from("Alice");
    let person = Person::new(&name, 30);
    person.introduce();
    println!("{}", name);  // name still valid
}
```

### Example 4: Collection of References

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = String::from("rust");

    let strings = vec![&s1, &s2, &s3];  // Vector of references

    for s in strings {
        println!("{}", s);
    }

    // All original strings still valid
    println!("s1: {}", s1);
}
```

### Example 5: Function Chaining with Borrowing

```rust
struct Text {
    content: String,
}

impl Text {
    fn new(content: &str) -> Self {
        Text {
            content: content.to_string(),
        }
    }

    fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    fn uppercase(&mut self) {
        self.content = self.content.to_uppercase();
    }

    fn get_content(&self) -> &str {
        &self.content
    }
}

fn main() {
    let mut text = Text::new("hello world");

    println!("Words: {}", text.word_count());  // Immutable borrow
    println!("Content: {}", text.get_content());  // Immutable borrow

    text.uppercase();  // Mutable borrow
    println!("Modified: {}", text.get_content());
}
```

## Related Concepts

### Prerequisites
- Module 01: Variables, Functions, Scope
- Module 02: Collections, String operations
- Module 03: Pattern matching, error handling

### Follow-ups
- Traits and Polymorphism (bounded by lifetime/ownership)
- Generics (combining with ownership rules)
- Advanced Lifetime Patterns (self-referential structs)
- Smart Pointers (Box, Rc, RefCell)

## Best Practices

1. **Prefer Borrowing** - Use references instead of moving when possible
2. **Use Mutable Borrowing** - Only when modification needed
3. **Scope Borrows** - Keep borrow scopes as small as possible
4. **Explicit Lifetimes** - Annotate when compiler can't infer
5. **Avoid Cloning** - Only clone when necessary
6. **Document Ownership** - In comments if not obvious
7. **Return Ownership** - If function modifies data
8. **Use Slices** - Instead of references to entire collections

## Summary

Ownership and borrowing are Rust's answer to memory safety without garbage collection. By enforcing strict rules about who owns data and how it can be accessed, Rust prevents entire categories of bugs while maintaining zero-cost abstractions. Understanding these concepts transforms you from a Rust novice to someone who can write safe, efficient code.

## Practice Exercise Ideas

1. Create a function that takes ownership and returns modified data
2. Build a text processor using borrowing patterns
3. Implement a vector wrapper with ownership semantics
4. Create structs with lifetime parameters
5. Write code demonstrating all mutable/immutable borrow scenarios

