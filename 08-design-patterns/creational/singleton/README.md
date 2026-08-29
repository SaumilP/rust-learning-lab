# Singleton Pattern

> Design reference: [C4 component explanation and embedded PlantUML](DESIGN.md).

## Overview

The Singleton pattern ensures a class has only one instance and provides a global point of access to it. In Rust, this is typically implemented using lazy statics or `Once` for thread-safe, one-time initialization.

## Problem It Solves

- Ensuring only one instance of a resource-heavy object exists
- Providing global access to a shared resource (logger, configuration, connection pool)
- Coordinating actions across the entire application
- Preventing race conditions during initialization
- Managing expensive resources efficiently

## Implementation in Rust

### Using `once_cell` (Modern Approach)

```rust
use once_cell::sync::Lazy;

pub struct Logger {
    name: String,
}

pub static LOGGER: Lazy<Logger> = Lazy::new(|| {
    Logger {
        name: "GlobalLogger".to_string(),
    }
});

impl Logger {
    pub fn log(&self, message: &str) {
        println!("[{}] {}", self.name, message);
    }
}

// Usage - thread-safe, lazily initialized
fn main() {
    LOGGER.log("Application started");
    LOGGER.log("Processing data");
}
```

### Using `std::sync::Once` for Manual Control

```rust
use std::sync::Once;

pub struct Config {
    db_url: String,
    api_key: String,
}

static mut CONFIG: Option<Config> = None;
static INIT: Once = Once::new();

impl Config {
    pub fn get() -> &'static Config {
        unsafe {
            INIT.call_once(|| {
                CONFIG = Some(Config {
                    db_url: "postgres://localhost".to_string(),
                    api_key: "secret-key".to_string(),
                });
            });
            CONFIG.as_ref().unwrap()
        }
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

// Usage
fn main() {
    let config = Config::get();
    println!("Database: {}", config.db_url());
}
```

### Using `Arc<Mutex<T>>` for Mutable State

```rust
use std::sync::{Arc, Mutex};

pub struct Database {
    connection_count: u32,
}

impl Database {
    pub fn new() -> Self {
        Self {
            connection_count: 0,
        }
    }

    pub fn connect(&mut self) {
        self.connection_count += 1;
        println!("Connection established. Total: {}", self.connection_count);
    }
}

use once_cell::sync::Lazy;

pub static DB: Lazy<Arc<Mutex<Database>>> =
    Lazy::new(|| Arc::new(Mutex::new(Database::new())));

// Usage
fn main() {
    {
        let mut db = DB.lock().unwrap();
        db.connect();
        db.connect();
    }

    {
        let mut db = DB.lock().unwrap();
        db.connect(); // Same instance
    }
}
```

### Thread-Safe Singleton with Initialization Function

```rust
use std::sync::{Arc, Mutex, Once};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ApplicationState {
    initialized: bool,
    data: Vec<String>,
}

pub struct AppStateManager;

static mut APP_STATE: Option<Arc<Mutex<ApplicationState>>> = None;
static INIT: Once = Once::new();
static INIT_FLAG: AtomicBool = AtomicBool::new(false);

impl AppStateManager {
    pub fn initialize(initial_data: Vec<String>) -> Result<(), String> {
        if INIT_FLAG.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            return Err("Already initialized".to_string());
        }

        unsafe {
            INIT.call_once(|| {
                APP_STATE = Some(Arc::new(Mutex::new(ApplicationState {
                    initialized: true,
                    data: initial_data,
                })));
            });
        }
        Ok(())
    }

    pub fn get() -> Option<Arc<Mutex<ApplicationState>>> {
        unsafe { APP_STATE.clone() }
    }
}

// Usage
fn main() {
    AppStateManager::initialize(vec!["data1".to_string()]).unwrap();

    if let Some(state) = AppStateManager::get() {
        let state = state.lock().unwrap();
        println!("Initialized: {}", state.initialized);
    }
}
```

## Benefits

1. **Controlled Access** - Single global point of access
2. **Lazy Initialization** - Only initialized when first needed
3. **Thread Safety** - Modern implementations are thread-safe by default
4. **Resource Management** - Single instance for expensive resources
5. **Consistency** - Same state across application

## Common Patterns

