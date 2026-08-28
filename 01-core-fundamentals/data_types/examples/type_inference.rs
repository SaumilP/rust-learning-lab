// Example: Type Inference in Rust
//
// Demonstrates:
// - Automatic type inference with `let`
// - When to use explicit type annotations
// - Inference from context and usage
// - Generic type inference

fn main() {
    println!("=== Basic Type Inference ===\n");

    // Rust infers types from values
    let integer = 42; // Inferred as i32 (default integer type)
    let float = 3.14; // Inferred as f64 (default float type)
    let boolean = true; // Inferred as bool
    let character = 'R'; // Inferred as char
    let string_slice = "hello"; // Inferred as &str

    println!("integer: {} (inferred i32)", integer);
    println!("float: {} (inferred f64)", float);
    println!("boolean: {} (inferred bool)", boolean);
    println!("character: {} (inferred char)", character);
    println!("string_slice: {} (inferred &str)", string_slice);

    println!("\n=== Inference from Assignment ===\n");

    // Type is inferred from what value is assigned
    let message = String::from("Hello, Rust!"); // Inferred as String
    let numbers = vec![1, 2, 3, 4, 5]; // Inferred as Vec<i32>
    let pair = (42, "meaning"); // Inferred as (i32, &str)

    println!("message: {} (String)", message);
    println!("numbers: {:?} (Vec<i32>)", numbers);
    println!("pair: {:?} ((i32, &str))", pair);

    println!("\n=== Inference from Usage ===\n");

    // The compiler looks at how a variable is used to determine its type
    let mut v = Vec::new(); // The element type is inferred from the next line.
    v.push(42); // The compiler now knows this is Vec<i32>.
    println!("Inferred from use: {:?} (Vec<i32>)", v);

    // With type annotation since we don't use it
    let _empty_vec: Vec<i32> = Vec::new();
    println!("Created empty Vec<i32>");

    // Inferred from method call
    let parsed = "42".parse::<i32>().unwrap();
    println!("Parsed value: {} (i32 specified in parse)", parsed);

    // Turbofish operator ::<> specifies type
    let collected: Vec<_> = (1..5).collect();
    println!("Collected range: {:?}", collected);

    println!("\n=== Inference from Function Return ===\n");

    let result = calculate(); // Type inferred from function signature
    println!("calculate() returned: {} (f64 from signature)", result);

    let greeting = greet("World"); // Inferred as String
    println!("greet() returned: '{}'", greeting);

    println!("\n=== When Type Annotation is Required ===\n");

    // 1. When multiple types are possible
    let explicit_u8: u8 = 42;
    let explicit_i64: i64 = 42;
    println!(
        "Same value, different types: {} (u8), {} (i64)",
        explicit_u8, explicit_i64
    );

    // 2. When parsing strings
    let parsed_int: i32 = "100".parse().unwrap();
    let parsed_float: f64 = "3.14".parse().unwrap();
    println!("Parsed: {} (i32), {} (f64)", parsed_int, parsed_float);

    // 3. With collect() when type isn't obvious
    let chars: Vec<char> = "hello".chars().collect();
    println!("Collected chars: {:?}", chars);

    println!("\n=== Partial Type Inference ===\n");

    // Use _ for partial inference
    let numbers: Vec<_> = vec![1, 2, 3]; // Element type inferred
    println!("numbers: {:?} (Vec<_> where _ = i32)", numbers);

    let result: Result<_, std::num::ParseIntError> = "42".parse::<i32>();
    println!("Parse result: {:?}", result);

    println!("\n=== Inference with Closures ===\n");

    // Closure parameter and return types are inferred
    let double = |x| x * 2; // x and return type inferred from usage
    let result = double(21); // Now inferred as i32 -> i32
    println!("double(21) = {}", result);

    // Multiple uses lock in the type
    let add = |a, b| a + b;
    let sum: i32 = add(5, 10);
    // let sum2: f64 = add(5.0, 10.0);  // Error! Type already inferred as i32
    println!("add(5, 10) = {}", sum);

    println!("\n=== Inference with Generics ===\n");

    // Generic functions can have types inferred
    let v1 = create_pair(1, 2); // Pair<i32>
    let v2 = create_pair("a", "b"); // Pair<&str>
    let v3 = create_pair(1.0, 2.0); // Pair<f64>

    println!("Pairs: {:?}, {:?}, {:?}", v1, v2, v3);

    // Or specify explicitly with turbofish
    let v4 = create_pair::<u8>(10, 20);
    println!("Explicit pair: {:?}", v4);

    println!("\n=== Default Type Parameters ===\n");

    // i32 is default for integer literals
    let default_int = 42; // i32

    // f64 is default for float literals
    let default_float = 3.14; // f64

    // Use suffix for other types
    let specific_u8 = 42u8;
    let specific_i64 = 42i64;
    let specific_f32 = 3.14f32;

    println!("default_int: {}", default_int);
    println!("default_float: {}", default_float);
    println!("specific_u8: {}", specific_u8);
    println!("specific_i64: {}", specific_i64);
    println!("specific_f32: {}", specific_f32);

    println!("\n=== Type Inference in Match ===\n");

    let number = 42;
    let description = match number {
        0 => "zero",
        1..=10 => "small",
        11..=100 => "medium",
        _ => "large",
    };
    println!("{} is {}", number, description);

    // Return type is inferred from arms
    let result: i32 = match Some(5) {
        Some(x) => x * 2,
        None => 0,
    };
    println!("Match result: {}", result);
}

/// Function with explicit return type
fn calculate() -> f64 {
    42.0 * 3.14
}

/// Function returning String
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Generic function - types inferred at call site
#[derive(Debug)]
struct Pair<T>(T, T);

fn create_pair<T>(a: T, b: T) -> Pair<T> {
    Pair(a, b)
}
