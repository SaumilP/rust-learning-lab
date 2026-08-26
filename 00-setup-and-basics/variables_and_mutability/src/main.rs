fn main() {
    println!("=== Variables and Mutability in Rust ===\n");

    // 1. IMMUTABLE VARIABLES (default)
    println!("1. Immutable Variables:");
    let x = 5;
    println!("   x = {}", x);
    // x = 6; // ERROR! Cannot assign twice to immutable variable
    println!("   ✓ Variables are immutable by default\n");

    // 2. MUTABLE VARIABLES
    println!("2. Mutable Variables:");
    let mut y = 5;
    println!("   y = {}", y);
    y = 6;  // This works because y is declared with 'mut'
    println!("   y = {} (after mutation)", y);
    println!("   ✓ Use 'mut' keyword to make variables mutable\n");

    // 3. CONSTANTS
    println!("3. Constants:");
    const MAX_POINTS: u32 = 100_000;
    println!("   MAX_POINTS = {}", MAX_POINTS);
    println!("   ✓ Constants are always immutable");
    println!("   ✓ Must be type annotated");
    println!("   ✓ Use SCREAMING_SNAKE_CASE\n");

    // 4. SHADOWING
    println!("4. Shadowing:");
    let z = 5;
    println!("   z = {}", z);

    let z = z + 1;  // Shadow the previous z
    println!("   z = {} (shadowed)", z);

    {
        let z = z * 2;  // Shadow in inner scope
        println!("   z = {} (inner scope)", z);
    }

    println!("   z = {} (outer scope)", z);
    println!("   ✓ Shadowing allows reusing variable names");
    println!("   ✓ Can change type when shadowing\n");

    // 5. TYPE CHANGE WITH SHADOWING
    println!("5. Type Change with Shadowing:");
    let spaces = "   ";  // string
    let spaces = spaces.len();  // number
    println!("   spaces = {} (changed from &str to usize)", spaces);
    println!("   ✓ Shadowing allows type changes\n");

    // With mut, type must stay the same:
    let mut count = "123";
    // count = count.len(); // ERROR! Expected &str, found usize

    // 6. SCOPE AND LIFETIME
    println!("6. Scope and Lifetime:");
    {
        let inner_var = 42;
        println!("   inner_var = {} (inside scope)", inner_var);
    }
    // println!("{}", inner_var); // ERROR! inner_var not available here
    println!("   ✓ Variables are dropped when they go out of scope\n");

    // 7. TYPE INFERENCE
    println!("7. Type Inference:");
    let inferred = 42;  // Rust infers i32
    let explicit: i32 = 42;  // Explicitly typed
    println!("   inferred = {}, explicit = {}", inferred, explicit);
    println!("   ✓ Rust infers types when possible\n");

    // 8. MULTIPLE ASSIGNMENTS
    println!("8. Multiple Assignments (Destructuring):");
    let (a, b, c) = (1, 2, 3);
    println!("   a = {}, b = {}, c = {}", a, b, c);
    println!("   ✓ Can destructure tuples into multiple variables\n");

    // 9. UNDERSCORE FOR UNUSED
    println!("9. Ignoring Values:");
    let _unused = "I won't be used";
    // Compiler won't warn about _unused
    println!("   ✓ Prefix with _ to suppress unused variable warnings\n");

    // 10. COMPARISON TO OTHER LANGUAGES
    println!("10. Key Differences from Other Languages:");
    println!("    - Variables are immutable by default (unlike C, Java, Python)");
    println!("    - Must explicitly opt-in to mutability");
    println!("    - No null/undefined - use Option<T> instead");
    println!("    - No garbage collector - ownership system manages memory");
}

/*
Key Takeaways:
==============
1. Variables are IMMUTABLE by default
2. Use `mut` to make them mutable
3. Constants use `const` keyword and must have types
4. Shadowing allows reusing names and changing types
5. Variables are dropped when they go out of scope
6. Rust infers types when it can

Run this:
    cargo run

Experiment:
    - Try uncommenting the error lines to see compiler messages
    - Add your own variables
    - Try shadowing with different types
*/
