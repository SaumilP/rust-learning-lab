# Facade Pattern

## Overview

The Facade pattern provides a unified, simplified interface to a set of interfaces in a subsystem. It hides the complexity of interactions between multiple components and provides a single point of access for common operations.

## Problem It Solves

- Simplifying complex subsystems with many components
- Reducing dependencies on internal subsystem details
- Providing a clean API to hide implementation complexity
- Coordinating interactions between multiple objects
- Making large systems easier to use

## Implementation in Rust

### Basic Facade

```rust
// Complex subsystem components
pub struct FileSystem;

impl FileSystem {
    pub fn create_file(&self, path: &str) -> Result<(), String> {
        println!("Creating file: {}", path);
        Ok(())
    }

    pub fn write_data(&self, path: &str, data: &[u8]) -> Result<(), String> {
        println!("Writing {} bytes to {}", data.len(), path);
        Ok(())
    }

    pub fn close_file(&self, path: &str) -> Result<(), String> {
        println!("Closing file: {}", path);
        Ok(())
    }
}

pub struct Encryption;

impl Encryption {
    pub fn encrypt(&self, data: &[u8], key: &str) -> Vec<u8> {
        println!("Encrypting {} bytes with key: {}", data.len(), key);
        // Simplified encryption
        data.to_vec()
    }
}

pub struct Compression;

impl Compression {
    pub fn compress(&self, data: &[u8]) -> Vec<u8> {
        println!("Compressing {} bytes", data.len());
        // Simplified compression
        vec![]
    }
}

// Facade - simplified interface
pub struct SecureFileWriter {
    file_system: FileSystem,
    encryption: Encryption,
    compression: Compression,
}

impl SecureFileWriter {
    pub fn new() -> Self {
        Self {
            file_system: FileSystem,
            encryption: Encryption,
            compression: Compression,
        }
    }

    // Single method that coordinates multiple subsystems
    pub fn write_secure_file(
        &self,
        path: &str,
        data: &[u8],
        encryption_key: &str,
    ) -> Result<(), String> {
        self.file_system.create_file(path)?;

        let compressed = self.compression.compress(data);
        let encrypted = self.encryption.encrypt(&compressed, encryption_key);

        self.file_system.write_data(path, &encrypted)?;
        self.file_system.close_file(path)?;

        Ok(())
    }
}

// Usage - client just calls one method
fn main() {
    let writer = SecureFileWriter::new();
    writer
        .write_secure_file("data.bin", b"Hello, World!", "secret-key")
        .unwrap();
}
```

### Database Facade

```rust
// Subsystem components
pub struct ConnectionPool {
    connections: Vec<Connection>,
}

pub struct Connection {
    id: u32,
}

pub struct QueryExecutor;

pub struct ResultMapper;

impl QueryExecutor {
    pub fn execute(&self, query: &str) -> Vec<String> {
        println!("Executing: {}", query);
        vec!["row1".to_string(), "row2".to_string()]
    }
}

impl ResultMapper {
    pub fn map_to_structs(&self, results: Vec<String>) -> Vec<User> {
        results
            .iter()
            .map(|r| User {
                id: 1,
                name: r.clone(),
            })
            .collect()
    }
}

#[derive(Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
}

// Facade
pub struct Database {
    pool: ConnectionPool,
    executor: QueryExecutor,
    mapper: ResultMapper,
}

impl Database {
    pub fn new(pool_size: usize) -> Self {
        let connections = (0..pool_size)
            .map(|i| Connection { id: i as u32 })
            .collect();

        Self {
            pool: ConnectionPool { connections },
            executor: QueryExecutor,
            mapper: ResultMapper,
        }
    }

    pub fn find_users(&self, query: &str) -> Result<Vec<User>, String> {
        // Get connection from pool
        let _connection = self.pool.connections.first();

        // Execute query
        let results = self.executor.execute(query);

        // Map results
        let users = self.mapper.map_to_structs(results);

        Ok(users)
    }

    pub fn create_user(&self, user: &User) -> Result<u32, String> {
        let query = format!("INSERT INTO users (name) VALUES ('{}')", user.name);
        let _results = self.executor.execute(&query);
        Ok(user.id)
    }
}

// Usage
fn main() {
    let db = Database::new(5);
    let users = db.find_users("SELECT * FROM users").unwrap();
    println!("Found {} users", users.len());
}
```

### Payment Processing Facade

