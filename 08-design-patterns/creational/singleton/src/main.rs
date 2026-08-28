// Singleton Pattern in Rust
// Ensures a class has only one instance and provides global access to it

use once_cell::sync::Lazy;
use std::sync::{Mutex, RwLock};

fn main() {
    println!("=== Singleton Pattern in Rust ===\n");

    // APPROACH 1: Lazy static (thread-safe, immutable)
    println!("1. LAZY STATIC (Thread-Safe, Immutable):\n");

    println!("   Config: {}", CONFIG.app_name);
    println!("   Version: {}", CONFIG.version);
    println!("   Maximum connections: {}", CONFIG.max_connections);
    println!("   ✓ Initialized once, thread-safe, immutable\n");

    // APPROACH 2: Lazy with Mutex (thread-safe, mutable)
    println!("2. LAZY WITH MUTEX (Thread-Safe, Mutable):\n");

    COUNTER.increment();
    COUNTER.increment();
    println!("   Counter: {}", COUNTER.get());

    COUNTER.increment();
    println!("   Counter: {}", COUNTER.get());

    println!("   ✓ Thread-safe mutable singleton\n");

    // APPROACH 3: Lazy with RwLock (multiple readers, single writer)
    println!("3. LAZY WITH RWLOCK (Multiple Readers):\n");

    CACHE.insert("user:1".to_string(), "Alice".to_string());
    CACHE.insert("user:2".to_string(), "Bob".to_string());

    if let Some(user) = CACHE.get("user:1") {
        println!("   Found: {}", user);
    }

    println!("   ✓ Efficient for read-heavy workloads\n");

    // APPROACH 4: Database connection pool (practical)
    println!("4. DATABASE CONNECTION POOL:\n");

    DB_POOL.execute("SELECT * FROM users");
    DB_POOL.execute("INSERT INTO logs VALUES (1, 'event')");

    let stats = DB_POOL.get_stats();
    println!("   {}", stats);

    println!("   ✓ Practical singleton for shared resources\n");

    // APPROACH 5: Logger (global state)
    println!("5. GLOBAL LOGGER:\n");

    LOGGER.log("Application started");
    LOGGER.log("Processing request");
    LOGGER.log("Request completed");

    println!("   ✓ Singleton pattern for logging\n");

    // APPROACH 6: Thread-safe demonstration
    println!("6. THREAD-SAFE DEMONSTRATION:\n");

    use std::thread;

    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                COUNTER.increment();
                println!("   Thread {} incremented counter", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("   Final counter: {}", COUNTER.get());
    println!("   ✓ Thread-safe across multiple threads\n");

    // APPROACH 7: Registry pattern (alternative to singleton)
    println!("7. REGISTRY PATTERN (Alternative):\n");

    let registry = ServiceRegistry::global();
    registry.register("auth", "AuthService v1.0");
    registry.register("db", "PostgreSQL");

    if let Some(service) = registry.get("auth") {
        println!("   Auth Service: {}", service);
    }
    if let Some(service) = registry.get("db") {
        println!("   Database: {}", service);
    }

    println!("   ✓ Registry pattern for service discovery\n");
}

// ========== APPROACH 1: Lazy Static (Immutable) ==========

static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    app_name: "MyApp".to_string(),
    version: "1.0.0".to_string(),
    max_connections: 100,
});

struct Config {
    app_name: String,
    version: String,
    max_connections: u32,
}

// ========== APPROACH 2: Lazy with Mutex (Mutable) ==========

static COUNTER: Lazy<Counter> = Lazy::new(|| Counter {
    value: Mutex::new(0),
});

struct Counter {
    value: Mutex<i32>,
}

impl Counter {
    fn increment(&self) {
        let mut count = self.value.lock().unwrap();
        *count += 1;
    }

