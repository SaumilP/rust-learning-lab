// Example: Option and Result Types
//
// Demonstrates:
// - Option<T> (Some/None)
// - Result<T, E> (Ok/Err)
// - Pattern matching
// - unwrap vs unwrap_or
// - Error propagation with ?

fn main() {
    println!("=== Option<T> ===\n");

    let some_value: Option<i32> = Some(5);
    let no_value: Option<i32> = None;

    println!("Some(5): {:?}", some_value);
    println!("None: {:?}", no_value);

    println!("\n=== Matching on Option ===\n");

    match some_value {
        Some(value) => println!("Got value: {}", value),
        None => println!("No value"),
    }

    match no_value {
        Some(value) => println!("Got value: {}", value),
        None => println!("No value"),
    }

    println!("\n=== Option Methods ===\n");

    let opt = Some(10);
    println!("opt.is_some(): {}", opt.is_some());
    println!("opt.is_none(): {}", opt.is_none());
    println!("opt.unwrap_or(0): {}", opt.unwrap_or(0));

    let empty: Option<i32> = None;
    println!("empty.unwrap_or(0): {}", empty.unwrap_or(0));

    println!("\n=== if let with Option ===\n");

    let maybe_value = Some(42);
    if let Some(value) = maybe_value {
        println!("Found: {}", value);
    }

    if let Some(value) = None::<i32> {
        println!("Found: {}", value);
    } else {
        println!("No value");
    }

    println!("\n=== Finding in Collections ===\n");

    let v = vec![1, 2, 3, 4, 5];
    let found = v.iter().find(|&&x| x > 3);
    println!("Vec: {:?}", v);
    println!("find(|x| x > 3): {:?}", found);

    if let Some(value) = found {
        println!("Found value: {}", value);
    }

    println!("\n=== Result<T, E> ===\n");

    let ok_value: Result<i32, String> = Ok(42);
    let err_value: Result<i32, String> = Err("Something went wrong".to_string());

    println!("Ok(42): {:?}", ok_value);
    println!("Err: {:?}", err_value);

    println!("\n=== Matching on Result ===\n");

    match ok_value {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match err_value {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Parse Example ===\n");

    let num_str = "42";
    let parsed = num_str.parse::<i32>();
    match parsed {
        Ok(num) => println!("Parsed '{}': {}", num_str, num),
        Err(e) => println!("Parse error: {}", e),
    }

    let bad_str = "abc";
    match bad_str.parse::<i32>() {
        Ok(num) => println!("Parsed: {}", num),
        Err(_) => println!("Failed to parse '{}'", bad_str),
    }

    println!("\n=== Result Methods ===\n");

    let ok_val: Result<i32, &str> = Ok(10);
    println!("is_ok: {}", ok_val.is_ok());
    println!("is_err: {}", ok_val.is_err());
    println!("unwrap_or(0): {}", ok_val.unwrap_or(0));

    let err_val: Result<i32, &str> = Err("error");
    println!("is_ok: {}", err_val.is_ok());
    println!("is_err: {}", err_val.is_err());
    println!("unwrap_or(0): {}", err_val.unwrap_or(0));

    println!("\n=== if let with Result ===\n");

    if let Ok(value) = ok_val {
        println!("Got ok value: {}", value);
    }

    if let Err(e) = err_val {
        println!("Got error: {}", e);
    }

    println!("\n=== Map and And_Then ===\n");

    let value: Option<i32> = Some(5);
    let doubled = value.map(|x| x * 2);
    println!("Some(5).map(|x| x * 2): {:?}", doubled);

    let no_value: Option<i32> = None;
    let result = no_value.map(|x| x * 2);
    println!("None.map(|x| x * 2): {:?}", result);

    println!("\n=== Chaining Operations ===\n");

    let numbers = vec!["1", "2", "3"];
    for num_str in numbers {
        match num_str.parse::<i32>() {
            Ok(num) => println!("'{}' => {}", num_str, num),
            Err(e) => println!("'{}' => Error: {}", num_str, e),
        }
    }

    println!("\n=== Vector Operations Return Option ===\n");

    let v = vec![10, 20, 30];
    println!("Vector: {:?}", v);
    println!("v.get(1): {:?}", v.get(1));
    println!("v.get(10): {:?}", v.get(10));
    println!("v.first(): {:?}", v.first());
    println!("v.last(): {:?}", v.last());

    println!("\n=== Custom Error Function ===\n");

    fn validate_age(age: u32) -> Result<(), String> {
        if age < 18 {
            Err("Too young".to_string())
        } else if age > 150 {
            Err("Unrealistic age".to_string())
        } else {
            Ok(())
        }
    }

    match validate_age(25) {
        Ok(()) => println!("Age 25: Valid"),
        Err(e) => println!("Age 25: {}", e),
    }

    match validate_age(15) {
        Ok(()) => println!("Age 15: Valid"),
        Err(e) => println!("Age 15: {}", e),
    }
}