### Pattern 1: Simple Static Singleton
```rust
use once_cell::sync::Lazy;

pub static INSTANCE: Lazy<MyType> = Lazy::new(|| MyType::new());
```

### Pattern 2: Singleton with Access Methods
```rust
pub struct Singleton {
    value: String,
}

impl Singleton {
    pub fn get() -> &'static Singleton {
        use once_cell::sync::Lazy;
        static INSTANCE: Lazy<Singleton> = Lazy::new(|| {
            Singleton {
                value: "initialized".to_string(),
            }
        });
        &INSTANCE
    }
}
```

### Pattern 3: Mutable Singleton
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static COUNTER: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));

pub fn increment() {
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1;
}
```

### Pattern 4: Singleton with Initialization Parameters
```rust
use std::sync::Once;

pub struct Config {
    environment: String,
}

static mut CONFIG: Option<Config> = None;
static INIT: Once = Once::new();

pub fn init_config(env: &str) {
    unsafe {
        INIT.call_once(|| {
            CONFIG = Some(Config {
                environment: env.to_string(),
            });
        });
    }
}

pub fn get_config() -> &'static Config {
    unsafe { CONFIG.as_ref().expect("Config not initialized") }
}
```

### Pattern 5: Singleton Factory
```rust
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct ServiceRegistry {
    services: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

pub static REGISTRY: Lazy<Mutex<ServiceRegistry>> = Lazy::new(|| {
    Mutex::new(ServiceRegistry {
        services: HashMap::new(),
    })
});

pub fn register_service(name: String, service: Box<dyn std::any::Any + Send + Sync>) {
    let mut registry = REGISTRY.lock().unwrap();
    registry.services.insert(name, service);
}
```

## Real-World Examples

### Global Logger Singleton
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct Logger {
    level: String,
}

pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
    Mutex::new(Logger {
        level: "INFO".to_string(),
    })
});

pub fn log(message: &str) {
    let logger = LOGGER.lock().unwrap();
    println!("[{}] {}", logger.level, message);
}
```

### Database Connection Pool Singleton
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub struct ConnectionPool {
    connections: Vec<DbConnection>,
    max_size: usize,
}

pub static DB_POOL: Lazy<Mutex<ConnectionPool>> = Lazy::new(|| {
    Mutex::new(ConnectionPool {
        connections: vec![],
        max_size: 10,
    })
});

pub fn get_connection() -> Result<DbConnection, String> {
    let mut pool = DB_POOL.lock().unwrap();
    pool.connections.pop().ok_or("No connections available".to_string())
}
```

### Application Configuration Singleton
```rust
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub version: String,
    pub debug_mode: bool,
}

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    // Load from environment or config file
    AppConfig {
        app_name: "MyApp".to_string(),
        version: "1.0.0".to_string(),
        debug_mode: cfg!(debug_assertions),
    }
});
```

### Metrics/Telemetry Singleton
```rust
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;

pub struct Metrics {
    counters: HashMap<String, u64>,
    timings: HashMap<String, Vec<u64>>,
}

pub static METRICS: Lazy<Mutex<Metrics>> = Lazy::new(|| {
    Mutex::new(Metrics {
        counters: HashMap::new(),
        timings: HashMap::new(),
    })
});

pub fn increment_counter(name: &str) {
    let mut metrics = METRICS.lock().unwrap();
    *metrics.counters.entry(name.to_string()).or_insert(0) += 1;
}
```

## Anti-Patterns to Avoid

- **Over-using Singletons** - Can make testing difficult and hide dependencies
- **Mutable Global State** - Prefer immutable singletons when possible
- **Not Thread-Safe** - Always use proven libraries like `once_cell`
- **Hidden Dependencies** - Make singleton usage explicit in code
- **Initialization Order Issues** - Be careful with multiple interdependent singletons

## Key Takeaways

- ✓ Use `once_cell::Lazy` for modern, safe singleton implementation
- ✓ Prefer immutable singletons for better thread safety
- ✓ Use `Arc<Mutex<T>>` when mutable state is necessary
- ✓ Avoid singletons for testing - use dependency injection instead
- ✓ Document singleton usage clearly
- ✓ Consider if the problem really needs a singleton (often doesn't)
- ✓ Thread-safe initialization is built-in with proper tools
