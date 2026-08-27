# Concept: Common Traits

## Overview

Traits are Rust's way of defining shared behavior. They are similar to interfaces in other languages but more powerful. Understanding common traits is essential because much of Rust's expressiveness comes from how traits enable generic programming and flexible abstractions. This concept covers the most important traits you'll encounter and how to use them effectively.

## Learning Objectives

By the end of this concept, you will understand:
- What traits are and how they work
- The difference between using and implementing traits
- Most important standard library traits
- How traits enable generic programming
- Trait bounds and where clauses
- Common trait patterns
- When to implement traits for custom types

## Theory

### What is a Trait?

A trait is a collection of methods defined for an unknown type. It's Rust's mechanism for defining shared behavior across different types.

```rust
// Define a trait
trait Drawable {
    fn draw(&self);
}

// Implement for a type
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}
```

### Traits vs Inheritance

Unlike traditional inheritance, Rust uses composition through traits:

```rust
// Multiple trait implementation
impl Drawable for Circle { }
impl Serializable for Circle { }
impl Cloneable for Circle { }

// Circle has all three behaviors without hierarchies
```

### Core Traits in Standard Library

The Rust standard library defines several core traits that are used throughout:

**Copy and Clone**

```rust
// Copy - implicit copying (only for simple types)
let x: i32 = 5;
let y = x;      // Copied, x still valid

// Clone - explicit deep copy
let s1 = String::from("hello");
let s2 = s1.clone();  // Deep copy
```

**Key difference:**
- `Copy` is implicit, automatic
- `Clone` is explicit, may be expensive

```rust
// Most primitives implement Copy
let a: i32 = 5;
let b = a;      // Copied

// String does not implement Copy
let s1 = String::from("hello");
let s2 = s1;    // Moved, not copied
let s3 = s1.clone();  // Explicit clone
```

**Eq, PartialEq, Ord, PartialOrd**

These traits enable comparison operations:

```rust
// PartialEq - equality (==, !=)
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 1, y: 2 };
println!("{}", p1 == p2);  // true

// Ord - total ordering (<, >, <=, >=)
#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Item {
    priority: u32,
}
```

**Hash**

Enables using types as HashMap keys:

```rust
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Key {
    id: u32,
}

let mut map = HashMap::new();
map.insert(Key { id: 1 }, "value");

// Without Hash, Eq, PartialEq: compile error
```

**Debug**

Enables printing with `{:?}`:

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

let p = Person { name: "Alice".to_string(), age: 30 };
println!("{:?}", p);   // Person { name: "Alice", age: 30 }
println!("{:#?}", p);  // Pretty-printed
```

**Display**

Enables printing with `{}`:

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 1, y: 2 };
println!("{}", p);  // (1, 2)
```

**Default**

Provides default values:

```rust
#[derive(Default)]
struct Config {
    timeout: u32,
    retries: u32,
}

let c = Config::default();  // Uses field defaults

// Or implement manually
impl Default for Config {
    fn default() -> Self {
        Config {
            timeout: 30,
            retries: 3,
        }
    }
}
```

### Derive Macros

The `#[derive(...)]` attribute automatically implements traits:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Automatically implements:
// - Debug (for {:?} printing)
// - Clone (explicit deep copy)
// - Copy (implicit copying)
// - PartialEq (== operator)
```

**Common derivable traits:**
- `Debug` - for debugging printing
- `Clone` - for explicit copying
- `Copy` - for implicit copying
- `Default` - for default values
- `PartialEq` - for equality comparison
- `Eq` - for total equality
- `PartialOrd` - for partial ordering
- `Ord` - for total ordering
- `Hash` - for use in HashMap/HashSet

### Iterator Trait

```rust
// The trait definition (simplified)
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// Implementing Iterator
struct CountUp {
    current: u32,
    max: u32,
}

impl Iterator for CountUp {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current <= self.max {
            self.current += 1;
            Some(self.current - 1)
        } else {
            None
        }
    }
}
```

### Trait Bounds

Trait bounds specify that a generic type must implement certain traits:

```rust
// Single trait bound
fn print_debug<T: Debug>(value: &T) {
    println!("{:?}", value);
}

// Multiple trait bounds
fn serialize<T: Debug + Clone>(value: &T) {
    let copy = value.clone();
    println!("{:?}", copy);
}

