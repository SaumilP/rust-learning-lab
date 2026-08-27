use my_builder::Builder;

// Example 1: Simple struct with all required fields
#[derive(Debug, Builder)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// Example 2: Struct with optional fields
#[derive(Debug, Builder)]
struct Product {
    id: u32,
    name: String,
    #[builder(optional)]
    description: Option<String>,
    #[builder(optional)]
    price: Option<f64>,
}

// Example 3: Configuration struct
#[derive(Debug, Builder)]
struct ServerConfig {
    host: String,
    port: u16,
    #[builder(optional)]
    workers: Option<usize>,
    #[builder(optional)]
    timeout_secs: Option<u64>,
    #[builder(optional)]
    enable_logging: Option<bool>,
}

// Example 4: Complex nested structure
#[derive(Debug, Builder)]
struct DatabaseConnection {
    host: String,
    port: u16,
    database: String,
    username: String,
    #[builder(optional)]
    password: Option<String>,
    #[builder(optional)]
    max_connections: Option<u32>,
    #[builder(optional)]
    ssl: Option<bool>,
}

fn main() {
    println!("🏗️  Builder Pattern Examples\n");

    // Example 1: All required fields
    println!("=== 1. Simple Builder ===");
    let user = User::builder()
        .id(1)
        .name("Alice".to_string())
        .email("alice@example.com".to_string())
        .build()
        .unwrap();
    println!("User: {:?}\n", user);

    // Example 2: With optional fields
    println!("=== 2. Builder with Optional Fields ===");
    let product1 = Product::builder()
        .id(101)
        .name("Laptop".to_string())
        .description("High-performance laptop".to_string())
        .price(999.99)
        .build()
        .unwrap();
    println!("Product 1: {:?}", product1);

    let product2 = Product::builder()
        .id(102)
        .name("Mouse".to_string())
        .build()
        .unwrap();
    println!("Product 2 (minimal): {:?}\n", product2);

    // Example 3: Configuration
    println!("=== 3. Server Configuration ===");
    let config = ServerConfig::builder()
        .host("0.0.0.0".to_string())
        .port(8080)
        .workers(4)
        .enable_logging(true)
        .build()
        .unwrap();
    println!("Server config: {:?}\n", config);

    // Example 4: Database connection
    println!("=== 4. Database Connection ===");
    let db_conn = DatabaseConnection::builder()
        .host("localhost".to_string())
        .port(5432)
        .database("myapp".to_string())
        .username("admin".to_string())
        .password("secret123".to_string())
        .max_connections(10)
        .ssl(true)
        .build()
        .unwrap();
    println!("Database: {:?}\n", db_conn);

    // Example 5: Error handling - missing required field
    println!("=== 5. Error Handling ===");
    let result = User::builder()
        .id(2)
        .name("Bob".to_string())
        // Missing email field
        .build();

    match result {
        Ok(user) => println!("User created: {:?}", user),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n✅ All builder pattern examples completed!");
}
