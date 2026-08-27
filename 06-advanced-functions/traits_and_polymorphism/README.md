# Traits and Polymorphism

## Overview

Traits are Rust's way of defining shared behavior that different types can implement. They enable polymorphism - the ability to write generic code that works with multiple types. Traits are similar to interfaces in other languages but are more powerful and flexible. Understanding traits is essential for writing reusable, extensible code and for working with Rust's ecosystem.

## Theory

### What is a Trait?

A trait defines a collection of method signatures that a type can choose to implement. It describes "what a type can do" rather than "what a type is."

```
Trait = interface + default implementations + static methods
```

### Types of Polymorphism

1. **Compile-time (Monomorphic)** - Static Dispatch
   - Generic code is specialized at compile time
   - No runtime overhead
   - Larger binary size

2. **Runtime (Dynamic)** - Dynamic Dispatch
   - Trait objects enable runtime polymorphism
   - Small binary size
   - Runtime overhead

### Common Traits

**Standard Library Traits:**
- `Debug` - Can be printed with `{:?}`
- `Display` - Can be printed with `{}`
- `Clone` - Can be explicitly copied
- `Copy` - Automatically copied
- `Default` - Has a default value
- `Eq` / `PartialEq` - Can be compared
- `Ord` / `PartialOrd` - Can be ordered
- `Hash` - Can be used as map key
- `Iterator` - Can produce sequence of values

### Trait Objects

Trait objects (`dyn Trait`) enable dynamic dispatch:
- Erase type information at runtime
- Use vtable (virtual method table) for method lookup
- Trade compile-time specialization for runtime flexibility

## Syntax

### Defining a Trait

```rust
trait Animal {
    fn make_sound(&self) -> String;
    fn get_name(&self) -> &str;
}
```

### Implementing a Trait

```rust
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
```

### Trait Methods with Default Implementation

```rust
trait Shape {
    fn area(&self) -> f64;

    // Default implementation
    fn describe(&self) -> String {
        format!("Area: {}", self.area())
    }
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    // describe uses default implementation
}
```

### Trait Bounds on Generics

```rust
// Single trait bound
fn print_it<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

// Multiple trait bounds
fn compare_and_print<T: PartialOrd + std::fmt::Display>(t: T, u: T) {
    if t > u {
        println!("{} > {}", t, u);
    }
}

// Where clause syntax
fn process<T>(t: T)
where
    T: Clone + Default,
{
    let t2 = t.clone();
}
```

### Trait Objects (Dynamic Dispatch)

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing circle");
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing square");
    }
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];

    for shape in shapes {
        shape.draw();
    }
}
```

### Associated Types

```rust
trait Container {
    type Item;

    fn get(&self) -> &Self::Item;
}

struct Box<T> {
    value: T,
}

impl<T> Container for Box<T> {
    type Item = T;

    fn get(&self) -> &Self::Item {
        &self.value
    }
}
```

### Associated Constants

```rust
trait Configuration {
    const VERSION: &'static str;
    const MAX_SIZE: usize;
}

struct MyConfig;

impl Configuration for MyConfig {
    const VERSION: &'static str = "1.0";
    const MAX_SIZE: usize = 100;
}
```

## Common Patterns

### Pattern 1: Trait for Behavior Abstraction

```rust
trait FileOps {
    fn read(&self) -> String;
    fn write(&self, content: &str);
}

struct JsonFile {
    path: String,
}

struct CsvFile {
    path: String,
}

impl FileOps for JsonFile {
    fn read(&self) -> String {
        // JSON-specific reading
        "{}".to_string()
    }

    fn write(&self, content: &str) {
        // JSON-specific writing
    }
}

impl FileOps for CsvFile {
    fn read(&self) -> String {
        // CSV-specific reading
        "".to_string()
    }

    fn write(&self, content: &str) {
        // CSV-specific writing
    }
}

fn process_file<F: FileOps>(file: &F) {
    let content = file.read();
    println!("{}", content);
}
```

### Pattern 2: Trait Objects for Collections

```rust
trait Animal {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof".to_string()
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        "Meow".to_string()
    }
}

