// Example: Result Type - Ok and Err Variants
//
// Demonstrates:
// - Creating Result values with Ok and Err
// - Pattern matching on Results
// - Result methods and error handling
// - The ? operator for error propagation

fn main() {
    println!("=== Creating Result Values ===\n");

    // Result has two variants: Ok(value) and Err(error)
    let success: Result<i32, String> = Ok(42);
    let failure: Result<i32, String> = Err(String::from("Something went wrong"));

    println!("Ok(42) = {:?}", success);
    println!("Err(...) = {:?}", failure);

    // Type inference with Ok
    let parsed: Result<i32, _> = "42".parse();
    println!("\"42\".parse() = {:?}", parsed);

    let parse_error: Result<i32, _> = "not_a_number".parse();
    println!("\"not_a_number\".parse() = {:?}", parse_error);

    println!("\n=== Pattern Matching on Result ===\n");

    fn check_result(result: Result<i32, &str>) {
        match result {
            Ok(value) => println!("Success: {}", value),
            Err(error) => println!("Error: {}", error),
        }
    }

    check_result(Ok(100));
    check_result(Err("Operation failed"));

    println!("\n=== is_ok() and is_err() ===\n");

    let ok_result: Result<i32, &str> = Ok(10);
    let err_result: Result<i32, &str> = Err("error");

    println!("Ok(10).is_ok() = {}", ok_result.is_ok());
    println!("Ok(10).is_err() = {}", ok_result.is_err());
    println!("Err.is_ok() = {}", err_result.is_ok());
    println!("Err.is_err() = {}", err_result.is_err());

    // Use in conditions
    if ok_result.is_ok() {
        println!("Operation succeeded!");
    }

    println!("\n=== if let Pattern ===\n");

    let result: Result<i32, &str> = Ok(42);

    if let Ok(value) = result {
        println!("Got value: {}", value);
    }

    let error: Result<i32, &str> = Err("failed");

    if let Err(e) = error {
        println!("Got error: {}", e);
    }

    println!("\n=== unwrap() and expect() ===\n");

    // unwrap() - panics on Err
    let ok_value: Result<i32, &str> = Ok(100);
    let value = ok_value.unwrap();
    println!("Ok(100).unwrap() = {}", value);

    // expect() - panics with custom message
    let value = Ok::<i32, &str>(200).expect("Should have a value");
    println!("Ok(200).expect(...) = {}", value);

    // These would panic:
    // Err("error").unwrap();
    // Err("error").expect("This will panic");

    println!("\n=== unwrap_or() and unwrap_or_else() ===\n");

    let ok: Result<i32, &str> = Ok(42);
    let err: Result<i32, &str> = Err("error");

    // unwrap_or - use default on error
    println!("Ok(42).unwrap_or(0) = {}", ok.unwrap_or(0));
    println!("Err.unwrap_or(0) = {}", err.unwrap_or(0));

    // unwrap_or_default - use Default trait
    let err: Result<i32, &str> = Err("error");
    println!("Err.unwrap_or_default() = {}", err.unwrap_or_default());

    // unwrap_or_else - lazy default with closure
    let value = err.unwrap_or_else(|e| {
        println!("  Error was: {}, using fallback", e);
        -1
    });
    println!("Err.unwrap_or_else(...) = {}", value);

    println!("\n=== map() and map_err() ===\n");

    let result: Result<i32, &str> = Ok(5);

    // map transforms the Ok value
    let doubled = result.map(|x| x * 2);
    println!("Ok(5).map(|x| x * 2) = {:?}", doubled);

    let string_result = result.map(|x| x.to_string());
    println!("Ok(5).map(|x| x.to_string()) = {:?}", string_result);

    // map on Err returns the Err unchanged
    let err: Result<i32, &str> = Err("error");
    let mapped = err.map(|x| x * 2);
    println!("Err.map(...) = {:?}", mapped);

    // map_err transforms the error
    let result: Result<i32, &str> = Err("original error");
    let mapped = result.map_err(|e| format!("Wrapped: {}", e));
    println!("Err.map_err(...) = {:?}", mapped);

    println!("\n=== and_then() - Chaining Results ===\n");

    fn parse_number(s: &str) -> Result<i32, String> {
        s.parse().map_err(|_| format!("Cannot parse '{}'", s))
    }

    fn validate_positive(n: i32) -> Result<i32, String> {
        if n > 0 {
            Ok(n)
        } else {
            Err(format!("{} is not positive", n))
        }
    }

    // Chain operations that can fail
    let result = parse_number("42").and_then(validate_positive);
    println!("parse(\"42\").and_then(validate) = {:?}", result);

    let result = parse_number("-5").and_then(validate_positive);
    println!("parse(\"-5\").and_then(validate) = {:?}", result);

    let result = parse_number("invalid").and_then(validate_positive);
    println!("parse(\"invalid\").and_then(validate) = {:?}", result);

    println!("\n=== or() and or_else() - Fallback Results ===\n");

    let primary: Result<i32, &str> = Err("primary failed");
    let backup: Result<i32, &str> = Ok(100);

    // Use backup if primary failed
    let result = primary.or(backup);
    println!("Err.or(Ok(100)) = {:?}", result);

    let result = Ok::<i32, &str>(42).or(backup);
    println!("Ok(42).or(Ok(100)) = {:?}", result);

    // or_else with lazy evaluation
    let result: Result<i32, &str> = primary.or_else(|e| {
        println!("  Primary failed with: {}, trying backup", e);
        Ok(999)
    });
    println!("Err.or_else(...) = {:?}", result);

    println!("\n=== The ? Operator ===\n");

    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    fn complex_calculation() -> Result<i32, String> {
        let x = divide(100, 2)?; // Returns Err early if division fails
        let y = divide(x, 5)?; // Same here
        let z = divide(y, 2)?; // And here
        Ok(z)
    }

    match complex_calculation() {
        Ok(result) => println!("Calculation result: {}", result),
        Err(e) => println!("Calculation failed: {}", e),
    }

    fn failing_calculation() -> Result<i32, String> {
        let x = divide(100, 0)?; // This will return early with Err
        Ok(x * 2) // This won't execute
    }

    match failing_calculation() {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Failed: {}", e),
    }

    println!("\n=== Converting Result to Option ===\n");

    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("error");

    // ok() converts Ok to Some, Err to None
    println!("Ok(42).ok() = {:?}", ok_result.ok());
    println!("Err.ok() = {:?}", err_result.ok());

    // err() converts Err to Some, Ok to None
    println!("Ok(42).err() = {:?}", ok_result.err());
    println!("Err.err() = {:?}", err_result.err());

    println!("\n=== Practical Examples ===\n");

    // Example 1: File operation simulation
    fn read_config(path: &str) -> Result<String, String> {
        if path == "config.txt" {
            Ok(String::from("key=value"))
        } else {
            Err(format!("File not found: {}", path))
        }
    }

    match read_config("config.txt") {
        Ok(content) => println!("Config: {}", content),
        Err(e) => println!("Error: {}", e),
    }

    match read_config("missing.txt") {
        Ok(content) => println!("Config: {}", content),
        Err(e) => println!("Error: {}", e),
    }

    // Example 2: Validation chain
    fn validate_username(name: &str) -> Result<&str, &str> {
        if name.is_empty() {
            Err("Username cannot be empty")
        } else if name.len() < 3 {
            Err("Username too short")
        } else if name.len() > 20 {
            Err("Username too long")
        } else {
            Ok(name)
        }
    }

    fn validate_email(email: &str) -> Result<&str, &str> {
        if email.contains('@') {
            Ok(email)
        } else {
            Err("Invalid email format")
        }
    }

    println!("\nValidation examples:");
    println!(
        "validate_username(\"alice\"): {:?}",
        validate_username("alice")
    );
    println!("validate_username(\"ab\"): {:?}", validate_username("ab"));
    println!(
        "validate_email(\"test@example.com\"): {:?}",
        validate_email("test@example.com")
    );
    println!(
        "validate_email(\"invalid\"): {:?}",
        validate_email("invalid")
    );
}
