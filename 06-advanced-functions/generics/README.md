# Generics

## Overview

Generics allow you to write code that works with many types without sacrificing type safety. Instead of writing separate functions or types for each data type, you write a single generic definition that the compiler specializes for each concrete type. Generics are Rust's primary mechanism for code reuse and are pervasive throughout the language and standard library.

## Theory

### What are Generics?

Generics are placeholders for concrete types that are determined at compile time. They enable:

- **Type Reuse** - Write once, use with many types
- **Type Safety** - Full type checking at compile time
- **Zero Cost** - No runtime overhead
- **Flexibility** - Combined with traits for powerful abstractions

### Generic Instantiation

```
Generic Definition (source) → Monomorphization → Concrete Specializations (binary)

fn print<T: Display>(t: T) { }

Called with i32 → fn print_i32(t: i32) { }
Called with String → fn print_string(t: String) { }
```

### Types of Generics

1. **Generic Functions** - Functions parametrized by type
2. **Generic Structs** - Structs with generic fields
3. **Generic Enums** - Enums with generic variants
4. **Generic Traits** - Traits with generic parameters
5. **Lifetime Generics** - Code parametrized by lifetime

### Monomorphization

At compile time, Rust creates specialized versions:

```
Generic code:
fn double<T: std::ops::Mul + Clone>(t: T) -> T { t.clone() * t }

Becomes:
fn double_i32(t: i32) -> i32 { t * t }
fn double_f64(t: f64) -> f64 { t * t }
```

### Constraints and Bounds

Generics must be constrained to ensure operations are valid:

```rust
fn process<T>(t: T) { }  // Too generic - can't do anything

fn process<T: Clone>(t: T) { }  // Can clone

fn process<T: Clone + Display>(t: T) { }  // Can clone and display
```

## Syntax

### Generic Functions

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("{}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest(&chars));
}
```

### Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T: std::fmt::Display> Point<T> {
    fn print_coordinates(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

fn main() {
    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("{}", int_point.x());
}
```

### Generic Enums

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let opt: Option<i32> = Some(5);
    let res: Result<String, String> = Ok("Success".to_string());
}
```

### Multiple Type Parameters

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T: std::fmt::Display, U: std::fmt::Display> Pair<T, U> {
    fn display_both(&self) {
        println!("First: {}, Second: {}", self.first, self.second);
    }
}

fn main() {
    let pair = Pair {
        first: 5,
        second: "hello",
    };
    pair.display_both();
}
```

### Generic Trait Implementation

```rust
trait Comparable<T> {
    fn compare(&self, other: &T) -> std::cmp::Ordering;
}

impl Comparable<i32> for i32 {
    fn compare(&self, other: &i32) -> std::cmp::Ordering {
        self.cmp(other)
    }
}
```

### Where Clauses for Complex Bounds

```rust
fn process<T, U>(t: T, u: U)
where
    T: Clone + std::fmt::Display,
    U: std::fmt::Debug,
{
    // Implementation
}

struct Container<T, U>
where
    T: Clone,
    U: std::fmt::Debug,
{
    item: T,
    metadata: U,
}
```

### Generic Lifetime Parameters

```rust
struct Wrapper<'a, T> {
    reference: &'a T,
    data: T,
}

impl<'a, T: std::fmt::Display> Wrapper<'a, T> {
    fn print_ref(&self) {
        println!("{}", self.reference);
    }
}
```

## Common Patterns

### Pattern 1: Container with Generic Operations

```rust
struct Box<T> {
    item: T,
}

impl<T> Box<T> {
    fn new(item: T) -> Self {
        Box { item }
    }

    fn get(&self) -> &T {
        &self.item
    }

    fn take(self) -> T {
        self.item
    }

    fn map<U, F>(self, f: F) -> Box<U>
    where
        F: FnOnce(T) -> U,
    {
        Box {
            item: f(self.item),
        }
    }
}

fn main() {
    let box_int = Box::new(5);
    let box_str = box_int.map(|n| n.to_string());
}
```

### Pattern 2: Constrained Generic Struct

```rust
struct Repository<T>
where
    T: Clone + std::fmt::Debug,
{
    items: Vec<T>,
}

impl<T> Repository<T>
where
    T: Clone + std::fmt::Debug,
{
    fn new() -> Self {
        Repository {
            items: Vec::new(),
        }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn list(&self) {
        for item in &self.items {
            println!("{:?}", item);
        }
    }
}
```

### Pattern 3: Generic Builder Pattern

```rust
struct Builder<T> {
    value: Option<T>,
}

impl<T> Builder<T> {
    fn new() -> Self {
        Builder { value: None }
    }

    fn set(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    fn build(self) -> Option<T> {
        self.value
    }
}

fn main() {
    let result = Builder::new()
        .set(42)
        .build();
    assert_eq!(result, Some(42));
}
```

### Pattern 4: Generic Conversion Trait

```rust
trait Into<T> {
    fn into(self) -> T;
}

impl Into<f64> for i32 {
    fn into(self) -> f64 {
        self as f64
    }
}

fn accept_any<T>(t: T)
where
    T: Into<String>,
{
    let s: String = t.into();
    println!("{}", s);
}
```

### Pattern 5: Generic Collection Processing

```rust
fn apply_to_all<T, F>(items: Vec<T>, f: F)
where
    F: Fn(T),
{
    for item in items {
        f(item);
    }
}

fn filter_by_condition<T, F>(items: Vec<T>, predicate: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    items.into_iter().filter(predicate).collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let evens = filter_by_condition(numbers, |n| n % 2 == 0);
    println!("{:?}", evens);
}
```

