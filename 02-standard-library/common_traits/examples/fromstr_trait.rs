// Example: FromStr Parsing Trait
//
// Demonstrates:
// - Using FromStr for string parsing
// - str::parse() method
// - Implementing FromStr for custom types
// - Error handling in parsing

use std::str::FromStr;

fn main() {
    println!("=== Basic FromStr Usage ===\n");

    // FromStr enables str::parse()
    let number: i32 = "42".parse().unwrap();
    println!("\"42\".parse::<i32>() = {}", number);

    let float: f64 = "3.14".parse().unwrap();
    println!("\"3.14\".parse::<f64>() = {}", float);

    let boolean: bool = "true".parse().unwrap();
    println!("\"true\".parse::<bool>() = {}", boolean);

    println!("\n=== Using FromStr::from_str() ===\n");

    // Direct FromStr trait usage
    let n = i32::from_str("100").unwrap();
    println!("i32::from_str(\"100\") = {}", n);

    let f = f64::from_str("2.718").unwrap();
    println!("f64::from_str(\"2.718\") = {}", f);

    let b = bool::from_str("false").unwrap();
    println!("bool::from_str(\"false\") = {}", b);

    println!("\n=== Handling Parse Errors ===\n");

    // parse() returns Result
    let result: Result<i32, _> = "not_a_number".parse();
    match result {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }

    // Using unwrap_or for default
    let n: i32 = "invalid".parse().unwrap_or(0);
    println!("\"invalid\".parse().unwrap_or(0) = {}", n);

    // Using ok() to convert to Option
    let maybe: Option<i32> = "123".parse().ok();
    println!("\"123\".parse().ok() = {:?}", maybe);

    let maybe: Option<i32> = "bad".parse().ok();
    println!("\"bad\".parse().ok() = {:?}", maybe);

    println!("\n=== Implementing FromStr for Custom Type ===\n");

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct ParsePointError {
        message: String,
    }

    impl std::fmt::Display for ParsePointError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ParsePointError: {}", self.message)
        }
    }

    impl FromStr for Point {
        type Err = ParsePointError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // Expected format: "(x, y)" or "x,y"
            let s = s.trim();
            let s = s.trim_matches(|c| c == '(' || c == ')');

            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 2 {
                return Err(ParsePointError {
                    message: format!("Expected 2 coordinates, got {}", parts.len()),
                });
            }

            let x = parts[0].trim().parse().map_err(|_| ParsePointError {
                message: format!("Invalid x coordinate: '{}'", parts[0]),
            })?;

            let y = parts[1].trim().parse().map_err(|_| ParsePointError {
                message: format!("Invalid y coordinate: '{}'", parts[1]),
            })?;

            Ok(Point { x, y })
        }
    }

    // Now we can use parse() with Point
    let point: Point = "(10, 20)".parse().unwrap();
    println!("\"(10, 20)\".parse::<Point>() = {:?}", point);

    let point: Point = "5, 15".parse().unwrap();
    println!("\"5, 15\".parse::<Point>() = {:?}", point);

    // Handle errors
    let result: Result<Point, _> = "invalid".parse();
    match result {
        Ok(p) => println!("Parsed: {:?}", p),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== FromStr for Enums ===\n");

    #[derive(Debug, PartialEq)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    #[derive(Debug)]
    struct ParseColorError;

    impl std::fmt::Display for ParseColorError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Unknown color")
        }
    }

    impl FromStr for Color {
        type Err = ParseColorError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim().to_lowercase().as_str() {
                "red" => Ok(Color::Red),
                "green" => Ok(Color::Green),
                "blue" => Ok(Color::Blue),
                _ => Err(ParseColorError),
            }
        }
    }

    let color: Color = "Red".parse().unwrap();
    println!("\"Red\".parse::<Color>() = {:?}", color);

    let color: Color = "BLUE".parse().unwrap();
    println!("\"BLUE\".parse::<Color>() = {:?}", color);

    let result: Result<Color, _> = "yellow".parse();
    println!("\"yellow\".parse::<Color>() = {:?}", result);

    println!("\n=== FromStr for Complex Types ===\n");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    #[derive(Debug)]
    struct ParsePersonError(String);

    impl std::fmt::Display for ParsePersonError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ParsePersonError: {}", self.0)
        }
    }

    impl FromStr for Person {
        type Err = ParsePersonError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // Expected format: "name:age"
            let parts: Vec<&str> = s.split(':').collect();
            if parts.len() != 2 {
                return Err(ParsePersonError("Expected format 'name:age'".to_string()));
            }

            let name = parts[0].trim().to_string();
            if name.is_empty() {
                return Err(ParsePersonError("Name cannot be empty".to_string()));
            }

            let age: u32 = parts[1]
                .trim()
                .parse()
                .map_err(|_| ParsePersonError(format!("Invalid age: '{}'", parts[1])))?;

            Ok(Person { name, age })
        }
    }

    let person: Person = "Alice:30".parse().unwrap();
    println!("\"Alice:30\".parse::<Person>() = {:?}", person);

    let result: Result<Person, _> = "Bob:invalid".parse();
    println!("\"Bob:invalid\".parse() = {:?}", result);

    println!("\n=== FromStr in Generic Functions ===\n");

    fn parse_or_default<T: FromStr + Default>(s: &str) -> T {
        s.parse().unwrap_or_default()
    }

    let n: i32 = parse_or_default("42");
    println!("parse_or_default::<i32>(\"42\") = {}", n);

    let n: i32 = parse_or_default("invalid");
    println!("parse_or_default::<i32>(\"invalid\") = {}", n);

    fn parse_all<T: FromStr>(inputs: &[&str]) -> Vec<T>
    where
        T::Err: std::fmt::Debug,
    {
        inputs.iter().filter_map(|s| s.parse().ok()).collect()
    }

    let numbers: Vec<i32> = parse_all(&["1", "2", "three", "4", "5"]);
    println!(
        "parse_all([\"1\", \"2\", \"three\", \"4\", \"5\"]) = {:?}",
        numbers
    );

    println!("\n=== Practical Example: Config Parsing ===\n");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
        debug: bool,
    }

    #[derive(Debug)]
    struct ConfigParseError(String);

    impl std::fmt::Display for ConfigParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Config error: {}", self.0)
        }
    }

    impl FromStr for Config {
        type Err = ConfigParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut host = String::from("localhost");
            let mut port: u16 = 8080;
            let mut debug = false;

            for line in s.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() != 2 {
                    continue;
                }

                let key = parts[0].trim();
                let value = parts[1].trim();

                match key {
                    "host" => host = value.to_string(),
                    "port" => {
                        port = value
                            .parse()
                            .map_err(|_| ConfigParseError(format!("Invalid port: {}", value)))?;
                    }
                    "debug" => {
                        debug = value.parse().map_err(|_| {
                            ConfigParseError(format!("Invalid debug value: {}", value))
                        })?;
                    }
                    _ => {} // Ignore unknown keys
                }
            }

            Ok(Config { host, port, debug })
        }
    }

    let config_text = r#"
        # Server configuration
        host = 0.0.0.0
        port = 3000
        debug = true
    "#;

    let config: Config = config_text.parse().unwrap();
    println!("Parsed config: {:?}", config);
}