    fn get(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

// ========== APPROACH 3: Lazy with RwLock (Read-Heavy) ==========

use std::collections::HashMap;

static CACHE: Lazy<Cache> = Lazy::new(|| Cache {
    data: RwLock::new(HashMap::new()),
});

struct Cache {
    data: RwLock<HashMap<String, String>>,
}

impl Cache {
    fn insert(&self, key: String, value: String) {
        let mut cache = self.data.write().unwrap();
        cache.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<String> {
        let cache = self.data.read().unwrap();
        cache.get(key).cloned()
    }
}

// ========== APPROACH 4: Database Connection Pool ==========

static DB_POOL: Lazy<DatabasePool> = Lazy::new(|| DatabasePool {
    connections: Mutex::new(10),
    queries_executed: Mutex::new(0),
});

struct DatabasePool {
    connections: Mutex<u32>,
    queries_executed: Mutex<u64>,
}

impl DatabasePool {
    fn execute(&self, query: &str) {
        println!("   Executing: {}", query);
        let mut count = self.queries_executed.lock().unwrap();
        *count += 1;
    }

    fn get_stats(&self) -> String {
        let connections = *self.connections.lock().unwrap();
        let queries = *self.queries_executed.lock().unwrap();
        format!(
            "Connections: {}, Queries executed: {}",
            connections, queries
        )
    }
}

// ========== APPROACH 5: Logger ==========

static LOGGER: Lazy<Logger> = Lazy::new(|| Logger {
    logs: Mutex::new(Vec::new()),
});

struct Logger {
    logs: Mutex<Vec<String>>,
}

impl Logger {
    fn log(&self, message: &str) {
        let mut logs = self.logs.lock().unwrap();
        let entry = format!("[{}] {}", logs.len() + 1, message);
        logs.push(entry.clone());
        println!("   {}", entry);
    }
}

// ========== APPROACH 7: Registry Pattern ==========

static SERVICE_REGISTRY: Lazy<ServiceRegistry> = Lazy::new(|| ServiceRegistry {
    services: RwLock::new(HashMap::new()),
});

struct ServiceRegistry {
    services: RwLock<HashMap<String, String>>,
}

impl ServiceRegistry {
    fn global() -> &'static Self {
        &SERVICE_REGISTRY
    }

    fn register(&self, name: &str, description: &str) {
        let mut services = self.services.write().unwrap();
        services.insert(name.to_string(), description.to_string());
    }

    fn get(&self, name: &str) -> Option<String> {
        let services = self.services.read().unwrap();
        services.get(name).cloned()
    }
}

/*
Singleton Pattern Summary:
==========================

WHEN TO USE:
- Need exactly one instance
- Global access point required
- Shared resource (DB pool, config, cache)
- Thread-safe global state

RUST APPROACHES:

1. LAZY STATIC (Immutable):
   - Use: once_cell::sync::Lazy
   - Thread-safe
   - Zero-cost after first access
   - Best for configuration

2. LAZY + MUTEX (Mutable):
   - Use: Lazy<Mutex<T>>
   - Thread-safe mutable state
   - Blocks on contention
   - Best for counters, simple state

3. LAZY + RWLOCK (Read-Heavy):
   - Use: Lazy<RwLock<T>>
   - Multiple readers
   - Single writer
   - Best for caches

BENEFITS:
- Controlled access to single instance
- Lazy initialization
- Thread-safe in Rust
- No global mutable state issues

TRADEOFFS:
- Global state (harder to test)
- Hidden dependencies
- Mutex overhead for mutability
- Can make code less modular

ALTERNATIVES TO SINGLETON:
- Dependency injection
- Pass instances explicitly
- Use builder pattern
- Registry pattern

COMPARED TO OTHER LANGUAGES:
Java:    Double-checked locking
Python:  Module-level variables
Go:      sync.Once
Rust:    Lazy + Mutex/RwLock (safe!)

RUST ADVANTAGES:
- Compile-time thread safety
- No data races
- Explicit mutability
- Zero-cost abstractions

BEST PRACTICES:
- Prefer dependency injection
- Use singletons sparingly
- Make them immutable when possible
- Consider RwLock for read-heavy
- Test with mock instances

Run this:
    cargo run

Experiment:
    - Add more threads and see thread safety
    - Try removing Mutex/RwLock (won't compile!)
    - Create your own singleton
    - Compare Mutex vs RwLock performance
    - Test registry pattern for services

Common Use Cases:
- Configuration
- Database connection pools
- Caching
- Logging
- Thread pools
- Application state
*/
