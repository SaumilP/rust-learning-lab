# Factory Pattern

> Design reference: [C4 component explanation and embedded PlantUML](DESIGN.md).

## Overview

The Factory pattern provides an interface for creating objects without specifying their exact classes. Instead of using `new` directly, you call a factory method that handles object creation. This decouples client code from concrete implementations.

## Problem It Solves

- Creating objects of different types based on configuration or parameters
- Hiding complex initialization logic
- Making it easy to add new object types without changing client code
- Supporting multiple implementations of the same interface
- Centralizing object creation rules

## Implementation in Rust

### Simple Factory Function

```rust
pub trait Shape {
    fn area(&self) -> f64;
    fn description(&self) -> String;
}

pub struct Circle {
    radius: f64,
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn description(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn description(&self) -> String {
        format!("Rectangle {}x{}", self.width, self.height)
    }
}

// Factory function
pub fn create_shape(shape_type: &str, params: &[f64]) -> Option<Box<dyn Shape>> {
    match shape_type {
        "circle" if params.len() >= 1 => {
            Some(Box::new(Circle { radius: params[0] }))
        }
        "rectangle" if params.len() >= 2 => {
            Some(Box::new(Rectangle {
                width: params[0],
                height: params[1],
            }))
        }
        _ => None,
    }
}

// Usage
let circle = create_shape("circle", &[5.0]).unwrap();
println!("Area: {}", circle.area());

let rectangle = create_shape("rectangle", &[4.0, 6.0]).unwrap();
println!("Description: {}", rectangle.description());
```

### Factory Methods (Associated Functions)

```rust
pub struct DatabaseConnection {
    host: String,
    port: u16,
}

impl DatabaseConnection {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }

    // Factory method for development
    pub fn dev() -> Self {
        Self::new("localhost", 5432)
    }

    // Factory method for production
    pub fn prod() -> Self {
        Self::new("prod-db.example.com", 5432)
    }

    // Factory method with connection string parsing
    pub fn from_connection_string(conn_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = conn_str.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid connection string".to_string());
        }
        let port = parts[1]
            .parse::<u16>()
            .map_err(|_| "Invalid port")?;
        Ok(Self::new(parts[0], port))
    }
}

// Usage
let dev_db = DatabaseConnection::dev();
let prod_db = DatabaseConnection::prod();
let custom_db = DatabaseConnection::from_connection_string("db.example.com:3306")?;
```

### Factory with Enums

```rust
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub trait Logger {
    fn log(&self, message: &str);
}

pub struct ConsoleLogger;
pub struct FileLogger;

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("CONSOLE: {}", message);
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        println!("FILE: {}", message); // Would write to file in real implementation
    }
}

pub fn create_logger(log_type: LogLevel) -> Box<dyn Logger> {
    match log_type {
        LogLevel::Debug | LogLevel::Info => Box::new(ConsoleLogger),
        LogLevel::Warn | LogLevel::Error => Box::new(FileLogger),
    }
}

// Usage
let logger = create_logger(LogLevel::Debug);
logger.log("Application started");
```

## Benefits

1. **Decoupling** - Client code doesn't depend on concrete classes
2. **Flexibility** - Easy to add new types without changing existing code
3. **Centralization** - All creation logic in one place
4. **Testability** - Can inject mock factories for testing
5. **Configuration-driven** - Create objects based on config files or runtime conditions

## Common Patterns

### Pattern 1: String-based Factory
```rust
pub fn create_handler(handler_type: &str) -> Option<Box<dyn Handler>> {
    match handler_type {
        "json" => Some(Box::new(JsonHandler)),
        "xml" => Some(Box::new(XmlHandler)),
        "csv" => Some(Box::new(CsvHandler)),
        _ => None,
    }
}
```

### Pattern 2: Enum-based Factory
```rust
pub enum DataFormat {
    Json,
    Xml,
    Csv,
}

pub fn create_handler(format: DataFormat) -> Box<dyn Handler> {
    match format {
        DataFormat::Json => Box::new(JsonHandler),
        DataFormat::Xml => Box::new(XmlHandler),
        DataFormat::Csv => Box::new(CsvHandler),
    }
}
```

