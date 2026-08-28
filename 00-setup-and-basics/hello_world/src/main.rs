// The classic first program in Rust
// Entry point for every Rust program is the main function

fn main() {
    // println! is a macro (note the ! symbol)
    // Macros are different from functions - they generate code at compile time
    println!("Hello, world!");

    // You can also use formatting
    let audience = "Rust";
    println!("Hello, {}!", audience);

    // Multiple arguments
    let language = "Rust";
    let year = 2024;
    println!("{} was created in 2010 and it's {}", language, year);

    // Named arguments (Rust 1.58+)
    let name = "Ferris";
    let role = "the Rust crab mascot";
    println!("My name is {name} and I'm {role}");

    // Debug formatting with {:?}
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);

    // Pretty-print with {:#?}
    println!("Numbers (pretty):\n{:#?}", numbers);
}

/*
To run this program:
    cargo run

To build (compile) without running:
    cargo build

To check for errors without building:
    cargo check
*/