fn main() {
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
        Box::new(Dog),
    ];

    for animal in animals {
        println!("{}", animal.speak());
    }
}
```

### Pattern 3: Implementing Multiple Traits

```rust
trait Drawable {
    fn draw(&self);
}

trait Serializable {
    fn to_json(&self) -> String;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing rectangle");
    }
}

impl Serializable for Rectangle {
    fn to_json(&self) -> String {
        format!(r#"{{"width": {}, "height": {}}}"#, self.width, self.height)
    }
}

fn draw_and_serialize<T: Drawable + Serializable>(item: &T) {
    item.draw();
    println!("{}", item.to_json());
}
```

### Pattern 4: Trait Methods with Self

```rust
trait Cloneable {
    fn clone_box(&self) -> Box<dyn Cloneable>;
}

struct Point(i32, i32);

impl Cloneable for Point {
    fn clone_box(&self) -> Box<dyn Cloneable> {
        Box::new(*self)
    }
}
```

### Pattern 5: Generic Trait Bounds

```rust
struct Container<T> {
    items: Vec<T>,
}

impl<T: std::fmt::Display> Container<T> {
    fn print_all(&self) {
        for item in &self.items {
            println!("{}", item);
        }
    }
}

impl<T: Clone> Container<T> {
    fn duplicate(&self) -> Self {
        Container {
            items: self.items.clone(),
        }
    }
}
```

## Common Mistakes

### Mistake 1: Missing Trait Bound

```rust
// ❌ WRONG - Assumes T has println!
fn print_value<T>(value: T) {
    println!("{}", value);  // ERROR: T might not implement Display
}

// ✅ CORRECT - Add Display bound
fn print_value<T: std::fmt::Display>(value: T) {
    println!("{}", value);
}
```

### Mistake 2: Forgetting `&self` in Method

```rust
// ❌ WRONG - self type not specified
trait Action {
    fn perform();  // What self?
}

// ✅ CORRECT - Explicitly specify self
trait Action {
    fn perform(&self);
    fn perform_mut(&mut self);
    fn consume(self);
}
```

### Mistake 3: Using `dyn Trait` Without Box/Reference

```rust
// ❌ WRONG - dyn Trait is unsized
fn process(t: dyn Trait) {  // ERROR: dyn Trait is unsized
}

// ✅ CORRECT - Wrap in pointer
fn process(t: &dyn Trait) {}
fn process(t: Box<dyn Trait>) {}
```

### Mistake 4: Type Mismatch in Trait Objects

```rust
// ❌ WRONG - All must be same type
let mut v: Vec<Box<dyn Display>> = vec![];

// ✅ CORRECT - They are all dyn Display
let v: Vec<Box<dyn std::fmt::Display>> = vec![
    Box::new(5),
    Box::new("hello"),
];
```

### Mistake 5: Orphan Rule Violation

```rust
// ❌ WRONG - Can't implement external trait for external type
impl std::fmt::Display for String {  // ERROR: both external
}

// ✅ CORRECT - Implement your trait or for your type
trait MyDisplay {}
impl MyDisplay for String {}  // OK: MyDisplay is yours
```

## Real-World Examples

### Example 1: Logger Trait

```rust
use std::fmt::Display;

trait Logger {
    fn log(&self, message: &str);
    fn error(&self, error: &str) {
        self.log(&format!("ERROR: {}", error));
    }
}

struct ConsoleLogger;
struct FileLogger {
    path: String,
}

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("{}", message);
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        println!("Writing to {}: {}", self.path, message);
    }
}

fn run_app<L: Logger>(logger: &L) {
    logger.log("App started");
    logger.error("Something went wrong");
}

fn main() {
    run_app(&ConsoleLogger);
    run_app(&FileLogger {
        path: "app.log".to_string(),
    });
}
```

### Example 2: Iterator Pattern

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl std::iter::Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= self.max {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter = Counter { count: 0, max: 5 };
    for num in counter {
        println!("{}", num);
    }
}
```

