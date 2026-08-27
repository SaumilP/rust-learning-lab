# Decorator Pattern

## Overview

The Decorator pattern allows behavior to be added to objects dynamically. Instead of using subclasses to extend functionality, decorators wrap objects and add responsibilities. This is particularly powerful in Rust due to traits and composition.

## Problem It Solves

- Adding features to objects without modifying their class
- Avoiding explosion of subclasses for every feature combination
- Dynamically adding and removing responsibilities at runtime
- Mixing and matching features flexibly
- Open/Closed Principle - open for extension, closed for modification

## Implementation in Rust

### Basic Decorator Pattern

```rust
// Core trait
pub trait Component {
    fn operation(&self) -> String;
}

// Concrete component
pub struct SimpleComponent;

impl Component for SimpleComponent {
    fn operation(&self) -> String {
        "Simple".to_string()
    }
}

// Base decorator - implements Component and wraps another Component
pub struct BaseDecorator {
    component: Box<dyn Component>,
}

impl BaseDecorator {
    pub fn new(component: Box<dyn Component>) -> Self {
        Self { component }
    }
}

impl Component for BaseDecorator {
    fn operation(&self) -> String {
        self.component.operation()
    }
}

// Concrete decorators
pub struct ConcreteDecoratorA {
    component: Box<dyn Component>,
}

impl ConcreteDecoratorA {
    pub fn new(component: Box<dyn Component>) -> Self {
        Self { component }
    }
}

impl Component for ConcreteDecoratorA {
    fn operation(&self) -> String {
        format!("DecoratorA({})", self.component.operation())
    }
}

pub struct ConcreteDecoratorB {
    component: Box<dyn Component>,
}

impl ConcreteDecoratorB {
    pub fn new(component: Box<dyn Component>) -> Self {
        Self { component }
    }
}

impl Component for ConcreteDecoratorB {
    fn operation(&self) -> String {
        format!("DecoratorB({})", self.component.operation())
    }
}

// Usage - stack decorators
fn main() {
    let simple = Box::new(SimpleComponent);
    let with_a = Box::new(ConcreteDecoratorA::new(simple));
    let with_ab = Box::new(ConcreteDecoratorB::new(with_a));

    println!("{}", with_ab.operation()); // DecoratorB(DecoratorA(Simple))
}
```

### Data Stream Decorator

```rust
pub trait DataStream {
    fn write(&mut self, data: &[u8]) -> usize;
    fn close(&mut self);
}

pub struct FileStream {
    filename: String,
}

impl DataStream for FileStream {
    fn write(&mut self, data: &[u8]) -> usize {
        println!("Writing {} bytes to {}", data.len(), self.filename);
        data.len()
    }

    fn close(&mut self) {
        println!("Closing file: {}", self.filename);
    }
}

// Decorator: Add compression
pub struct CompressionDecorator {
    stream: Box<dyn DataStream>,
}

impl CompressionDecorator {
    pub fn new(stream: Box<dyn DataStream>) -> Self {
        Self { stream }
    }
}

impl DataStream for CompressionDecorator {
    fn write(&mut self, data: &[u8]) -> usize {
        let compressed_size = (data.len() as f64 * 0.7) as usize;
        println!("Compressing {} bytes to {}", data.len(), compressed_size);
        self.stream.write(&vec![0; compressed_size])
    }

    fn close(&mut self) {
        self.stream.close();
    }
}

// Decorator: Add encryption
pub struct EncryptionDecorator {
    stream: Box<dyn DataStream>,
}

impl EncryptionDecorator {
    pub fn new(stream: Box<dyn DataStream>) -> Self {
        Self { stream }
    }
}

impl DataStream for EncryptionDecorator {
    fn write(&mut self, data: &[u8]) -> usize {
        println!("Encrypting {} bytes", data.len());
        self.stream.write(data)
    }

    fn close(&mut self) {
        self.stream.close();
    }
}

// Usage
fn main() {
    let file = Box::new(FileStream {
        filename: "data.txt".to_string(),
    });
    let compressed = Box::new(CompressionDecorator::new(file));
    let encrypted = Box::new(EncryptionDecorator::new(compressed));

    let mut stream: Box<dyn DataStream> = encrypted;
    stream.write(b"Hello, World!");
    stream.close();
}
```

### Generic Decorator