### Pattern 3: Configuration-based Factory
```rust
pub struct FactoryConfig {
    handler_type: String,
    options: HashMap<String, String>,
}

pub fn create_from_config(config: &FactoryConfig) -> Result<Box<dyn Handler>, String> {
    let handler = match config.handler_type.as_str() {
        "json" => Box::new(JsonHandler::with_options(&config.options)?),
        "xml" => Box::new(XmlHandler::with_options(&config.options)?),
        _ => return Err("Unknown handler type".to_string()),
    };
    Ok(handler)
}
```

### Pattern 4: Builder + Factory Combination
```rust
pub struct TransportFactory {
    protocol: String,
    encryption: bool,
}

impl TransportFactory {
    pub fn new(protocol: &str) -> Self {
        Self {
            protocol: protocol.to_string(),
            encryption: false,
        }
    }

    pub fn with_encryption(mut self) -> Self {
        self.encryption = true;
        self
    }

    pub fn build(self) -> Box<dyn Transport> {
        match self.protocol.as_str() {
            "http" => {
                if self.encryption {
                    Box::new(HttpsTransport::new())
                } else {
                    Box::new(HttpTransport::new())
                }
            }
            "tcp" => {
                if self.encryption {
                    Box::new(TcpSslTransport::new())
                } else {
                    Box::new(TcpTransport::new())
                }
            }
            _ => Box::new(HttpTransport::new()), // Default
        }
    }
}

// Usage
let https = TransportFactory::new("http").with_encryption().build();
```

### Pattern 5: Lazy Factory with Caching
```rust
use std::collections::HashMap;
use std::sync::Mutex;

pub struct CachedFactory {
    cache: Mutex<HashMap<String, Box<dyn Service>>>,
}

impl CachedFactory {
    pub fn new() -> Self {
        Self {
            cache: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_service(&self, service_type: &str) -> Box<dyn Service> {
        let mut cache = self.cache.lock().unwrap();

        if let Some(service) = cache.get(service_type) {
            return Box::new(service.clone()); // Clone if service is Clone
        }

        let service = self.create_service(service_type);
        cache.insert(service_type.to_string(), Box::new(service.clone()));
        Box::new(service)
    }

    fn create_service(&self, service_type: &str) -> impl Service + Clone {
        // Creation logic
        ServiceImpl::new()
    }
}
```

## Real-World Examples

### Database Connection Pool Factory
```rust
pub enum DatabaseType {
    PostgreSQL,
    MySQL,
    SQLite,
}

pub fn create_pool(db_type: DatabaseType, url: &str) -> Result<Box<dyn Pool>, String> {
    match db_type {
        DatabaseType::PostgreSQL => {
            // Create PostgreSQL connection pool
            Ok(Box::new(PostgresPool::new(url)?))
        }
        DatabaseType::MySQL => {
            Ok(Box::new(MysqlPool::new(url)?))
        }
        DatabaseType::SQLite => {
            Ok(Box::new(SqlitePool::new(url)?))
        }
    }
}
```

### HTTP Client Factory
```rust
pub struct HttpClientFactory {
    timeout_secs: u64,
    retries: u32,
}

impl HttpClientFactory {
    pub fn create_client(&self, client_type: &str) -> Box<dyn HttpClient> {
        match client_type {
            "standard" => Box::new(StandardClient::new(self.timeout_secs)),
            "resilient" => Box::new(ResilientClient::new(self.timeout_secs, self.retries)),
            "cached" => Box::new(CachedClient::new(self.timeout_secs)),
            _ => Box::new(StandardClient::new(self.timeout_secs)),
        }
    }
}
```

### Plugin System Factory
```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, input: &str) -> String;
}

pub struct PluginFactory {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginFactory {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, name: String, plugin: Box<dyn Plugin>) {
        self.plugins.insert(name, plugin);
    }

    pub fn get_plugin(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.get(name).map(|p| p.as_ref())
    }
}
```

## Key Takeaways

- ✓ Use factory functions when object creation depends on runtime data
- ✓ Prefer associated functions (::new, ::dev, ::prod) for simple cases
- ✓ Use trait objects (Box<dyn Trait>) to return different types
- ✓ Combine with builder pattern for complex construction
- ✓ Consider caching for expensive object creation
- ✓ Match enums for type-safe factory dispatch
- ✓ Hide implementation details behind factory functions
