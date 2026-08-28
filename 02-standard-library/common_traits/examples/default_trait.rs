#![allow(dead_code)]

// Example: Default Trait Usage
//
// Demonstrates:
// - Using Default trait for default values
// - Deriving Default for custom types
// - Default with Option and collections
// - Struct update syntax with Default

fn main() {
    println!("=== Default for Primitive Types ===\n");

    // Primitive types have Default implementations
    let int_default: i32 = Default::default();
    let float_default: f64 = Default::default();
    let bool_default: bool = Default::default();
    let char_default: char = Default::default();

    println!("i32::default() = {}", int_default);
    println!("f64::default() = {}", float_default);
    println!("bool::default() = {}", bool_default);
    println!("char::default() = '{}' (null character)", char_default);

    // Alternative syntax
    let x = i32::default();
    let y = String::default();
    println!("\ni32::default() = {}", x);
    println!("String::default() = '{}'", y);

    println!("\n=== Default for Collections ===\n");

    // Collections default to empty
    let vec_default: Vec<i32> = Default::default();
    let string_default: String = Default::default();

    println!("Vec::<i32>::default() = {:?}", vec_default);
    println!("String::default() = '{}'", string_default);

    use std::collections::HashMap;
    let map_default: HashMap<String, i32> = Default::default();
    println!("HashMap::default() = {:?}", map_default);

    println!("\n=== Deriving Default for Structs ===\n");

    #[derive(Debug, Default)]
    struct Config {
        debug: bool,  // defaults to false
        timeout: u32, // defaults to 0
        name: String, // defaults to ""
    }

    let default_config = Config::default();
    println!("Config::default() = {:?}", default_config);

    // Create with some fields, default the rest
    let custom_config = Config {
        debug: true,
        ..Default::default()
    };
    println!("Custom config = {:?}", custom_config);

    println!("\n=== Custom Default Implementation ===\n");

    #[derive(Debug)]
    struct Settings {
        volume: u32,
        brightness: u32,
        theme: String,
    }

    impl Default for Settings {
        fn default() -> Self {
            Settings {
                volume: 50, // Sensible defaults
                brightness: 75,
                theme: String::from("dark"),
            }
        }
    }

    let default_settings = Settings::default();
    println!("Settings::default() = {:?}", default_settings);

    // Override some defaults
    let custom_settings = Settings {
        volume: 100,
        ..Default::default()
    };
    println!("Custom settings = {:?}", custom_settings);

    println!("\n=== Default with Option ===\n");

    // Option<T>::default() is None
    let opt_int: Option<i32> = Default::default();
    let opt_string: Option<String> = Default::default();

    println!("Option::<i32>::default() = {:?}", opt_int);
    println!("Option::<String>::default() = {:?}", opt_string);

    #[derive(Debug, Default)]
    struct User {
        name: String,
        email: Option<String>,
        age: Option<u32>,
    }

    let user = User {
        name: String::from("Alice"),
        ..Default::default() // email and age are None
    };
    println!("\nUser with defaults = {:?}", user);

    println!("\n=== Default in Generics ===\n");

    fn create_default<T: Default>() -> T {
        T::default()
    }

    let int: i32 = create_default();
    let vec: Vec<String> = create_default();
    let string: String = create_default();

    println!("create_default::<i32>() = {}", int);
    println!("create_default::<Vec<String>>() = {:?}", vec);
    println!("create_default::<String>() = '{}'", string);

    println!("\n=== unwrap_or_default() Pattern ===\n");

    // Get value from Option, or use default
    let maybe_number: Option<i32> = Some(42);
    let number = maybe_number.unwrap_or_default();
    println!("Some(42).unwrap_or_default() = {}", number);

    let no_number: Option<i32> = None;
    let number = no_number.unwrap_or_default();
    println!("None::<i32>.unwrap_or_default() = {}", number);

    // Works with Result too
    let result: Result<String, &str> = Err("error");
    let value = result.unwrap_or_default();
    println!("Err.unwrap_or_default() = '{}'", value);

    println!("\n=== Default in Entry API ===\n");

    let mut counts: HashMap<char, i32> = HashMap::new();

    for c in "hello world".chars() {
        // or_default() uses Default::default()
        *counts.entry(c).or_default() += 1;
    }
    println!("Character counts: {:?}", counts);

    println!("\n=== Builder Pattern with Default ===\n");

    #[derive(Debug, Default)]
    struct ServerConfig {
        host: String,
        port: u16,
        max_connections: u32,
        timeout_seconds: u32,
        ssl_enabled: bool,
    }

    impl ServerConfig {
        fn new() -> Self {
            ServerConfig {
                host: String::from("localhost"),
                port: 8080,
                max_connections: 100,
                timeout_seconds: 30,
                ssl_enabled: false,
            }
        }

        fn host(mut self, host: &str) -> Self {
            self.host = host.to_string();
            self
        }

        fn port(mut self, port: u16) -> Self {
            self.port = port;
            self
        }

        fn ssl(mut self, enabled: bool) -> Self {
            self.ssl_enabled = enabled;
            self
        }
    }

    // Using Default::default()
    let config1 = ServerConfig::default();
    println!("Default config: {:?}", config1);

    // Using custom new()
    let config2 = ServerConfig::new();
    println!("Custom new config: {:?}", config2);

    // Using builder pattern
    let config3 = ServerConfig::new().host("0.0.0.0").port(443).ssl(true);
    println!("Builder config: {:?}", config3);

    println!("\n=== Default for Nested Structs ===\n");

    #[derive(Debug, Default)]
    struct Address {
        street: String,
        city: String,
        country: String,
    }

    #[derive(Debug, Default)]
    struct Person {
        name: String,
        age: u32,
        address: Address, // Address also implements Default
    }

    let person = Person {
        name: String::from("Alice"),
        ..Default::default()
    };
    println!("Person with nested defaults: {:?}", person);

    println!("\n=== Practical Example: Configuration ===\n");

    #[derive(Debug)]
    struct AppConfig {
        database_url: String,
        pool_size: u32,
        log_level: String,
        cache_enabled: bool,
    }

    impl Default for AppConfig {
        fn default() -> Self {
            AppConfig {
                database_url: String::from("postgres://localhost/app"),
                pool_size: 10,
                log_level: String::from("info"),
                cache_enabled: true,
            }
        }
    }

    // Load config with environment overrides (simulated)
    fn load_config() -> AppConfig {
        let mut config = AppConfig::default();

        // Simulate reading from environment
        let env_db_url = std::env::var("DATABASE_URL").ok();
        if let Some(url) = env_db_url {
            config.database_url = url;
        }

        config
    }

    let config = load_config();
    println!("Loaded config: {:?}", config);
}
