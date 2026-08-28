// Example: Constants vs Variables
//
// Demonstrates:
// - Constant declaration with const
// - Differences between const and let
// - Compile-time vs runtime values
// - Constant naming conventions

// Constants are evaluated at compile time
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;
const DAYS_IN_WEEK: u8 = 7;

// Can also define constants in functions
fn main() {
    println!("=== Constants ===\n");

    println!("MAX_POINTS: {}", MAX_POINTS);
    println!("PI: {}", PI);
    println!("DAYS_IN_WEEK: {}", DAYS_IN_WEEK);

    // Constants CANNOT be modified
    // MAX_POINTS = 50000;  // Error: cannot assign to const

    println!("\n=== Const vs Let ===\n");

    // let (runtime value)
    let variable = 10;
    println!("variable: {}", variable);
    // variable = 20;  // Error: cannot assign twice to immutable variable

    // const (compile-time value)
    const CONSTANT: i32 = 20;
    println!("CONSTANT: {}", CONSTANT);
    // CONSTANT = 30;  // Error: cannot assign to const

    println!("\n=== Const in Conditional ===\n");

    if MAX_POINTS > 50000 {
        println!("MAX_POINTS exceeds 50000");
    }

    if PI > 3.0 {
        println!("PI is greater than 3");
    }

    println!("\n=== Using Constants in Calculations ===\n");

    let circle_area = PI * 5.0 * 5.0; // Area = πr²
    println!("Circle area (r=5): {}", circle_area);

    let weeks_in_year = 365 / DAYS_IN_WEEK as u32;
    println!("Weeks in year: ~{}", weeks_in_year);

    println!("\n=== Constants with Units ===\n");

    const EARTH_GRAVITY: f64 = 9.81; // m/s²
    const SPEED_OF_LIGHT: u64 = 299_792_458; // m/s
                                             // This value is larger than u64::MAX, so use Rust's wider u128 type.
    const AVOGADRO_NUMBER: u128 = 602_214_076_000_000_000_000_000;

    println!("Earth gravity: {} m/s²", EARTH_GRAVITY);
    println!("Speed of light: {} m/s", SPEED_OF_LIGHT);
    println!("Avogadro's number: {}", AVOGADRO_NUMBER);

    println!("\n=== Constants at Module Level ===\n");

    println!("Config timeout: {} seconds", CONFIG_TIMEOUT);
    println!("Config max retries: {}", CONFIG_MAX_RETRIES);

    println!("\n=== Shadowing Constants ===\n");

    const X: i32 = 10;
    println!("X (const): {}", X);

    let x = 20; // Shadow with different binding
    println!("x (let): {}", x);

    // const X = 30;  // Error: can't redeclare const

    println!("\n=== Constants in Arrays ===\n");

    let fibonacci: [u32; 5] = [1, 1, 2, 3, 5];
    println!("Fibonacci sequence: {:?}", fibonacci);
    println!("First 5 Fibonacci numbers, max count: {}", fibonacci.len());

    println!("\n=== Reusing Module Constants in a Function ===\n");
    example_with_constants();
}

// Module-level constants
const CONFIG_TIMEOUT: u32 = 30;
const CONFIG_MAX_RETRIES: u32 = 3;

fn example_with_constants() {
    println!("Using module constants:");
    println!("Timeout: {}s", CONFIG_TIMEOUT);
    println!("Retries: {}", CONFIG_MAX_RETRIES);
}