## Common Mistakes

### Mistake 1: Overly Broad Generic

```rust
// ❌ WRONG - No constraints means can't do anything
fn process<T>(t: T) {
    println!("{}", t);  // ERROR: T doesn't implement Display
}

// ✅ CORRECT - Add necessary bounds
fn process<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}
```

### Mistake 2: Insufficient Bounds

```rust
// ❌ WRONG - Missing Clone bound
fn duplicate<T>(t: T) -> (T, T) {
    (t.clone(), t.clone())  // ERROR: no Clone
}

// ✅ CORRECT - Add Clone bound
fn duplicate<T: Clone>(t: T) -> (T, T) {
    (t.clone(), t.clone())
}
```

### Mistake 3: Mismatched Type Parameters

```rust
// ❌ WRONG - f expects same types
fn pair<T>(a: T, b: T) -> (T, T) {
    (a, b)
}

let p = pair(5, "hello");  // ERROR: i32 != &str

// ✅ CORRECT - Use different type parameters if needed
fn pair<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

let p = pair(5, "hello");
```

### Mistake 4: Lifetime Parameter Issues

```rust
// ❌ WRONG - Mismatched lifetime
fn choose<'a>(a: &'a str, b: &str) -> &'a str {
    if a.len() > b.len() { a } else { b }  // ERROR: b might not live as long
}

// ✅ CORRECT - Same lifetime
fn choose<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

### Mistake 5: Over-Constraining with Where Clauses

```rust
// ❌ WRONG - Too many constraints
struct Container<T>
where
    T: Clone + Default + Display + Debug + Serialize + Deserialize,
{
    item: T,
}

// ✅ CORRECT - Only require what you use
struct Container<T: Clone> {
    item: T,
}
```

## Real-World Examples

### Example 1: Generic Stack

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            items: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), Some(2));
}
```

### Example 2: Generic Cache with Trait Bound

```rust
use std::collections::HashMap;

trait Cacheable: Clone {
    fn size(&self) -> usize;
}

struct Cache<T: Cacheable> {
    storage: HashMap<String, T>,
    max_size: usize,
    current_size: usize,
}

impl<T: Cacheable> Cache<T> {
    fn new(max_size: usize) -> Self {
        Cache {
            storage: HashMap::new(),
            max_size,
            current_size: 0,
        }
    }

    fn insert(&mut self, key: String, value: T) {
        let size = value.size();
        if self.current_size + size <= self.max_size {
            self.storage.insert(key, value);
            self.current_size += size;
        }
    }

    fn get(&self, key: &str) -> Option<T> {
        self.storage.get(key).cloned()
    }
}
```

### Example 3: Generic Result Handler

```rust
trait ErrorHandler {
    fn handle_error(&self, error: &str);
}

struct LoggerHandler;
impl ErrorHandler for LoggerHandler {
    fn handle_error(&self, error: &str) {
        println!("Error: {}", error);
    }
}

fn process_result<T, E, H>(result: Result<T, E>, handler: &H)
where
    E: std::fmt::Display,
    H: ErrorHandler,
{
    match result {
        Ok(_) => println!("Success!"),
        Err(e) => handler.handle_error(&e.to_string()),
    }
}
```

### Example 4: Generic Tree Structure

```rust
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            match &mut self.left {
                Some(node) => node.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else {
            match &mut self.right {
                Some(node) => node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }
}
```

### Example 5: Generic Pipeline

```rust
struct Pipeline<T> {
    data: T,
}

impl<T> Pipeline<T> {
    fn new(data: T) -> Self {
        Pipeline { data }
    }

    fn map<U, F>(self, f: F) -> Pipeline<U>
    where
        F: FnOnce(T) -> U,
    {
        Pipeline {
            data: f(self.data),
        }
    }

    fn filter<F>(self, predicate: F) -> Option<Pipeline<T>>
    where
        F: FnOnce(&T) -> bool,
    {
        if predicate(&self.data) {
            Some(self)
        } else {
            None
        }
    }

    fn get(self) -> T {
        self.data
    }
}

fn main() {
    let result = Pipeline::new(5)
        .map(|x| x * 2)
        .filter(|x| x > &5)
        .map(|x| x.to_string())
        .get();
    println!("{}", result);
}
```

## Related Concepts

### Prerequisites
- Module 01: Functions, Structs
- Module 02: Collections
- Module 06: Ownership, Traits

### Follow-ups
- Advanced Generic Patterns (GAT, HRTB)
- Procedural Macros
- Type-Level Programming

## Best Practices

1. **Start Simple** - Begin with concrete types, generalize later
2. **Use Trait Bounds** - Make requirements explicit
3. **Prefer Generics to Dynamic Dispatch** - When possible for performance
4. **Name Type Parameters Clearly** - `T` for simple, `Item` for specific
5. **Avoid Over-Generalization** - Don't over-constrain
6. **Use Where Clauses** - For complex bounds
7. **Document Generic Constraints** - Why bounds are needed
8. **Test with Multiple Types** - Ensure generics work broadly

## Summary

Generics enable writing flexible, reusable code without sacrificing Rust's type safety. Through monomorphization, generics have zero runtime cost while providing the flexibility needed for powerful abstractions. Combining generics with traits creates the foundation for effective, idiomatic Rust programming.

## Practice Exercise Ideas

1. Create a generic linked list implementation
2. Build a generic binary search tree
3. Implement a generic graph data structure
4. Create a generic state machine
5. Build a generic protocol parser