// Where clause (alternative syntax)
fn serialize<T>(value: &T)
where
    T: Debug + Clone,
{
    let copy = value.clone();
    println!("{:?}", copy);
}
```

### Common Trait Patterns

**Generic functions with trait bounds**

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

println!("{}", largest(&[1, 2, 3]));      // 3
println!("{}", largest(&[1.5, 2.1, 0.5])); // 2.1
```

**Trait objects (dynamic dispatch)**

```rust
trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

// Trait object: &dyn Trait
let animals: Vec<&dyn Animal> = vec![
    &Dog,
    &Cat,
];

for animal in animals {
    animal.speak();
}
```

## Syntax

### Defining a Trait

```rust
trait Name {
    fn method_name(&self);
    fn method_with_return(&self) -> i32;
    fn method_with_params(&self, x: i32) -> String;
}
```

### Implementing a Trait

```rust
impl TraitName for TypeName {
    fn method_name(&self) {
        // Implementation
    }
}
```

### Derive Macro

```rust
#[derive(Debug, Clone, PartialEq)]
struct MyType {
    field: i32,
}
```

### Trait Bounds

```rust
// In function signature
fn function<T: TraitName>(param: T) { }

// Multiple bounds
fn function<T: Trait1 + Trait2>(param: T) { }

// With where clause
fn function<T>(param: T)
where
    T: Trait1 + Trait2,
{ }
```

### Trait Objects

```rust
let obj: &dyn TraitName = &value;
let obj: Box<dyn TraitName> = Box::new(value);
```

## Common Patterns

### Pattern 1: Deriving comparison traits

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
}

let p1 = Person { name: "Alice".to_string(), age: 30 };
let p2 = Person { name: "Bob".to_string(), age: 25 };

println!("{}", p1 > p2);  // Uses Ord
```

### Pattern 2: Custom Display implementation

```rust
use std::fmt;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

let c = Color { r: 255, g: 0, b: 128 };
println!("{}", c);  // rgb(255, 0, 128)
```

### Pattern 3: Generic function with bounds

```rust
fn sort_and_display<T: Ord + std::fmt::Display>(mut items: Vec<T>) {
    items.sort();
    for item in items {
        println!("{}", item);
    }
}
```

### Pattern 4: Trait objects for polymorphism

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Rectangle { width: 10.0, height: 5.0 }),
];

for shape in shapes {
    println!("Area: {}", shape.area());
}
```

### Pattern 5: Conditional trait implementation

```rust
// Only implement Clone if T implements Clone
impl<T: Clone> Clone for Container<T> {
    fn clone(&self) -> Self {
        Container {
            data: self.data.clone(),
        }
    }
}
```

### Pattern 6: Default implementation in trait

```rust
trait Animal {
    fn speak(&self);

    fn greet(&self) {
        println!("Hello!");
        self.speak();
    }
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
    // greet() inherited with default implementation
}

let dog = Dog;
dog.greet();  // Prints "Hello!" then "Woof!"
```

## Common Mistakes

### Mistake 1: Forgetting trait import

```rust
// ❌ Error: Drop trait methods not in scope
let s = String::from("hello");

// ✅ Correct: import trait
use std::ops::Drop;
```

### Mistake 2: Not implementing required methods

```rust
// ❌ Error: trait method not implemented
trait Animal {
    fn speak(&self);
}

impl Animal for Dog {
    // Missing speak() implementation
}

// ✅ Correct
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
```

### Mistake 3: Type mismatch with trait bounds

```rust
// ❌ Error: String doesn't implement Copy
fn print_copy<T: Copy>(value: T) {
    println!("{:?}", value);
}

print_copy(String::from("hello"));

// ✅ Correct: use Clone instead
fn print_cloned<T: Clone>(value: T) {
    let copy = value.clone();
    println!("{:?}", copy);
}
```

### Mistake 4: Forgetting dyn keyword for trait objects

```rust
// ❌ Error: need dyn keyword
let obj: &Animal = &dog;

// ✅ Correct
let obj: &dyn Animal = &dog;
```

### Mistake 5: Conflicting trait implementations

```rust
// ❌ Error: can't derive and implement
#[derive(Clone)]
struct MyType;

impl Clone for MyType {
    fn clone(&self) -> Self { ... }
}

// ✅ Use one or the other
#[derive(Clone)]
struct MyType;
```

## Real-World Examples

### Example 1: Custom type in HashMap

```rust
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct UserId(u32);

let mut users = HashMap::new();
users.insert(UserId(1), "Alice");
users.insert(UserId(2), "Bob");

println!("{:?}", users);
```

### Example 2: Comparable struct

