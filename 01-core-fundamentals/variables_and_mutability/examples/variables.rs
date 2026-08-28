// Example: Basic Variable Declaration and Mutability
//
// Demonstrates:
// - Variable declaration with let
// - Immutability by default
// - Mutable variables with mut
// - Type inference and explicit types

fn main() {
    println!("=== Basic Variables ===\n");

    // Immutable variable
    let x = 5;
    println!("x = {}", x);
    // x = 6;  // Error: cannot assign twice to immutable variable

    // Mutable variable
    let mut y = 10;
    println!("y = {}", y);
    y = 20; // OK: y is mutable
    println!("y after mutation = {}", y);

    println!("\n=== Type Inference ===\n");

    // Type inference
    let a = 42; // Inferred as i32
    let b = 3.14; // Inferred as f64
    let c = true; // Inferred as bool
    let d = 'Z'; // Inferred as char

    println!("a: {} (type: i32)", a);
    println!("b: {} (type: f64)", b);
    println!("c: {} (type: bool)", c);
    println!("d: {} (type: char)", d);

    println!("\n=== Explicit Type Annotation ===\n");

    // Explicit type annotation
    let count: u32 = 100;
    let temperature: f32 = 98.6;
    let is_valid: bool = true;

    println!("count: {} (type: u32)", count);
    println!("temperature: {} (type: f32)", temperature);
    println!("is_valid: {} (type: bool)", is_valid);

    println!("\n=== Variable Scope ===\n");

    let outer = "I'm outer";
    println!("outer: {}", outer);

    {
        let inner = "I'm inner";
        println!("inner: {}", inner);
        println!("outer from inner scope: {}", outer);
    }

    // println!("{}", inner);  // Error: inner is out of scope
    println!("outer after inner scope: {}", outer);

    println!("\n=== Declaring Multiple Variables ===\n");

    let (x1, x2, x3) = (10, 20, 30);
    println!("x1: {}, x2: {}, x3: {}", x1, x2, x3);

    let (mut m1, mut m2) = (1, 2);
    println!("before mutation: m1 = {}, m2 = {}", m1, m2);
    m1 = 10;
    m2 = 20;
    println!("m1: {}, m2: {} (mutable)", m1, m2);

    println!("\n=== Variables with Different Numeric Types ===\n");

    let int_var: i32 = -42;
    let unsigned: u32 = 42;
    let float_var: f64 = 3.14159;
    let small_int: i8 = 127; // Range: -128 to 127

    println!("int_var: {} (i32)", int_var);
    println!("unsigned: {} (u32)", unsigned);
    println!("float_var: {} (f64)", float_var);
    println!("small_int: {} (i8)", small_int);
}
