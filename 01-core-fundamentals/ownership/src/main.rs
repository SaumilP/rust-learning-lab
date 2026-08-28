fn main() {
    println!("=== Ownership in Rust ===\n");
    println!("Rust's ownership system is what makes it unique!");
    println!("It provides memory safety without garbage collection.\n");

    // 1. OWNERSHIP RULES
    println!("1. THE THREE OWNERSHIP RULES:\n");
    println!("   Rule 1: Each value has ONE owner");
    println!("   Rule 2: There can only be ONE owner at a time");
    println!("   Rule 3: When owner goes out of scope, value is dropped\n");

    // 2. VARIABLE SCOPE
    println!("2. VARIABLE SCOPE:\n");
    {
        let s = "hello"; // s is valid from this point
        println!("   Inside scope: {}", s);
    } // s goes out of scope and is dropped here
      // println!("{}", s);  // ERROR! s is no longer valid

    println!("   ✓ Variables are valid until end of scope\n");

    // 3. STRING TYPE (Ownership in Action)
    println!("3. STRING vs &str:\n");

    // String literal (&str) - immutable, fixed size, stack-allocated
    let s1 = "hello";
    println!("   String literal (&str): {}", s1);

    // String - mutable, growable, heap-allocated
    let mut s2 = String::from("hello");
    s2.push_str(", world!");
    println!("   String (owned): {}", s2);
    println!("   ✓ String owns heap data, &str is a reference\n");

    // 4. MOVE SEMANTICS
    println!("4. MOVE SEMANTICS:\n");

    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2 (not copied!)

    // println!("{}", s1);  // ERROR! s1 is no longer valid
    println!("   s2 = {} (s1 was moved to s2)", s2);
    println!("   ✓ s1 is no longer valid after move");
    println!("   ✓ Prevents double-free errors\n");

    // 5. CLONE - Deep Copy
    println!("5. CLONE (Deep Copy):\n");

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Explicit deep copy

    println!("   s1 = {}, s2 = {}", s1, s2);
    println!("   ✓ Both s1 and s2 are valid");
    println!("   ✓ clone() creates a deep copy on the heap\n");

    // 6. COPY TRAIT (Stack-Only Data)
    println!("6. COPY TRAIT (Stack-Only Data):\n");

    let x = 5;
    let y = x; // x is copied (not moved!)

    println!("   x = {}, y = {}", x, y);
    println!("   ✓ Simple types implement Copy trait");
    println!("   ✓ Types: integers, floats, bool, char, tuples of Copy types\n");

    // 7. OWNERSHIP AND FUNCTIONS
    println!("7. OWNERSHIP AND FUNCTIONS:\n");

    let s = String::from("hello");
    takes_ownership(s); // s is moved into function
                        // println!("{}", s);  // ERROR! s is no longer valid

    let x = 5;
    makes_copy(x); // x is copied (not moved)
    println!("   x is still valid: {}", x); // OK! x is still valid

    println!("   ✓ Passing to function transfers ownership\n");

    // 8. RETURN VALUES AND OWNERSHIP
    println!("8. RETURN VALUES TRANSFER OWNERSHIP:\n");

    let s1 = gives_ownership(); // Function returns ownership
    println!("   s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 moved, ownership returned
                                       // println!("{}", s2);  // ERROR! s2 was moved
    println!("   s3 = {}", s3);

    println!("   ✓ Return values transfer ownership\n");

    // 9. RETURNING MULTIPLE VALUES
    println!("9. RETURNING MULTIPLE VALUES:\n");

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("   String: {}, Length: {}", s2, len);
    println!("   ✓ Use tuples to return ownership + data\n");

    // 10. MEMORY MANAGEMENT COMPARISON
    println!("10. MEMORY MANAGEMENT COMPARISON:\n");
    println!("   C/C++:  Manual (malloc/free) - Easy to leak or double-free");
    println!("   Java:   Garbage Collector - Runtime overhead, unpredictable pauses");
    println!("   Rust:   Ownership System - Compile-time, zero-cost, deterministic");
    println!("   ✓ Rust prevents memory leaks and double-frees at compile time!\n");

    // 11. DROP TRAIT
    println!("11. DROP TRAIT (Automatic Cleanup):\n");

    {
        let _s = String::from("hello");
        // drop is called automatically when _s goes out of scope
        println!("   String created");
    } // _s.drop() called here automatically
    println!("   String dropped (memory freed)");
    println!("   ✓ Drop trait provides deterministic cleanup\n");

    // 12. PRACTICAL EXAMPLE
    println!("12. PRACTICAL EXAMPLE:\n");

    let mut data = vec![1, 2, 3, 4, 5];
    println!("   Original data: {:?}", data);

    process_data(data.clone()); // Clone to keep ownership
    println!("   After process_data: {:?}", data);

    data = transform_data(data); // Transfer ownership
    println!("   After transform_data: {:?}", data);

    println!("   ✓ Choose between clone() and move based on needs\n");
}

// Takes ownership of String
fn takes_ownership(s: String) {
    println!("   takes_ownership: {}", s);
} // s is dropped here

// Makes a copy (Copy trait)
fn makes_copy(x: i32) {
    println!("   makes_copy: {}", x);
} // x goes out of scope, but nothing special happens (Copy trait)

// Gives ownership of return value
fn gives_ownership() -> String {
    String::from("yours")
}

// Takes and gives back ownership
fn takes_and_gives_back(s: String) -> String {
    s // Returns ownership to caller
}

// Returns tuple (ownership + data)
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// Processes data (takes ownership, doesn't return)
fn process_data(data: Vec<i32>) {
    println!("   Processing: {:?}", data);
    // data is dropped here
}

// Transforms data (takes ownership, returns new)
fn transform_data(mut data: Vec<i32>) -> Vec<i32> {
    data.iter_mut().for_each(|x| *x *= 2);
    data // Return ownership
}

/*
Ownership Summary:
==================

KEY CONCEPTS:
1. Each value has exactly ONE owner
2. When owner goes out of scope, value is dropped
3. Ownership can be transferred (moved)
4. Some types implement Copy (stack types)
5. clone() creates deep copy (expensive)
6. Functions can take/return ownership

BENEFITS:
- No garbage collector needed
- No manual memory management
- Memory safety guaranteed at compile time
- No data races
- Deterministic destruction

COMMON PATTERNS:
- Move when you don't need the original
- Clone when you need both copies
- Use references (borrowing) to avoid moves
- Return ownership when caller needs it

WHY IT MATTERS:
- Prevents use-after-free bugs
- Prevents double-free errors
- Prevents memory leaks
- Zero runtime cost
- Compile-time guarantees

Run this:
    cargo run

Experiment:
    - Try using moved values (uncomment error lines)
    - Compare move vs clone performance
    - Write functions that take/return ownership
    - Try moving vs copying different types

Next:
    - Learn about BORROWING (references)
    - Understand LIFETIMES
    - Master the borrow checker
*/