```rust
pub struct PaymentGateway;

impl PaymentGateway {
    pub fn authorize(&self, amount: f64) -> bool {
        println!("Authorizing payment: ${}", amount);
        true
    }

    pub fn process(&self, amount: f64) -> String {
        println!("Processing payment: ${}", amount);
        "TXN123".to_string()
    }
}

pub struct FraudDetection;

impl FraudDetection {
    pub fn check(&self, amount: f64) -> bool {
        println!("Checking for fraud: ${}", amount);
        true
    }
}

pub struct Logging;

impl Logging {
    pub fn log_transaction(&self, txn_id: &str, amount: f64) {
        println!("Logging transaction {} for ${}", txn_id, amount);
    }
}

// Facade - hides complexity of payment processing
pub struct PaymentProcessor {
    gateway: PaymentGateway,
    fraud_check: FraudDetection,
    logging: Logging,
}

impl PaymentProcessor {
    pub fn new() -> Self {
        Self {
            gateway: PaymentGateway,
            fraud_check: FraudDetection,
            logging: Logging,
        }
    }

    pub fn process_payment(&self, amount: f64) -> Result<String, String> {
        // Check fraud
        if !self.fraud_check.check(amount) {
            return Err("Fraud detected".to_string());
        }

        // Authorize
        if !self.gateway.authorize(amount) {
            return Err("Authorization failed".to_string());
        }

        // Process
        let txn_id = self.gateway.process(amount);

        // Log
        self.logging.log_transaction(&txn_id, amount);

        Ok(txn_id)
    }
}

// Usage
fn main() {
    let processor = PaymentProcessor::new();
    match processor.process_payment(99.99) {
        Ok(txn_id) => println!("Payment successful: {}", txn_id),
        Err(e) => println!("Payment failed: {}", e),
    }
}
```

## Benefits

1. **Simplicity** - Clients see a simple interface, not complex subsystems
2. **Decoupling** - Clients depend on facade, not implementation details
3. **Maintainability** - Changes to subsystem don't affect client code
4. **Coordination** - Facade coordinates multiple components
5. **Clear Boundaries** - Defines clear separation between subsystem and client

## Common Patterns

### Pattern 1: Simple Facade Method
```rust
pub impl Facade {
    pub fn simple_operation(&self) -> Result<String, Error> {
        self.subsystem_a.complex_method()?;
        self.subsystem_b.another_method()?;
        Ok("Done".to_string())
    }
}
```

### Pattern 2: Configuration Facade
```rust
pub struct ConfigFacade {
    config: Config,
}

impl ConfigFacade {
    pub fn setup_environment(&self, env: &str) -> Result<(), String> {
        self.setup_database(env)?;
        self.setup_cache(env)?;
        self.setup_logging(env)?;
        Ok(())
    }
}
```

### Pattern 3: Layered Facade
```rust
pub struct HighLevelFacade {
    low_level: LowLevelFacade,
}

impl HighLevelFacade {
    pub fn business_operation(&self) -> Result<Output, Error> {
        let data = self.low_level.get_data()?;
        self.low_level.process(data)
    }
}
```

### Pattern 4: Generic Facade
```rust
pub struct GenericFacade<T: Subsystem> {
    subsystem: T,
}

impl<T: Subsystem> GenericFacade<T> {
    pub fn unified_operation(&self) -> Result<Output, Error> {
        self.subsystem.complex_operation()
    }
}
```

### Pattern 5: Builder-Style Facade
```rust
pub struct ConfigBuilder {
    steps: Vec<ConfigStep>,
}

impl ConfigBuilder {
    pub fn setup() -> Self {
        Self { steps: vec![] }
    }

    pub fn with_database(mut self, url: &str) -> Self {
        self.steps.push(ConfigStep::Database(url.to_string()));
        self
    }

    pub fn with_cache(mut self) -> Self {
        self.steps.push(ConfigStep::Cache);
        self
    }

    pub fn build(self) -> Result<Config, String> {
        // Execute all steps
        Ok(Config::new())
    }
}
```

## Real-World Examples

### Web Framework Facade
```rust
pub struct WebApplication {
    router: Router,
    middleware_chain: MiddlewareChain,
    db: Database,
}

impl WebApplication {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            middleware_chain: MiddlewareChain::new(),
            db: Database::new(),
        }
    }

    pub fn handle_request(&self, request: &Request) -> Response {
        let request = self.middleware_chain.process(request);
        let handler = self.router.find_handler(&request);
        handler.handle(&request, &self.db)
    }
}
```

### System Configuration Facade
```rust
pub struct SystemSetup {
    file_system: FileSystemSetup,
    network: NetworkSetup,
    security: SecuritySetup,
}

impl SystemSetup {
    pub fn initialize(&self) -> Result<(), String> {
        self.file_system.setup()?;
        self.network.setup()?;
        self.security.setup()?;
        println!("System initialized");
        Ok(())
    }
}
```

### Data Pipeline Facade
```rust
pub struct DataPipeline {
    reader: DataReader,
    processor: DataProcessor,
    writer: DataWriter,
}

impl DataPipeline {
    pub fn execute(&self, input_file: &str, output_file: &str) -> Result<(), String> {
        let data = self.reader.read(input_file)?;
        let processed = self.processor.process(data)?;
        self.writer.write(output_file, processed)?;
        Ok(())
    }
}
```

## Anti-Patterns to Avoid

- **God Facade** - Facade doing too much, not a single responsibility
- **Inadequate Facade** - Not hiding enough complexity
- **Leaky Abstraction** - Facade exposing subsystem implementation details
- **Facade Inflation** - Too many methods on facade

## Key Takeaways

- ✓ Facade should provide simple interface to complex subsystem
- ✓ Each facade method represents a high-level operation
- ✓ Hide all subsystem components from clients
- ✓ Coordinate multiple components for complex workflows
- ✓ Keep facade methods cohesive and focused
- ✓ Don't make facade a catch-all for random functionality
- ✓ Consider multiple facades for different client groups

