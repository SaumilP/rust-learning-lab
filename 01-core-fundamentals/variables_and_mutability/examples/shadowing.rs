// Example: Variable Shadowing
//
// Demonstrates:
// - Shadowing with let keyword
// - Type changes through shadowing
// - Scope-based shadowing
// - Difference between shadowing and mutation

fn main() {
    println!("=== Simple Shadowing ===\n");

    let x = 5;
    println!("x = {}", x);

    let x = x + 1; // Shadow x
    println!("x after shadowing = {}", x);

    {
        let x = x * 2; // Shadow x in inner scope
        println!("x in inner scope = {}", x);
    }

    println!("x after inner scope = {}", x);

    println!("\n=== Type Change Through Shadowing ===\n");

    let spaces = "   ";
    println!("spaces (String): '{}'", spaces);

    let spaces = spaces.len(); // Shadow: change from &str to usize
    println!("spaces (usize): {}", spaces);

    println!("\n=== Shadowing vs Mutation ===\n");

    let mut number = 5;
    println!("number (mutable): {}", number);
    number = number + 1;
    println!("number after mutation: {}", number);

    let number = "not a number"; // Can't change type with mutation
    println!("number (shadowed to string): '{}'", number);

    println!("\n=== Multiple Shadows ===\n");

    let value = "10";
    println!("value: '{}' (type: &str)", value);

    let value = value.parse::<i32>().unwrap_or(0);
    println!("value: {} (type: i32)", value);

    let value = value * 2;
    println!("value: {} (doubled)", value);

    let value = format!("The number is {}", value);
    println!("value: '{}' (type: String)", value);

    println!("\n=== Shadowing in Loop ===\n");

    let result = "5";
    println!("Input: '{}'", result);

    // Parse string to integer through shadowing
    let result = match result.parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0,
    };
    println!("Parsed: {} (type: i32)", result);

    // Shadow to string representation
    let result = format!("Result: {}", result);
    println!("Final: '{}'", result);

    println!("\n=== Shadowing Pattern: Transform Data ===\n");

    let input = "  hello world  ";
    println!("Original: '{}'", input);

    let input = input.trim();
    println!("After trim: '{}'", input);

    let input = input.to_uppercase();
    println!("After uppercase: '{}'", input);

    let input = input.split_whitespace().collect::<Vec<_>>();
    println!("After split: {:?}", input);

    println!("\n=== Shadowing with Closure ===\n");

    let x = 5;
    println!("x = {}", x);

    let add_one = |n: i32| n + 1;
    let x = add_one(x); // Shadow x with result
    println!("x after closure: {}", x);
}
