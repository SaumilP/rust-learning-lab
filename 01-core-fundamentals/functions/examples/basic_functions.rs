// Example: Basic Function Declaration and Calling
//
// Demonstrates:
// - Function declaration
// - Parameters and return types
// - Function calls
// - Multiple parameters

fn main() {
    println!("=== Simple Functions ===\n");

    greet();
    say_goodbye();

    println!("\n=== Functions with Parameters ===\n");

    print_number(5);
    print_number(42);

    println!("\n=== Functions with Return Values ===\n");

    let result = add(3, 4);
    println!("3 + 4 = {}", result);

    let product = multiply(5, 6);
    println!("5 * 6 = {}", product);

    println!("\n=== Using Return Values ===\n");

    let x = 10;
    let y = 20;
    let sum = add(x, y);
    let diff = subtract(x, y);

    println!("{} + {} = {}", x, y, sum);
    println!("{} - {} = {}", x, y, diff);

    println!("\n=== Functions with Multiple Parameters ===\n");

    let area = rectangle_area(5.0, 10.0);
    println!("Rectangle area (5 × 10): {}", area);

    let volume = box_volume(3.0, 4.0, 5.0);
    println!("Box volume (3 × 4 × 5): {}", volume);

    println!("\n=== Function Composition ===\n");

    let a = 5;
    let b = 3;
    let doubled_sum = double(add(a, b));
    println!("double(add({}, {})) = {}", a, b, doubled_sum);

    println!("\n=== Functions Returning Different Types ===\n");

    let int_result = get_answer();
    println!("get_answer() = {}", int_result);

    let str_result = get_name();
    println!("get_name() = '{}'", str_result);

    let bool_result = is_even(4);
    println!("is_even(4) = {}", bool_result);

    println!("\n=== Early Return ===\n");

    check_age(15);
    check_age(25);
    check_age(200);
}

// No parameters, no return
fn greet() {
    println!("Hello, there!");
}

fn say_goodbye() {
    println!("Goodbye!");
}

// Parameters, no return
fn print_number(n: i32) {
    println!("The number is: {}", n);
}

// Parameters and return value
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// Floating-point parameters and return
fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

fn box_volume(length: f64, width: f64, height: f64) -> f64 {
    length * width * height
}

// Function using return value as input to another function
fn double(x: i32) -> i32 {
    x * 2
}

// Different return types
fn get_answer() -> i32 {
    42
}

fn get_name() -> &'static str {
    "Rust"
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

// Function with early return
fn check_age(age: i32) {
    println!("Checking age: {}", age);

    if age < 0 {
        println!("  Invalid age");
        return;
    }

    if age < 18 {
        println!("  Minor");
        return;
    }

    if age > 120 {
        println!("  Unrealistic age");
        return;
    }

    println!("  Adult");
}
