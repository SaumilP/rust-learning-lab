# Builder Pattern

> Design reference: [C4 component explanation and embedded PlantUML](DESIGN.md).

## Overview

The Builder pattern separates the construction of complex objects from their representation. Instead of creating objects with many parameters all at once, builders allow step-by-step construction and optional parameters.

## Problem It Solves

- Objects with many configuration options
- Preventing invalid state combinations
- Making object construction readable and safe
- Optional vs required fields

## Implementation in Rust

```rust
#[derive(Clone)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
    timeout_secs: u64,
}

pub struct DatabaseConfigBuilder {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
    timeout_secs: u64,
}

impl DatabaseConfigBuilder {
    pub fn new(host: &str, username: &str, password: &str) -> Self {
        Self {
            host: host.to_string(),
            port: 5432,
            username: username.to_string(),
            password: password.to_string(),
            database: "default".to_string(),
            timeout_secs: 30,
        }
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn database(mut self, database: &str) -> Self {
        self.database = database.to_string();
        self
    }

    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn build(self) -> DatabaseConfig {
        DatabaseConfig {
            host: self.host,
            port: self.port,
            username: self.username,
            password: self.password,
            database: self.database,
            timeout_secs: self.timeout_secs,
        }
    }
}

// Usage
let config = DatabaseConfigBuilder::new("localhost", "admin", "secret")
    .port(5433)
    .database("myapp")
    .timeout_secs(60)
    .build();
```

## Benefits

1. **Readability** - Clear which field is which
2. **Flexibility** - Add new fields without breaking existing code
3. **Safety** - Required fields enforced
4. **Fluent Interface** - Chainable method calls

## Common Patterns

### Method chaining builder
```rust
pub fn host(mut self, host: &str) -> Self {
    self.host = host.to_string();
    self
}
```

### Typed builders for compile-time safety
Use phantom types to ensure required fields at compile time:
```rust
pub struct HasHost;
pub struct NoHost;

pub struct Builder<Host = NoHost> {
    // ...
    _host_marker: std::marker::PhantomData<Host>,
}
```

### Builder with validation
```rust
pub fn build(self) -> Result<DatabaseConfig, String> {
    if self.port == 0 {
        return Err("Port must be non-zero".to_string());
    }
    Ok(DatabaseConfig { /* ... */ })
}
```

## Real-World Example

HTTP request builder:
```rust
let response = http_client
    .post("https://api.example.com/users")
    .header("Authorization", "Bearer token")
    .json(&user_data)
    .timeout(Duration::from_secs(30))
    .send()?;
```

## Key Takeaways

- ✓ Separate construction from representation
- ✓ Support optional and required fields
- ✓ Use method chaining for fluent interface
- ✓ Consider validation in build() method
- ✓ Can combine with type state pattern for compile-time safety
