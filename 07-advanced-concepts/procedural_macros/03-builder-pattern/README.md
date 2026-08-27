# Builder Pattern Derive Macro

Automatically generate builder pattern implementation for structs.

## Features

- Automatic builder struct generation
- Required and optional fields
- Type-safe builder methods
- Compile-time field validation
- Fluent API

## Project Structure

```
03-builder-pattern/
├── my-builder/     # The proc-macro crate
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
└── example/        # Usage examples
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Running

```bash
cd example
cargo run
```

## Usage

### Basic Builder

```rust
#[derive(Builder)]
struct User {
    id: u32,
    name: String,
    email: String,
}

let user = User::builder()
    .id(1)
    .name("Alice".to_string())
    .email("alice@example.com".to_string())
    .build()
    .unwrap();
```

### Optional Fields

```rust
#[derive(Builder)]
struct Config {
    host: String,
    #[builder(optional)]
    port: Option<u16>,
    #[builder(optional)]
    timeout: Option<u64>,
}

let config = Config::builder()
    .host("localhost".to_string())
    .port(8080)
    .build()
    .unwrap();
```

## What You'll Learn

- Complex derive macro implementation
- Attribute parsing with helper attributes
- Type analysis (detecting Option<T>)
- Builder pattern generation
- Error reporting
- Generic code generation
