// Example: unwrap() and expect() Methods
//
// Demonstrates:
// - When to use unwrap() and expect()
// - Differences between them
// - Safer alternatives
// - Best practices for error handling

fn main() {
    println!("=== Basic unwrap() Usage ===\n");

    // unwrap() extracts value from Some/Ok
    let some_value: Option<i32> = Some(42);
    let value = some_value.unwrap();
    println!("Some(42).unwrap() = {}", value);

    let ok_result: Result<i32, &str> = Ok(100);
    let value = ok_result.unwrap();
    println!("Ok(100).unwrap() = {}", value);

    // WARNING: unwrap() panics on None/Err
    // let none_value: Option<i32> = None;
    // none_value.unwrap();  // PANIC!

    // let err_result: Result<i32, &str> = Err("error");
    // err_result.unwrap();  // PANIC!

    println!("\n=== Basic expect() Usage ===\n");

    // expect() is like unwrap() but with a custom message
    let some_value: Option<i32> = Some(42);
    let value = some_value.expect("Value should be present");
    println!("Some(42).expect(...) = {}", value);

    let ok_result: Result<i32, &str> = Ok(100);
    let value = ok_result.expect("Operation should succeed");
    println!("Ok(100).expect(...) = {}", value);

    // expect() panics with your message
    // let none: Option<i32> = None;
    // none.expect("This will be in the panic message");
    // Output: thread 'main' panicked at 'This will be in the panic message'

    println!("\n=== unwrap() vs expect() ===\n");

    // unwrap() panic message is generic
    // none.unwrap();
    // Output: thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'

    // expect() panic message is descriptive
    // none.expect("Configuration file not found");
    // Output: thread 'main' panicked at 'Configuration file not found'

    println!("unwrap() gives generic panic messages");
    println!("expect() lets you provide context");

    println!("\n=== When unwrap() is Safe ===\n");

    // Case 1: You've already checked
    let maybe_value: Option<i32> = Some(42);
    if maybe_value.is_some() {
        let value = maybe_value.unwrap(); // Safe - we checked
        println!("Checked before unwrap: {}", value);
    }

    // Case 2: Logically impossible to be None
    let numbers = vec![1, 2, 3, 4, 5];
    if !numbers.is_empty() {
        let first = numbers.first().unwrap(); // Safe - vec is not empty
        println!("First of non-empty vec: {}", first);
    }

    // Case 3: Tests and prototypes
    fn test_function() {
        let value: Result<i32, &str> = Ok(42);
        let v = value.unwrap(); // OK in tests
        assert_eq!(v, 42);
    }
    test_function();
    println!("Test passed");

    println!("\n=== expect() for Documentation ===\n");

    // Use expect() to document invariants
    fn get_config_value(key: &str) -> Option<&str> {
        match key {
            "database_url" => Some("postgres://localhost/db"),
            "port" => Some("8080"),
            _ => None,
        }
    }

    // Documents that these config values are required
    let db_url =
        get_config_value("database_url").expect("DATABASE_URL environment variable must be set");
    let port = get_config_value("port").expect("PORT must be specified in config");

    println!("Database: {}", db_url);
    println!("Port: {}", port);

    println!("\n=== Safer Alternatives ===\n");

    let maybe_value: Option<i32> = None;
    let err_result: Result<i32, &str> = Err("error");

    // unwrap_or - provide default value
    let value = maybe_value.unwrap_or(0);
    println!("None.unwrap_or(0) = {}", value);

    let value = err_result.unwrap_or(-1);
    println!("Err.unwrap_or(-1) = {}", value);

    // unwrap_or_default - use Default trait
    let value: i32 = maybe_value.unwrap_or_default();
    println!("None.unwrap_or_default() = {}", value);

    // unwrap_or_else - compute default lazily
    let value = maybe_value.unwrap_or_else(|| {
        println!("  Computing expensive default...");
        compute_default()
    });
    println!("None.unwrap_or_else(compute_default) = {}", value);

    println!("\n=== Pattern Matching Instead of unwrap ===\n");

    let result: Result<i32, &str> = Ok(42);

    // Instead of:
    // let value = result.unwrap();

    // Use match:
    match result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // Or if let:
    if let Ok(value) = result {
        println!("Got value: {}", value);
    }

    println!("\n=== map() and and_then() Instead of unwrap ===\n");

    // Instead of unwrapping and then processing:
    // let value = some_option.unwrap();
    // let result = process(value);

    // Use map:
    let numbers = Some(vec![1, 2, 3, 4, 5]);
    let sum = numbers.map(|v| v.iter().sum::<i32>());
    println!("Sum via map: {:?}", sum);

    // Chain operations:
    let result = Some("42")
        .map(|s| s.parse::<i32>())
        .and_then(|r| r.ok())
        .map(|n| n * 2);
    println!("Chained result: {:?}", result);

    println!("\n=== expect() Best Practices ===\n");

    // Good: Explains WHY it should be present
    fn good_expect_usage() {
        let config = Some("production");
        let _env = config.expect(
            "ENVIRONMENT must be set. Set it to 'development', 'staging', or 'production'.",
        );
    }
    good_expect_usage();
    println!("Good: expect() with helpful message");

    // Bad: Just says what happened (we already know that)
    fn bad_expect_usage() {
        let config = Some("production");
        let _env = config.expect("Config is None"); // Not helpful
    }
    bad_expect_usage();
    println!("Bad: expect() with unhelpful message");

    println!("\n=== Real-World Patterns ===\n");

    // Pattern 1: Required configuration
    fn load_config() -> Option<String> {
        Some(String::from("config_data"))
    }

    let config = load_config().expect(
        "Failed to load configuration. Ensure config.toml exists in the working directory.",
    );
    println!("Config loaded: {}", config);

    // Pattern 2: Invariants in data structures
    fn get_first_item(items: &[i32]) -> i32 {
        // Document the invariant
        assert!(!items.is_empty(), "items must not be empty");
        items.first().copied().unwrap() // Safe due to assertion
    }
    println!("First item: {}", get_first_item(&[1, 2, 3]));

    // Pattern 3: Converting between types
    fn parse_required_port(s: &str) -> u16 {
        s.parse().expect(&format!(
            "Invalid port '{}'. Port must be a number between 0 and 65535.",
            s
        ))
    }
    println!("Port: {}", parse_required_port("8080"));

    println!("\n=== Summary ===\n");

    println!("Use unwrap() when:");
    println!("  - In tests and prototypes");
    println!("  - After checking is_some()/is_ok()");
    println!("  - When None/Err is logically impossible");

    println!("\nUse expect() when:");
    println!("  - You want to document why the value must exist");
    println!("  - In production code where failure is unrecoverable");

    println!("\nUse alternatives when:");
    println!("  - unwrap_or() - when you have a default value");
    println!("  - unwrap_or_default() - when Default makes sense");
    println!("  - unwrap_or_else() - when default is expensive to compute");
    println!("  - match/if let - when you need to handle errors gracefully");
}

fn compute_default() -> i32 {
    42
}
