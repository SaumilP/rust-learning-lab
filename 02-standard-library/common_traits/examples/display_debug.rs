#![allow(dead_code)]

// Example: Display and Debug Traits
//
// Demonstrates:
// - Debug trait for developer output
// - Display trait for user-facing output
// - Deriving Debug automatically
// - Implementing Display manually
// - Pretty printing with {:#?}

use std::fmt;

fn main() {
    println!("=== Debug Trait - {{:?}} ===\n");

    // Debug is for developers - shows internal structure
    let number = 42;
    let text = "hello";
    let vector = vec![1, 2, 3];

    println!("Debug format with {{:?}}:");
    println!("  number: {:?}", number);
    println!("  text: {:?}", text);
    println!("  vector: {:?}", vector);

    // Debug shows structure, including quotes for strings
    let string = String::from("hello world");
    println!("  String: {:?}", string);

    println!("\n=== Pretty Debug - {{:#?}} ===\n");

    // {:#?} formats with indentation
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        hobbies: Vec<String>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
        hobbies: vec![
            String::from("reading"),
            String::from("coding"),
            String::from("hiking"),
        ],
    };

    println!("Regular debug {{:?}}:");
    println!("{:?}", person);

    println!("\nPretty debug {{:#?}}:");
    println!("{:#?}", person);

    println!("\n=== Deriving Debug ===\n");

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 10, y: 0 },
    };

    println!("Rectangle: {:?}", rect);
    println!("\nPretty:\n{:#?}", rect);

    println!("\n=== Display Trait - {{}} ===\n");

    // Display is for end users - clean, human-readable output
    let number = 42;
    let text = "hello";

    println!("Display format with {{}}:");
    println!("  number: {}", number);
    println!("  text: {}", text);

    // Note: Vec doesn't implement Display, only Debug
    // println!("{}", vec![1, 2, 3]);  // ERROR

    println!("\n=== Implementing Display ===\n");

    struct Coordinate {
        lat: f64,
        lon: f64,
    }

    impl fmt::Display for Coordinate {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({:.4}, {:.4})", self.lat, self.lon)
        }
    }

    // Also implement Debug for completeness
    impl fmt::Debug for Coordinate {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Coordinate")
                .field("lat", &self.lat)
                .field("lon", &self.lon)
                .finish()
        }
    }

    let location = Coordinate {
        lat: 37.7749,
        lon: -122.4194,
    };
    println!("Display: {}", location);
    println!("Debug: {:?}", location);

    println!("\n=== Display for Custom Types ===\n");

    struct Person2 {
        name: String,
        age: u32,
    }

    impl fmt::Display for Person2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} ({} years old)", self.name, self.age)
        }
    }

    impl fmt::Debug for Person2 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Person2")
                .field("name", &self.name)
                .field("age", &self.age)
                .finish()
        }
    }

    let person = Person2 {
        name: String::from("Bob"),
        age: 25,
    };
    println!("Display: {}", person);
    println!("Debug: {:?}", person);

    println!("\n=== Display for Enums ===\n");

    enum Status {
        Active,
        Inactive,
        Pending(String),
    }

    impl fmt::Display for Status {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Status::Active => write!(f, "Active"),
                Status::Inactive => write!(f, "Inactive"),
                Status::Pending(reason) => write!(f, "Pending: {}", reason),
            }
        }
    }

    impl fmt::Debug for Status {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Status::Active => write!(f, "Status::Active"),
                Status::Inactive => write!(f, "Status::Inactive"),
                Status::Pending(reason) => write!(f, "Status::Pending({:?})", reason),
            }
        }
    }

    let statuses = vec![
        Status::Active,
        Status::Inactive,
        Status::Pending(String::from("Awaiting approval")),
    ];

    println!("Display:");
    for status in &statuses {
        println!("  {}", status);
    }

    println!("\nDebug:");
    for status in &statuses {
        println!("  {:?}", status);
    }

    println!("\n=== Using to_string() ===\n");

    // Types implementing Display get to_string() for free
    let coord = Coordinate {
        lat: 40.7128,
        lon: -74.0060,
    };
    let coord_string: String = coord.to_string();
    println!("to_string(): '{}'", coord_string);

    // Works with any Display type
    let number = 42;
    let number_string = number.to_string();
    println!("42.to_string(): '{}'", number_string);

    println!("\n=== Formatting Options with Display ===\n");

    struct Money {
        dollars: i64,
        cents: u8,
    }

    impl fmt::Display for Money {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Respect width and alignment specifiers
            let amount = format!("${}.{:02}", self.dollars, self.cents);
            if let Some(width) = f.width() {
                write!(f, "{:>width$}", amount)
            } else {
                write!(f, "{}", amount)
            }
        }
    }

    let price = Money {
        dollars: 42,
        cents: 99,
    };
    println!("Price: {}", price);
    println!("Price (width 15): '{:15}'", price);

    println!("\n=== dbg! Macro for Debugging ===\n");

    // dbg! prints to stderr with file:line info
    let x = 5;
    let y = dbg!(x * 2); // Prints to stderr and returns value
    println!("y = {}", y);

    // Works in expressions
    let result = dbg!(dbg!(2 + 2) * dbg!(3));
    println!("result = {}", result);

    println!("\n=== Comparison: Debug vs Display ===\n");

    struct Error {
        code: i32,
        message: String,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // User-friendly message
            write!(f, "Error: {}", self.message)
        }
    }

    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Developer details
            f.debug_struct("Error")
                .field("code", &self.code)
                .field("message", &self.message)
                .finish()
        }
    }

    let err = Error {
        code: 404,
        message: String::from("Not Found"),
    };

    println!("For users (Display): {}", err);
    println!("For developers (Debug): {:?}", err);

    println!("\n=== Implementing Debug Manually ===\n");

    struct Secret {
        username: String,
        password: String, // Don't want to show this
    }

    impl fmt::Debug for Secret {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Secret")
                .field("username", &self.username)
                .field("password", &"[REDACTED]")
                .finish()
        }
    }

    let secret = Secret {
        username: String::from("admin"),
        password: String::from("super_secret_123"),
    };

    println!("Secret with redacted password: {:?}", secret);
}