```rust
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Product {
    id: u32,
    name: String,
    price: u32,
}

impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl Ord for Product {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

let mut products = vec![...];
products.sort();  // Sorts by price
```

### Example 3: Generic function for any comparable type

```rust
fn find_max<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    if items.is_empty() {
        return None;
    }

    let mut max = items[0];
    for &item in items {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

println!("{:?}", find_max(&[1, 5, 3, 9, 2]));      // Some(9)
println!("{:?}", find_max(&[1.5, 2.1, 0.5]));      // Some(2.1)
println!("{:?}", find_max(&["apple", "zebra"]));   // Some("zebra")
```

### Example 4: Polymorphic behavior with trait objects

```rust
trait Logger {
    fn log(&self, msg: &str);
}

struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, msg: &str) {
        println!("[INFO] {}", msg);
    }
}

struct FileLogger;
impl Logger for FileLogger {
    fn log(&self, msg: &str) {
        // Write to file
    }
}

fn log_event(logger: &dyn Logger, msg: &str) {
    logger.log(msg);
}

let console = ConsoleLogger;
let file = FileLogger;

log_event(&console, "User login");
log_event(&file, "User logout");
```

### Example 5: Conditional trait implementation

```rust
struct Container<T> {
    data: T,
}

impl<T: Clone> Container<T> {
    fn duplicate(&self) -> Container<T> {
        Container {
            data: self.data.clone(),
        }
    }
}

// Works for Cloneable types
let c = Container { data: "hello".to_string() };
let c2 = c.duplicate();

// Wouldn't work for non-Cloneable types
// let c3 = Container { data: something_not_clone };
// c3.duplicate();  // Error
```

## Related Concepts

### Prerequisites
- **Functions** - Traits use function syntax
- **Generics** - Traits work with generic types
- **Structs** - Traits implement for types

### What comes next
- **Advanced Traits** - Associated types, trait objects
- **Lifetimes** - Traits with lifetime parameters
- **Error Handling** - From trait for conversions

### Cross-references
- Module 01: Basic usage of traits
- Module 02: Collections implement traits (Iterator, etc.)
- Module 06: Advanced trait patterns

## Best Practices

### Use derive for simple cases

```rust
// ✅ Good: derives handle most cases
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// ❌ Avoid: manual implementation when derive works
impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ...
    }
}
```

### Prefer trait bounds in signatures

```rust
// ✅ Clear what types can be passed
fn process<T: Debug + Clone>(value: &T) { }

// ✅ More readable with where clause for complex bounds
fn process<T>(value: &T)
where
    T: Debug + Clone + PartialEq,
{ }
```

### Use trait objects for collections of different types

```rust
// ✅ Good: dynamic dispatch via trait objects
let handlers: Vec<Box<dyn Handler>> = vec![
    Box::new(ConsoleHandler),
    Box::new(FileHandler),
];

// ❌ Complicated: enum-based approach
enum Handler {
    Console(ConsoleHandler),
    File(FileHandler),
}
```

### Implement Display and Debug for custom types

```rust
// ✅ Good: both implementations
#[derive(Debug)]
struct User {
    name: String,
    id: u32,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (id: {})", self.name, self.id)
    }
}
```

## Summary

- **Traits** define shared behavior across types
- **Derive macros** automatically implement common traits
- **Trait bounds** specify type constraints in generics
- **Trait objects** enable dynamic dispatch (dyn Trait)
- **Common traits**: Copy, Clone, Eq, Debug, Display, Default
- **Generic functions** using traits are more flexible than concrete types
- **Trait implementations** can be conditional on other traits

## Key Takeaways

1. Traits enable generic programming and flexible abstractions
2. Use `#[derive(...)]` for automatic trait implementation
3. Trait bounds constrain generic types to those implementing specific traits
4. Different traits have different performance implications (Copy vs Clone)
5. Trait objects use dynamic dispatch for runtime polymorphism
6. Where clauses make complex bounds more readable
7. Traits are central to Rust's expressiveness

## Practice Exercise Ideas

1. Create a custom struct and derive common traits
2. Implement Display for a custom type
3. Write a generic function with trait bounds
4. Create a trait and implement for multiple types
5. Use trait objects for polymorphism
6. Work with Iterator trait implementations

---

**Time to complete this concept**: 2-2.5 hours
**Difficulty**: Intermediate
**Prerequisite**: Structs, Generics, Functions
**Next concept**: Error Handling Basics

For working examples, see the `examples/` folder.
For key takeaways, see `key_takeaways.md`.
