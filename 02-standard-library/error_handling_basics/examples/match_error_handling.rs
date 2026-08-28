// Example: Pattern Matching on Results
//
// Demonstrates:
// - Using match for comprehensive error handling
// - Extracting values and errors
// - Nested matching
// - Combining match with other techniques

fn main() {
    println!("=== Basic Match on Result ===\n");

    let result: Result<i32, &str> = Ok(42);

    match result {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    let error: Result<i32, &str> = Err("Something went wrong");

    match error {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    println!("\n=== Match with Value Processing ===\n");

    fn divide(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    let result = divide(100, 5);
    let doubled = match result {
        Ok(value) => value * 2,
        Err(_) => 0, // Default to 0 on error
    };
    println!("divide(100, 5) doubled = {}", doubled);

    let result = divide(100, 0);
    let doubled = match result {
        Ok(value) => value * 2,
        Err(e) => {
            println!("  Error occurred: {}", e);
            0
        }
    };
    println!("divide(100, 0) doubled = {}", doubled);

    println!("\n=== Match on Option ===\n");

    fn find_user(id: u32) -> Option<String> {
        match id {
            1 => Some(String::from("Alice")),
            2 => Some(String::from("Bob")),
            _ => None,
        }
    }

    let user = find_user(1);
    match user {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }

    let user = find_user(99);
    match user {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }

    println!("\n=== Match with Guards ===\n");

    fn check_value(result: Result<i32, &str>) {
        match result {
            Ok(n) if n > 100 => println!("Large value: {}", n),
            Ok(n) if n > 50 => println!("Medium value: {}", n),
            Ok(n) if n > 0 => println!("Small positive value: {}", n),
            Ok(n) => println!("Non-positive value: {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    check_value(Ok(150));
    check_value(Ok(75));
    check_value(Ok(25));
    check_value(Ok(-10));
    check_value(Err("failed"));

    println!("\n=== Matching Different Error Types ===\n");

    #[derive(Debug)]
    enum AppError {
        NotFound(String),
        PermissionDenied,
        NetworkError(String),
        Unknown,
    }

    fn process_request(request: &str) -> Result<String, AppError> {
        match request {
            "data" => Ok(String::from("Here's your data")),
            "secret" => Err(AppError::PermissionDenied),
            "remote" => Err(AppError::NetworkError(String::from("Connection timeout"))),
            "missing" => Err(AppError::NotFound(String::from("Resource not found"))),
            _ => Err(AppError::Unknown),
        }
    }

    fn handle_request(request: &str) {
        match process_request(request) {
            Ok(data) => println!("Success: {}", data),
            Err(AppError::NotFound(msg)) => println!("Not found: {}", msg),
            Err(AppError::PermissionDenied) => println!("Access denied"),
            Err(AppError::NetworkError(msg)) => println!("Network error: {}", msg),
            Err(AppError::Unknown) => println!("Unknown error occurred"),
        }
    }

    for req in ["data", "secret", "remote", "missing", "other"] {
        print!("Request '{}': ", req);
        handle_request(req);
    }

    println!("\n=== Nested Match ===\n");

    fn parse_and_calculate(input: &str) -> Result<i32, String> {
        match input.parse::<i32>() {
            Ok(n) => match n.checked_mul(2) {
                Some(result) => Ok(result),
                None => Err(String::from("Overflow occurred")),
            },
            Err(_) => Err(format!("Cannot parse '{}'", input)),
        }
    }

    println!(
        "parse_and_calculate(\"21\"): {:?}",
        parse_and_calculate("21")
    );
    println!(
        "parse_and_calculate(\"abc\"): {:?}",
        parse_and_calculate("abc")
    );

    println!("\n=== Match with Binding ===\n");

    fn process_value(opt: Option<i32>) {
        match opt {
            Some(n @ 1..=10) => println!("Small number: {}", n),
            Some(n @ 11..=100) => println!("Medium number: {}", n),
            Some(n) => println!("Large number: {}", n),
            None => println!("No value"),
        }
    }

    process_value(Some(5));
    process_value(Some(50));
    process_value(Some(500));
    process_value(None);

    println!("\n=== Match with Tuple Results ===\n");

    fn process_two_results(r1: Result<i32, &str>, r2: Result<i32, &str>) -> i32 {
        match (r1, r2) {
            (Ok(a), Ok(b)) => {
                println!("Both succeeded: {} + {} = {}", a, b, a + b);
                a + b
            }
            (Ok(a), Err(e)) => {
                println!("Second failed ({}), using first: {}", e, a);
                a
            }
            (Err(e), Ok(b)) => {
                println!("First failed ({}), using second: {}", e, b);
                b
            }
            (Err(e1), Err(e2)) => {
                println!("Both failed: {} and {}", e1, e2);
                0
            }
        }
    }

    process_two_results(Ok(10), Ok(20));
    process_two_results(Ok(10), Err("error2"));
    process_two_results(Err("error1"), Ok(20));
    process_two_results(Err("error1"), Err("error2"));

    println!("\n=== Match in Loops ===\n");

    let values = vec!["1", "2", "three", "4", "five"];

    let mut sum = 0;
    for value in &values {
        match value.parse::<i32>() {
            Ok(n) => {
                println!("  Parsed '{}' as {}", value, n);
                sum += n;
            }
            Err(_) => {
                println!("  Could not parse '{}'", value);
            }
        }
    }
    println!("Sum of parsed values: {}", sum);

    println!("\n=== Match with Early Return ===\n");

    fn calculate(a: &str, b: &str) -> Result<i32, String> {
        let x = match a.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(format!("Invalid first number: {}", a)),
        };

        let y = match b.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return Err(format!("Invalid second number: {}", b)),
        };

        Ok(x + y)
    }

    println!("calculate(\"10\", \"20\"): {:?}", calculate("10", "20"));
    println!("calculate(\"ten\", \"20\"): {:?}", calculate("ten", "20"));
    println!(
        "calculate(\"10\", \"twenty\"): {:?}",
        calculate("10", "twenty")
    );

    println!("\n=== Exhaustive Matching ===\n");

    // Rust requires exhaustive matching
    enum Status {
        Active,
        Inactive,
        Pending,
    }

    fn describe_status(status: Status) -> &'static str {
        match status {
            Status::Active => "Currently active",
            Status::Inactive => "Not active",
            Status::Pending => "Waiting for activation",
            // All variants must be covered!
        }
    }

    println!("Active: {}", describe_status(Status::Active));
    println!("Inactive: {}", describe_status(Status::Inactive));
    println!("Pending: {}", describe_status(Status::Pending));

    println!("\n=== Match vs if let ===\n");

    // Use match when handling both cases
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(v) => println!("Got: {}", v),
        Err(e) => println!("Error: {}", e),
    }

    // Use if let when only interested in one case
    if let Ok(v) = result {
        println!("Using if let, got: {}", v);
    }

    // Use if let with else for optional error handling
    let error: Result<i32, &str> = Err("oops");
    if let Ok(v) = error {
        println!("Got: {}", v);
    } else {
        println!("Something went wrong");
    }

    println!("\n=== Practical Example: Command Processing ===\n");

    enum Command {
        Add(i32),
        Subtract(i32),
        Multiply(i32),
        Divide(i32),
    }

    fn execute_command(value: i32, cmd: Command) -> Result<i32, String> {
        match cmd {
            Command::Add(n) => Ok(value + n),
            Command::Subtract(n) => Ok(value - n),
            Command::Multiply(n) => Ok(value * n),
            Command::Divide(n) => {
                if n == 0 {
                    Err(String::from("Cannot divide by zero"))
                } else {
                    Ok(value / n)
                }
            }
        }
    }

    let commands = vec![
        Command::Add(10),
        Command::Multiply(3),
        Command::Divide(2),
        Command::Subtract(5),
    ];

    let mut result = 0;
    println!("Starting with: {}", result);

    for cmd in commands {
        match execute_command(result, cmd) {
            Ok(new_value) => {
                println!("  -> {}", new_value);
                result = new_value;
            }
            Err(e) => {
                println!("  Error: {}", e);
                break;
            }
        }
    }
    println!("Final result: {}", result);
}
