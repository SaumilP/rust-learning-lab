// Example: Option Type - Some and None Variants
//
// Demonstrates:
// - Creating Option values with Some and None
// - Pattern matching on Options
// - Option methods: is_some, is_none, unwrap, etc.
// - Using Option for nullable values

fn main() {
    println!("=== Creating Option Values ===\n");

    // Option can be Some(value) or None
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    println!("Some(42) = {:?}", some_number);
    println!("None = {:?}", no_number);

    // Type inference with Some
    let some_string = Some(String::from("hello"));
    println!("Some(String) = {:?}", some_string);

    // None requires type annotation or inference
    let none_value: Option<String> = None;
    println!("None::<String> = {:?}", none_value);

    println!("\n=== Pattern Matching on Option ===\n");

    fn describe_option(opt: Option<i32>) {
        match opt {
            Some(value) => println!("Got value: {}", value),
            None => println!("No value present"),
        }
    }

    describe_option(Some(100));
    describe_option(None);

    println!("\n=== is_some() and is_none() ===\n");

    let has_value = Some(10);
    let no_value: Option<i32> = None;

    println!("Some(10).is_some() = {}", has_value.is_some());
    println!("Some(10).is_none() = {}", has_value.is_none());
    println!("None.is_some() = {}", no_value.is_some());
    println!("None.is_none() = {}", no_value.is_none());

    // Use in conditions
    if has_value.is_some() {
        println!("Value is present!");
    }

    println!("\n=== if let Pattern ===\n");

    let maybe_name = Some("Alice");

    // Cleaner than full match when only checking Some
    if let Some(name) = maybe_name {
        println!("Hello, {}!", name);
    }

    let no_name: Option<&str> = None;
    if let Some(name) = no_name {
        println!("Hello, {}!", name);
    } else {
        println!("No name provided");
    }

    println!("\n=== while let Pattern ===\n");

    let mut stack = vec![1, 2, 3];
    println!("Stack: {:?}", stack);

    // pop() returns Option
    println!("Popping:");
    while let Some(value) = stack.pop() {
        println!("  Popped: {}", value);
    }

    println!("\n=== Option Methods: Extracting Values ===\n");

    let some_value = Some(42);

    // unwrap() - panics if None
    let value = some_value.unwrap();
    println!("Some(42).unwrap() = {}", value);

    // unwrap_or() - provides default
    let none_value: Option<i32> = None;
    let value = none_value.unwrap_or(0);
    println!("None.unwrap_or(0) = {}", value);

    // unwrap_or_default() - uses Default trait
    let value = none_value.unwrap_or_default();
    println!("None.unwrap_or_default() = {}", value);

    // unwrap_or_else() - lazy default with closure
    let value = none_value.unwrap_or_else(|| {
        println!("  Computing default...");
        100
    });
    println!("None.unwrap_or_else(|| 100) = {}", value);

    println!("\n=== expect() - unwrap with Message ===\n");

    let config_value = Some("production");
    let value = config_value.expect("Config value must be set");
    println!("Config: {}", value);

    // This would panic with custom message:
    // let missing: Option<&str> = None;
    // missing.expect("Configuration is required");

    println!("\n=== map() - Transform Option ===\n");

    let maybe_number = Some(5);

    // Transform the inner value if Some
    let doubled = maybe_number.map(|x| x * 2);
    println!("Some(5).map(|x| x * 2) = {:?}", doubled);

    let squared = maybe_number.map(|x| x * x);
    println!("Some(5).map(|x| x * x) = {:?}", squared);

    // map on None returns None
    let no_number: Option<i32> = None;
    let result = no_number.map(|x| x * 2);
    println!("None.map(|x| x * 2) = {:?}", result);

    // Chain maps
    let result = Some("42")
        .map(|s| s.parse::<i32>()) // Option<Result<i32, _>>
        .map(|r| r.ok()) // Option<Option<i32>>
        .flatten(); // Option<i32>
    println!("Chained map result: {:?}", result);

    println!("\n=== and_then() - Flat Map ===\n");

    // and_then for operations that return Option
    fn parse_number(s: &str) -> Option<i32> {
        s.parse().ok()
    }

    fn double_if_positive(n: i32) -> Option<i32> {
        if n > 0 {
            Some(n * 2)
        } else {
            None
        }
    }

    let result = parse_number("5").and_then(double_if_positive);
    println!("parse(\"5\").and_then(double_if_positive) = {:?}", result);

    let result = parse_number("-5").and_then(double_if_positive);
    println!("parse(\"-5\").and_then(double_if_positive) = {:?}", result);

    let result = parse_number("invalid").and_then(double_if_positive);
    println!(
        "parse(\"invalid\").and_then(double_if_positive) = {:?}",
        result
    );

    println!("\n=== filter() - Conditional Option ===\n");

    let some_number = Some(10);

    let filtered = some_number.filter(|&x| x > 5);
    println!("Some(10).filter(|x| x > 5) = {:?}", filtered);

    let filtered = some_number.filter(|&x| x > 20);
    println!("Some(10).filter(|x| x > 20) = {:?}", filtered);

    println!("\n=== or() and or_else() - Alternatives ===\n");

    let primary: Option<i32> = None;
    let backup = Some(100);

    // Use backup if primary is None
    let value = primary.or(backup);
    println!("None.or(Some(100)) = {:?}", value);

    let value = Some(42).or(backup);
    println!("Some(42).or(Some(100)) = {:?}", value);

    // or_else with lazy evaluation
    let value = primary.or_else(|| {
        println!("  Computing fallback...");
        Some(999)
    });
    println!("None.or_else(...) = {:?}", value);

    println!("\n=== take() and replace() ===\n");

    let mut option = Some(42);
    println!("Initial: {:?}", option);

    // take() extracts value, leaving None
    let taken = option.take();
    println!("After take(): option = {:?}, taken = {:?}", option, taken);

    // replace() swaps value
    option = Some(10);
    let old = option.replace(20);
    println!("After replace(20): option = {:?}, old = {:?}", option, old);

    println!("\n=== Converting Option to Result ===\n");

    let some_value = Some(42);
    let none_value: Option<i32> = None;

    // ok_or() converts Option to Result
    let result: Result<i32, &str> = some_value.ok_or("No value");
    println!("Some(42).ok_or(\"No value\") = {:?}", result);

    let result: Result<i32, &str> = none_value.ok_or("No value");
    println!("None.ok_or(\"No value\") = {:?}", result);

    println!("\n=== Practical Examples ===\n");

    // Example 1: Safe division
    fn safe_divide(a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }

    println!("safe_divide(10, 2) = {:?}", safe_divide(10, 2));
    println!("safe_divide(10, 0) = {:?}", safe_divide(10, 0));

    // Example 2: Find in collection
    let numbers = vec![1, 2, 3, 4, 5];
    let found = numbers.iter().find(|&&x| x > 3);
    match found {
        Some(&n) => println!("Found number > 3: {}", n),
        None => println!("No number > 3 found"),
    }

    // Example 3: Optional struct fields
    #[derive(Debug)]
    struct User {
        name: String,
        email: Option<String>,
        phone: Option<String>,
    }

    let user = User {
        name: String::from("Alice"),
        email: Some(String::from("alice@example.com")),
        phone: None,
    };

    println!("\nUser: {}", user.name);
    if let Some(email) = &user.email {
        println!("Email: {}", email);
    }
    println!(
        "Phone: {}",
        user.phone.as_ref().unwrap_or(&String::from("Not provided"))
    );
}