```rust
pub trait Service {
    fn execute(&self, request: &str) -> String;
}

pub struct BasicService;

impl Service for BasicService {
    fn execute(&self, request: &str) -> String {
        format!("Executing: {}", request)
    }
}

// Generic decorator that wraps any Service
pub struct GenericDecorator<S: Service> {
    service: S,
    name: String,
}

impl<S: Service> GenericDecorator<S> {
    pub fn new(service: S, name: &str) -> Self {
        Self {
            service,
            name: name.to_string(),
        }
    }
}

impl<S: Service> Service for GenericDecorator<S> {
    fn execute(&self, request: &str) -> String {
        println!("[{}] Processing request", self.name);
        let result = self.service.execute(request);
        println!("[{}] Completed", self.name);
        result
    }
}

// Usage
fn main() {
    let basic = BasicService;
    let with_logging = GenericDecorator::new(basic, "Logger");
    let with_retry = GenericDecorator::new(with_logging, "Retry Handler");

    println!("{}", with_retry.execute("GET /users"));
}
```

## Benefits

1. **Flexibility** - Add features without modifying original objects
2. **Single Responsibility** - Each decorator handles one concern
3. **Composability** - Stack multiple decorators for different combinations
4. **Runtime Configuration** - Decide which decorators to use at runtime
5. **Open/Closed Principle** - Open for extension, closed for modification

## Common Patterns

### Pattern 1: Simple Decorator Wrapper
```rust
pub struct Decorator<T: Trait> {
    inner: T,
}

impl<T: Trait> Trait for Decorator<T> {
    fn method(&self) -> String {
        format!("Decorated({})", self.inner.method())
    }
}
```

### Pattern 2: Trait Object Decorator
```rust
pub struct DynamicDecorator {
    inner: Box<dyn Trait>,
}

impl Trait for DynamicDecorator {
    fn method(&self) -> String {
        format!("Decorated({})", self.inner.method())
    }
}
```

### Pattern 3: Layered Decorators
```rust
impl Component for Decorator {
    fn operation(&self) -> String {
        let inner = self.inner.operation();
        self.add_functionality(inner)
    }
}
```

### Pattern 4: Stateful Decorator
```rust
pub struct StatefulDecorator<T: Trait> {
    inner: T,
    state: State,
    call_count: u32,
}

impl<T: Trait> Trait for StatefulDecorator<T> {
    fn method(&self) -> String {
        self.inner.method()
    }
}
```

### Pattern 5: Conditional Decorator
```rust
pub fn decorate_conditionally<T: Trait>(
    inner: T,
    should_decorate: bool,
) -> Box<dyn Trait> {
    if should_decorate {
        Box::new(Decorator::new(inner))
    } else {
        Box::new(inner)
    }
}
```

## Real-World Examples

### HTTP Request Decorator
```rust
pub trait HttpHandler {
    fn handle(&self, request: &str) -> String;
}

pub struct BasicHttpHandler;

impl HttpHandler for BasicHttpHandler {
    fn handle(&self, request: &str) -> String {
        format!("Response to: {}", request)
    }
}

pub struct AuthenticationDecorator {
    handler: Box<dyn HttpHandler>,
}

impl HttpHandler for AuthenticationDecorator {
    fn handle(&self, request: &str) -> String {
        if request.contains("Authorization") {
            self.handler.handle(request)
        } else {
            "401 Unauthorized".to_string()
        }
    }
}

pub struct LoggingDecorator {
    handler: Box<dyn HttpHandler>,
}

impl HttpHandler for LoggingDecorator {
    fn handle(&self, request: &str) -> String {
        println!("Incoming request: {}", request);
        let response = self.handler.handle(request);
        println!("Response sent: {}", response);
        response
    }
}
```

### Reader Decorator
```rust
use std::io::Read;

pub struct CompressedReader<R: Read> {
    inner: R,
}

impl<R: Read> Read for CompressedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut temp = vec![0; buf.len() * 2];
        self.inner.read(&mut temp)
    }
}

pub struct DecryptedReader<R: Read> {
    inner: R,
}

impl<R: Read> Read for DecryptedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}
```

### Timer Decorator
```rust
use std::time::Instant;

pub struct TimerDecorator<T: Callable> {
    inner: T,
}

impl<T: Callable> Callable for TimerDecorator<T> {
    fn call(&self, input: &str) -> String {
        let start = Instant::now();
        let result = self.inner.call(input);
        let duration = start.elapsed();
        println!("Execution time: {:?}", duration);
        result
    }
}
```

## Anti-Patterns to Avoid

- **Deep Nesting** - Too many decorator layers can make code hard to understand
- **Over-Decoration** - Using decorators when composition would be clearer
- **State Inconsistency** - Ensure decorators don't leave objects in invalid states
- **Performance Overhead** - Each decorator layer adds a function call

## Key Takeaways

- ✓ Use decorators to add responsibilities dynamically
- ✓ Each decorator should handle a single concern
- ✓ Compose decorators for flexible feature combinations
- ✓ Prefer trait objects for runtime polymorphism
- ✓ Consider generic decorators for type safety when possible
- ✓ Keep decorator layers reasonably shallow
- ✓ Document the order and interactions of decorators

