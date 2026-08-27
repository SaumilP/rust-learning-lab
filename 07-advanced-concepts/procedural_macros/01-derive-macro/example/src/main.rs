use my_derive::{HelloWorld, FieldNames, EnumIter, CustomDefault};

// Define the trait that our derive macro will implement
trait HelloWorld {
    fn hello_world(&self);
}

// Example 1: Simple HelloWorld derive
#[derive(HelloWorld)]
struct Person {
    name: String,
    age: u32,
}

#[derive(HelloWorld)]
struct Robot;

// Example 2: FieldNames derive
#[derive(FieldNames)]
struct User {
    id: u32,
    username: String,
    email: String,
    is_active: bool,
}

// Example 3: EnumIter derive
#[derive(Debug, Clone, Copy, EnumIter)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Example 4: CustomDefault derive
#[derive(Debug, CustomDefault)]
struct Config {
    #[default_value = "String::from(\"localhost\")"]
    host: String,

    #[default_value = "8080"]
    port: u16,

    #[default_value = "true"]
    enabled: bool,

    // Uses Default::default() for this field
    timeout: u64,
}

fn main() {
    println!("🎯 Derive Macro Examples\n");

    // Example 1: HelloWorld
    println!("=== 1. HelloWorld Derive ===");
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    person.hello_world();

    let robot = Robot;
    robot.hello_world();

    println!();

    // Example 2: FieldNames
    println!("=== 2. FieldNames Derive ===");
    let field_names = User::field_names();
    println!("User fields: {:?}", field_names);
    println!("Field count: {}", field_names.len());

    println!();

    // Example 3: EnumIter
    println!("=== 3. EnumIter Derive ===");
    println!("Colors:");
    for color in Color::iter() {
        println!("  - {:?}", color);
    }
    println!("Total colors: {}", Color::variant_count());

    println!("\nDirections:");
    for direction in Direction::iter() {
        println!("  - {:?}", direction);
    }

    println!();

    // Example 4: CustomDefault
    println!("=== 4. CustomDefault Derive ===");
    let config = Config::default();
    println!("Config: {:?}", config);
    println!("  Host: {}", config.host);
    println!("  Port: {}", config.port);
    println!("  Enabled: {}", config.enabled);
    println!("  Timeout: {}", config.timeout);

    println!("\n✅ All derive macro examples completed!");
}
