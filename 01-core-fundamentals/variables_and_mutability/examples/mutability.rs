// Example: Using the `mut` Keyword for Mutability
//
// Demonstrates:
// - Declaring mutable variables with `mut`
// - Modifying mutable variables
// - Mutable references and their restrictions
// - Common mutability patterns

fn main() {
    println!("=== Basic Mutability ===\n");

    // Immutable by default
    let immutable_value = 42;
    println!("Immutable value: {}", immutable_value);
    // immutable_value = 100;  // Error: cannot assign twice to immutable variable

    // Mutable with `mut` keyword
    let mut mutable_value = 42;
    println!("Mutable value (initial): {}", mutable_value);
    mutable_value = 100;
    println!("Mutable value (modified): {}", mutable_value);

    println!("\n=== Mutating with Operations ===\n");

    let mut counter = 0;
    println!("counter (initial): {}", counter);

    counter += 1;
    println!("counter after += 1: {}", counter);

    counter *= 5;
    println!("counter after *= 5: {}", counter);

    counter -= 2;
    println!("counter after -= 2: {}", counter);

    println!("\n=== Mutable Collections ===\n");

    // Mutable vector
    let mut numbers = vec![1, 2, 3];
    println!("numbers (initial): {:?}", numbers);

    numbers.push(4);
    println!("numbers after push(4): {:?}", numbers);

    numbers.pop();
    println!("numbers after pop(): {:?}", numbers);

    numbers[0] = 100;
    println!("numbers after modifying index 0: {:?}", numbers);

    println!("\n=== Mutable Strings ===\n");

    let mut greeting = String::from("Hello");
    println!("greeting (initial): '{}'", greeting);

    greeting.push_str(", World!");
    println!("greeting after push_str: '{}'", greeting);

    greeting.push('!');
    println!("greeting after push: '{}'", greeting);

    greeting = greeting.to_uppercase();
    println!("greeting after to_uppercase: '{}'", greeting);

    println!("\n=== Interior Mutability Pattern ===\n");

    // Sometimes you need mutability even with immutable references
    // This is called "interior mutability" pattern
    use std::cell::RefCell;

    let data = RefCell::new(5);
    println!("data (initial): {:?}", data);

    // Borrow mutably and modify
    *data.borrow_mut() = 10;
    println!("data (after modification): {:?}", data);

    println!("\n=== Mutable Loop Variables ===\n");

    let mut sum = 0;
    let numbers_to_sum = [1, 2, 3, 4, 5];

    for number in numbers_to_sum.iter() {
        sum += number;
    }
    println!("Sum of {:?} = {}", numbers_to_sum, sum);

    // Mutable iterator
    let mut values = vec![1, 2, 3, 4, 5];
    println!("values before: {:?}", values);

    for value in values.iter_mut() {
        *value *= 2;
    }
    println!("values after doubling: {:?}", values);

    println!("\n=== Mutable Function Parameters ===\n");

    let mut value = 10;
    println!("value before add_ten: {}", value);

    add_ten(&mut value);
    println!("value after add_ten: {}", value);

    println!("\n=== Mutable Return Values ===\n");

    let result = create_mutable_string();
    println!("Created string: '{}'", result);

    println!("\n=== Conditionally Mutable ===\n");

    let mut optional_value: Option<i32> = None;
    println!("optional_value (initial): {:?}", optional_value);

    optional_value = Some(42);
    println!("optional_value (after assignment): {:?}", optional_value);

    if let Some(ref mut val) = optional_value {
        *val += 10;
    }
    println!("optional_value (after modification): {:?}", optional_value);

    println!("\n=== Swap Pattern ===\n");

    let mut a = 5;
    let mut b = 10;
    println!("Before swap: a = {}, b = {}", a, b);

    std::mem::swap(&mut a, &mut b);
    println!("After swap: a = {}, b = {}", a, b);
}

/// Demonstrates mutable reference as a parameter
fn add_ten(value: &mut i32) {
    *value += 10;
}

/// Demonstrates returning a mutable value
fn create_mutable_string() -> String {
    let mut s = String::new();
    s.push_str("Hello from function!");
    s
}

/// Example: Builder pattern using mutability
#[allow(dead_code)]
struct Config {
    timeout: u32,
    retries: u32,
}

#[allow(dead_code)]
impl Config {
    fn new() -> Config {
        Config {
            timeout: 30,
            retries: 3,
        }
    }

    fn with_timeout(mut self, timeout: u32) -> Config {
        self.timeout = timeout;
        self
    }

    fn with_retries(mut self, retries: u32) -> Config {
        self.retries = retries;
        self
    }
}