### Example 3: Polymorphic Functions

```rust
trait HasName {
    fn get_name(&self) -> &str;
}

struct Person {
    name: String,
}

struct Company {
    name: String,
}

impl HasName for Person {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl HasName for Company {
    fn get_name(&self) -> &str {
        &self.name
    }
}

fn greet<T: HasName>(entity: &T) {
    println!("Hello, {}", entity.get_name());
}

fn greet_multiple(entities: Vec<Box<dyn HasName>>) {
    for entity in entities {
        println!("Hello, {}", entity.get_name());
    }
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
    };
    let company = Company {
        name: "ACME".to_string(),
    };

    greet(&person);
    greet(&company);

    let mixed: Vec<Box<dyn HasName>> = vec![
        Box::new(Person {
            name: "Bob".to_string(),
        }),
        Box::new(Company {
            name: "XYZ Inc".to_string(),
        }),
    ];

    greet_multiple(mixed);
}
```

### Example 4: Formatter Trait

```rust
trait Formatter {
    fn format(&self, text: &str) -> String;
}

struct UppercaseFormatter;
struct LowercaseFormatter;
struct TitleFormatter;

impl Formatter for UppercaseFormatter {
    fn format(&self, text: &str) -> String {
        text.to_uppercase()
    }
}

impl Formatter for LowercaseFormatter {
    fn format(&self, text: &str) -> String {
        text.to_lowercase()
    }
}

impl Formatter for TitleFormatter {
    fn format(&self, text: &str) -> String {
        text
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn apply_formatter(formatter: &dyn Formatter, text: &str) {
    println!("{}", formatter.format(text));
}

fn main() {
    let text = "hello world rust";
    apply_formatter(&UppercaseFormatter, text);
    apply_formatter(&LowercaseFormatter, text);
    apply_formatter(&TitleFormatter, text);
}
```

### Example 5: Plugin System

```rust
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

struct ReversePlugin;
struct UppercasePlugin;

impl Plugin for ReversePlugin {
    fn name(&self) -> &str {
        "Reverse"
    }

    fn execute(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}

impl Plugin for UppercasePlugin {
    fn name(&self) -> &str {
        "Uppercase"
    }

    fn execute(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    fn run_all(&self, input: &str) {
        for plugin in &self.plugins {
            println!("{}: {}", plugin.name(), plugin.execute(input));
        }
    }
}

fn main() {
    let mut manager = PluginManager::new();
    manager.add_plugin(Box::new(ReversePlugin));
    manager.add_plugin(Box::new(UppercasePlugin));
    manager.run_all("hello");
}
```

## Related Concepts

### Prerequisites
- Module 01: Structs, Implementations
- Module 02: Collections
- Module 05: Ownership and borrowing

### Follow-ups
- Generics (combining with traits)
- Advanced trait patterns (GAT, HRTB)
- Async/await (Stream trait)

## Best Practices

1. **Define Traits for Behaviors** - Not data structures
2. **Use Trait Bounds** - Make requirements explicit
3. **Prefer Static Dispatch** - Generic traits when possible
4. **Dynamic Dispatch Judiciously** - Use dyn Trait when necessary
5. **Document Trait Requirements** - What implementations must do
6. **Implement Common Traits** - Debug, Display, Clone
7. **Keep Traits Focused** - Small, cohesive interfaces
8. **Use Associated Types** - When possible instead of generics

## Summary

Traits are Rust's powerful abstraction mechanism that enable both static and dynamic polymorphism. They allow you to write flexible, reusable code that works with multiple types while maintaining type safety and performance. Understanding traits is key to writing idiomatic Rust and leveraging the ecosystem's design patterns.

## Practice Exercise Ideas

1. Create a payment processing trait with different payment methods
2. Build a database trait with multiple backend implementations
3. Implement a cache trait with memory and persistent backends
4. Create a notification system with email, SMS, and push providers
5. Build a serialization system with JSON, XML, YAML formatters

