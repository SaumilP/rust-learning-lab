// Example: String Parsing - parse() and FromStr
//
// Demonstrates:
// - Using parse() to convert strings to other types
// - Handling parse errors with Result
// - The FromStr trait
// - Parsing different numeric types

fn main() {
    println!("=== Basic parse() Usage ===\n");

    // parse() with turbofish syntax
    let number: i32 = "42".parse().unwrap();
    println!("\"42\".parse::<i32>() = {}", number);

    // Alternative: type annotation
    let number: f64 = "3.14159".parse().unwrap();
    println!("\"3.14159\".parse::<f64>() = {}", number);

    // With turbofish
    let boolean = "true".parse::<bool>().unwrap();
    println!("\"true\".parse::<bool>() = {}", boolean);

    println!("\n=== Parsing Different Numeric Types ===\n");

    // Integers
    let int8: i8 = "127".parse().unwrap();
    let int16: i16 = "32000".parse().unwrap();
    let int32: i32 = "2147483647".parse().unwrap();
    let int64: i64 = "9223372036854775807".parse().unwrap();

    println!("i8:  {}", int8);
    println!("i16: {}", int16);
    println!("i32: {}", int32);
    println!("i64: {}", int64);

    // Unsigned integers
    let uint8: u8 = "255".parse().unwrap();
    let uint32: u32 = "4294967295".parse().unwrap();

    println!("\nu8:  {}", uint8);
    println!("u32: {}", uint32);

    // Floating point
    let f32_val: f32 = "3.14".parse().unwrap();
    let f64_val: f64 = "2.718281828".parse().unwrap();

    println!("\nf32: {}", f32_val);
    println!("f64: {}", f64_val);

    println!("\n=== Handling Parse Errors ===\n");

    // parse() returns Result<T, ParseError>
    let result: Result<i32, _> = "42".parse();
    match result {
        Ok(n) => println!("Parsed successfully: {}", n),
        Err(e) => println!("Parse error: {}", e),
    }

    // Invalid number
    let result: Result<i32, _> = "not a number".parse();
    match result {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Failed to parse 'not a number': {}", e),
    }

    // Overflow
    let result: Result<i8, _> = "200".parse(); // i8 max is 127
    match result {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Failed to parse '200' as i8: {}", e),
    }

    println!("\n=== Safe Parsing Patterns ===\n");

    // Using unwrap_or for default value
    let value: i32 = "invalid".parse().unwrap_or(0);
    println!("\"invalid\".parse().unwrap_or(0) = {}", value);

    // Using unwrap_or_default
    let value: i32 = "bad".parse().unwrap_or_default();
    println!("\"bad\".parse().unwrap_or_default() = {}", value);

    // Using ok() to convert to Option
    let maybe_number: Option<i32> = "42".parse().ok();
    println!("\"42\".parse().ok() = {:?}", maybe_number);

    let maybe_number: Option<i32> = "invalid".parse().ok();
    println!("\"invalid\".parse().ok() = {:?}", maybe_number);

    // Using if let
    if let Ok(n) = "100".parse::<i32>() {
        println!("Parsed with if let: {}", n);
    }

    println!("\n=== Parsing with trim() ===\n");

    // Common pattern: trim before parsing
    let input = "   42   \n";
    let result: Result<i32, _> = input.trim().parse();
    println!("Parsing '{}' (trimmed): {:?}", input.escape_debug(), result);

    // Without trim, parsing fails
    let result_no_trim: Result<i32, _> = input.parse();
    println!("Parsing without trim: {:?}", result_no_trim);

    println!("\n=== Parsing in Functions ===\n");

    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.trim().parse()
    }

    println!("parse_number(\"123\"): {:?}", parse_number("123"));
    println!("parse_number(\"  456  \"): {:?}", parse_number("  456  "));
    println!("parse_number(\"abc\"): {:?}", parse_number("abc"));

    // Function returning Option
    fn try_parse(s: &str) -> Option<i32> {
        s.trim().parse().ok()
    }

    println!("\ntry_parse(\"789\"): {:?}", try_parse("789"));
    println!("try_parse(\"xyz\"): {:?}", try_parse("xyz"));

    println!("\n=== Parsing Collections ===\n");

    // Parse comma-separated numbers
    let csv = "1, 2, 3, 4, 5";
    let numbers: Vec<i32> = csv
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    println!("CSV '{}' -> {:?}", csv, numbers);

    // With proper error handling
    let csv = "1, 2, three, 4, 5";
    let numbers: Vec<Result<i32, _>> = csv.split(',').map(|s| s.trim().parse()).collect();
    println!("CSV with error '{}': {:?}", csv, numbers);

    // Collect only successful parses
    let valid: Vec<i32> = csv
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    println!("Valid numbers only: {:?}", valid);

    println!("\n=== FromStr Trait ===\n");

    // Types implementing FromStr can be parsed
    use std::str::FromStr;

    // Direct FromStr call
    let n = i32::from_str("42").unwrap();
    println!("i32::from_str(\"42\") = {}", n);

    // Parsing IP addresses
    use std::net::IpAddr;
    let ip: IpAddr = "192.168.1.1".parse().unwrap();
    println!("Parsed IP: {}", ip);

    // Parsing socket addresses
    use std::net::SocketAddr;
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("Parsed socket address: {}", addr);

    println!("\n=== Custom FromStr Implementation ===\n");

    // Custom type with FromStr
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl std::str::FromStr for Point {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = s
                .trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .collect();

            if parts.len() != 2 {
                return Err("Expected format: (x,y)".to_string());
            }

            let x = parts[0]
                .trim()
                .parse()
                .map_err(|_| "Invalid x coordinate")?;
            let y = parts[1]
                .trim()
                .parse()
                .map_err(|_| "Invalid y coordinate")?;

            Ok(Point { x, y })
        }
    }

    let point: Point = "(10, 20)".parse().unwrap();
    println!("Parsed point: {:?}", point);

    let point: Result<Point, _> = "(invalid, 20)".parse();
    println!("Invalid point: {:?}", point);

    println!("\n=== Practical Examples ===\n");

    // Example 1: Parse command line arguments style input
    fn parse_key_value(s: &str) -> Option<(String, i32)> {
        let parts: Vec<&str> = s.split('=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value: i32 = parts[1].trim().parse().ok()?;
            Some((key, value))
        } else {
            None
        }
    }

    println!(
        "parse_key_value(\"count=42\"): {:?}",
        parse_key_value("count=42")
    );
    println!(
        "parse_key_value(\"invalid\"): {:?}",
        parse_key_value("invalid")
    );

    // Example 2: Parse multiple numbers from user input
    fn parse_numbers(input: &str) -> Vec<i32> {
        input
            .split(|c: char| !c.is_numeric() && c != '-')
            .filter(|s| !s.is_empty())
            .filter_map(|s| s.parse().ok())
            .collect()
    }

    let input = "The values are 10, 20, and 30";
    println!("Numbers in '{}': {:?}", input, parse_numbers(input));
}
