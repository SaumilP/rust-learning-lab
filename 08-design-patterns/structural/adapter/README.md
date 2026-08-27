# Adapter Pattern

## Overview

The Adapter pattern converts the interface of a class into another interface clients expect. It lets classes work together that couldn't otherwise because of incompatible interfaces. This is particularly useful when integrating legacy code or third-party libraries.

## Problem It Solves

- Integrating incompatible interfaces into a system
- Wrapping legacy code to work with new interfaces
- Supporting multiple versions of an interface
- Reducing code duplication when working with similar but incompatible types
- Bridging the gap between different API designs

## Implementation in Rust

### Class Adapter (Composition)

```rust
// Old interface that we need to adapt
pub trait OldDataSource {
    fn fetch_data(&self) -> Vec<String>;
}

pub struct LegacyDatabase {
    data: Vec<String>,
}

impl OldDataSource for LegacyDatabase {
    fn fetch_data(&self) -> Vec<String> {
        self.data.clone()
    }
}

// New interface we want to support
pub trait DataProvider {
    fn get_records(&self) -> Vec<Record>;
}

pub struct Record {
    pub id: u32,
    pub content: String,
}

// Adapter that makes OldDataSource work as DataProvider
pub struct DatabaseAdapter {
    legacy_db: LegacyDatabase,
}

impl DatabaseAdapter {
    pub fn new(db: LegacyDatabase) -> Self {
        Self { legacy_db: db }
    }
}

impl DataProvider for DatabaseAdapter {
    fn get_records(&self) -> Vec<Record> {
        self.legacy_db
            .fetch_data()
            .iter()
            .enumerate()
            .map(|(i, content)| Record {
                id: i as u32,
                content: content.clone(),
            })
            .collect()
    }
}

// Usage
fn main() {
    let legacy_db = LegacyDatabase {
        data: vec!["record1".to_string(), "record2".to_string()],
    };

    let adapter = DatabaseAdapter::new(legacy_db);
    let records = adapter.get_records();
    println!("Records: {:?}", records);
}
```

### Type-Level Adapter with Generics

```rust
pub trait Logger {
    fn log(&self, msg: &str);
}

pub struct ConsoleLogger;

impl Logger for ConsoleLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

// Third-party interface we need to adapt
pub struct ExternalLogger {
    prefix: String,
}

impl ExternalLogger {
    pub fn write(&self, message: &str) {
        println!("{}: {}", self.prefix, message);
    }
}

// Generic adapter
pub struct LoggerAdapter<T> {
    inner: T,
}

impl<T> LoggerAdapter<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl Logger for LoggerAdapter<ExternalLogger> {
    fn log(&self, msg: &str) {
        self.inner.write(msg);
    }
}

// Usage
fn log_with_adapter(logger: &dyn Logger) {
    logger.log("Hello from adapter");
}

fn main() {
    let external = ExternalLogger {
        prefix: "[APP]".to_string(),
    };
    let adapted = LoggerAdapter::new(external);
    log_with_adapter(&adapted);
}
```

### Trait Object Adapter

```rust
// New trait-based interface
pub trait PaymentProcessor {
    fn process_payment(&self, amount: f64) -> Result<String, String>;
}

// Old third-party API
pub struct LegacyPaymentGateway;

impl LegacyPaymentGateway {
    pub fn charge_card(amount_cents: i32) -> bool {
        // Simulated charge
        amount_cents > 0
    }
}

// Adapter
pub struct PaymentGatewayAdapter;

impl PaymentProcessor for PaymentGatewayAdapter {
    fn process_payment(&self, amount: f64) -> Result<String, String> {
        let amount_cents = (amount * 100.0) as i32;
        if LegacyPaymentGateway::charge_card(amount_cents) {
            Ok(format!("Payment of ${} processed", amount))
        } else {
            Err("Payment failed".to_string())
        }
    }
}

// Usage
fn checkout(processor: &dyn PaymentProcessor, amount: f64) -> Result<(), String> {
    processor.process_payment(amount)?;
    println!("Checkout complete");
    Ok(())
}

fn main() {
    let adapter = PaymentGatewayAdapter;
    checkout(&adapter, 99.99).unwrap();
}
```

