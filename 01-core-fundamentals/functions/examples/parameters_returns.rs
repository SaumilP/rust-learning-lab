// Example: Function Parameters and Return Types
//
// Demonstrates:
// - Different parameter types
// - Multiple parameters
// - Return types and the unit type ()
// - Passing by value vs reference
// - Complex return types

fn main() {
    println!("=== Single Parameter Functions ===\n");

    let squared = square(5);
    println!("square(5) = {}", squared);

    let absolute = abs(-42);
    println!("abs(-42) = {}", absolute);

    println!("\n=== Multiple Parameters ===\n");

    let sum = add(10, 20);
    println!("add(10, 20) = {}", sum);

    let full_name = combine_names("John", "Doe");
    println!("combine_names(\"John\", \"Doe\") = '{}'", full_name);

    let formatted = format_coordinates(45.0, -122.0);
    println!("format_coordinates(45.0, -122.0) = '{}'", formatted);

    println!("\n=== Different Parameter Types ===\n");

    let message = create_message("Hello", 3);
    println!("create_message(\"Hello\", 3) = '{}'", message);

    let grade = letter_grade(85.5);
    println!("letter_grade(85.5) = '{}'", grade);

    println!("\n=== Returning Unit Type () ===\n");

    // Functions that don't return a value implicitly return ()
    print_separator();
    log_message("This is a log entry");

    println!("\n=== Returning Tuples ===\n");

    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 / 5 = {} remainder {}", quotient, remainder);

    let (min, max) = min_max(42, 17);
    println!("min_max(42, 17) = ({}, {})", min, max);

    println!("\n=== Returning Options ===\n");

    match safe_divide(10, 2) {
        Some(result) => println!("10 / 2 = {}", result),
        None => println!("Division by zero!"),
    }

    match safe_divide(10, 0) {
        Some(result) => println!("10 / 0 = {}", result),
        None => println!("10 / 0 = Division by zero!"),
    }

    println!("\n=== Returning Results ===\n");

    match parse_and_double("21") {
        Ok(value) => println!("parse_and_double(\"21\") = {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match parse_and_double("not a number") {
        Ok(value) => println!("parse_and_double(\"not a number\") = {}", value),
        Err(e) => println!("Error parsing: {}", e),
    }

    println!("\n=== Pass by Reference ===\n");

    let name = String::from("Alice");
    let greeting = greet_by_ref(&name);
    println!("greet_by_ref(&\"{}\") = '{}'", name, greeting);
    // name is still valid here because we only borrowed it

    let numbers = vec![1, 2, 3, 4, 5];
    let sum = sum_slice(&numbers);
    println!("sum_slice({:?}) = {}", numbers, sum);
    // numbers is still valid

    println!("\n=== Mutable References ===\n");

    let mut value = 10;
    println!("value before: {}", value);
    increment(&mut value);
    println!("value after increment: {}", value);

    let mut data = vec![1, 2, 3];
    println!("data before: {:?}", data);
    append_sum(&mut data);
    println!("data after append_sum: {:?}", data);

    println!("\n=== Returning Owned Values ===\n");

    let owned_string = create_greeting("World");
    println!("create_greeting(\"World\") = '{}'", owned_string);

    let owned_vec = create_range(1, 5);
    println!("create_range(1, 5) = {:?}", owned_vec);

    println!("\n=== Generic Parameter Types ===\n");

    println!("largest of 5, 10: {}", largest(5, 10));
    println!("largest of 'a', 'z': {}", largest('a', 'z'));

    println!("\n=== Functions with Array Parameters ===\n");

    let array = [1, 2, 3, 4, 5];
    let avg = average(&array);
    println!("average({:?}) = {}", array, avg);
}

// --- Single parameter functions ---

fn square(x: i32) -> i32 {
    x * x
}

fn abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

// --- Multiple parameters ---

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn combine_names(first: &str, last: &str) -> String {
    format!("{} {}", first, last)
}

fn format_coordinates(lat: f64, lon: f64) -> String {
    format!("({:.2}, {:.2})", lat, lon)
}

// --- Mixed parameter types ---

fn create_message(text: &str, repeat: usize) -> String {
    text.repeat(repeat)
}

fn letter_grade(score: f64) -> char {
    match score as i32 {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    }
}

// --- Unit type return ---

fn print_separator() {
    println!("-------------------");
}

fn log_message(msg: &str) {
    println!("[LOG] {}", msg);
}

// --- Returning tuples ---

fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

// --- Returning Option ---

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

// --- Returning Result ---

fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let number: i32 = s.parse()?;
    Ok(number * 2)
}

// --- Pass by reference ---

fn greet_by_ref(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn sum_slice(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

// --- Mutable references ---

fn increment(value: &mut i32) {
    *value += 1;
}

fn append_sum(data: &mut Vec<i32>) {
    let sum: i32 = data.iter().sum();
    data.push(sum);
}

// --- Returning owned values ---

fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn create_range(start: i32, end: i32) -> Vec<i32> {
    (start..=end).collect()
}

// --- Generic parameters ---

fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// --- Array parameters ---

fn average(numbers: &[i32]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}