## Benefits

1. **Compatibility** - Makes incompatible interfaces work together
2. **Non-Intrusive** - Doesn't modify existing code
3. **Single Responsibility** - Adapter handles only interface conversion
4. **Reusability** - Adapters can be reused for multiple implementations
5. **Flexibility** - Easy to swap implementations or add new adapters

## Common Patterns

### Pattern 1: Wrapper Adapter
```rust
pub struct Adapter<T> {
    inner: T,
}

impl<T: OldInterface> NewInterface for Adapter<T> {
    fn new_method(&self) -> String {
        self.inner.old_method().to_uppercase()
    }
}
```

### Pattern 2: Extension Trait Adapter
```rust
pub trait JsonStringExt: ToString {
    fn to_json_string(&self) -> String {
        format!("\"{}\"", self.to_string())
    }
}

impl<T: ToString> JsonStringExt for T {}
```

### Pattern 3: Function Adapter
```rust
pub fn adapt_function<F: Fn(i32) -> i32>(f: F) -> impl Fn(String) -> String {
    move |s: String| {
        let num = s.parse::<i32>().unwrap_or(0);
        f(num).to_string()
    }
}
```

### Pattern 4: Converter Adapter
```rust
pub struct TypeAdapter<T, U> {
    value: T,
    converter: fn(T) -> U,
}

impl<T, U> TypeAdapter<T, U> {
    pub fn convert(self) -> U {
        (self.converter)(self.value)
    }
}
```

### Pattern 5: Protocol Adapter
```rust
pub struct ProtocolAdapter {
    inner: Box<dyn OldProtocol>,
}

impl ProtocolAdapter {
    pub fn adapt_to_new_protocol(&self) -> Box<dyn NewProtocol> {
        // Convert old protocol calls to new protocol
        Box::new(AdaptedProtocol {
            data: self.inner.get_data(),
        })
    }
}
```

## Real-World Examples

### JSON to Database Row Adapter
```rust
use serde_json::Value;
use std::collections::HashMap;

pub trait DatabaseRow {
    fn get_column(&self, name: &str) -> Option<String>;
}

pub struct JsonRowAdapter {
    data: Value,
}

impl JsonRowAdapter {
    pub fn new(json: Value) -> Self {
        Self { data: json }
    }
}

impl DatabaseRow for JsonRowAdapter {
    fn get_column(&self, name: &str) -> Option<String> {
        self.data
            .get(name)
            .map(|v| v.to_string())
    }
}
```

### File System Adapter
```rust
use std::path::Path;

pub trait FileStore {
    fn read(&self, path: &str) -> Result<String, String>;
    fn write(&self, path: &str, content: &str) -> Result<(), String>;
}

pub struct OsFileAdapter;

impl FileStore for OsFileAdapter {
    fn read(&self, path: &str) -> Result<String, String> {
        std::fs::read_to_string(path)
            .map_err(|e| e.to_string())
    }

    fn write(&self, path: &str, content: &str) -> Result<(), String> {
        std::fs::write(path, content)
            .map_err(|e| e.to_string())
    }
}
```

### Iterator Adapter
```rust
pub struct ReverseIteratorAdapter<I: Iterator> {
    items: Vec<I::Item>,
    index: usize,
}

impl<I: Iterator> Iterator for ReverseIteratorAdapter<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 0 {
            self.index -= 1;
            self.items.get(self.index).cloned()
        } else {
            None
        }
    }
}
```

## Key Takeaways

- ✓ Use adapters to make incompatible interfaces work together
- ✓ Keep adapters focused on interface conversion
- ✓ Prefer composition over inheritance in Rust
- ✓ Use trait objects for runtime polymorphism
- ✓ Consider extension traits for adapting built-in types
- ✓ Adapters should be transparent to clients
- ✓ Document clearly how old and new interfaces map

