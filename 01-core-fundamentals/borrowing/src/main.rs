fn main() {
    println!("=== Borrowing and References in Rust ===\n");
    println!("Borrowing lets you reference values WITHOUT taking ownership\n");

    // 1. THE BORROWING RULES
    println!("1. THE BORROWING RULES:\n");
    println!("   Rule 1: At any time, you can have EITHER:");
    println!("           - One mutable reference, OR");
    println!("           - Any number of immutable references");
    println!("   Rule 2: References must always be valid\n");

    // 2. IMMUTABLE REFERENCES (&T)
    println!("2. IMMUTABLE REFERENCES (&T):\n");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Borrow s1 (don't take ownership)

    println!("   String: '{}', Length: {}", s1, len);
    println!("   ✓ s1 is still valid after borrowing");
    println!("   ✓ & creates a reference (borrow)\n");

    // 3. MULTIPLE IMMUTABLE REFERENCES
    println!("3. MULTIPLE IMMUTABLE REFERENCES:\n");

    let s = String::from("hello world");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    println!("   r1: {}, r2: {}, r3: {}", r1, r2, r3);
    println!("   ✓ Multiple immutable references are OK\n");

    // 4. MUTABLE REFERENCES (&mut T)
    println!("4. MUTABLE REFERENCES (&mut T):\n");

    let mut s = String::from("hello");
    change(&mut s);  // Borrow mutably

    println!("   Changed string: {}", s);
    println!("   ✓ &mut allows modifying borrowed data\n");

    // 5. MUTABLE REFERENCE RESTRICTION
    println!("5. MUTABLE REFERENCE RESTRICTION:\n");

    let mut s = String::from("hello");

    let r1 = &mut s;
    r1.push_str(" world");
    println!("   First mut ref: {}", r1);

    // Can't have two mutable references at the same time:
    // let r2 = &mut s;  // ERROR! Cannot borrow as mutable more than once
    // println!("{} {}", r1, r2);

    // But can have another after first goes out of scope:
    let r2 = &mut s;
    r2.push_str("!");
    println!("   Second mut ref: {}", r2);

    println!("   ✓ Only ONE mutable reference at a time\n");

    // 6. MIXING REFERENCES
    println!("6. MIXING MUTABLE AND IMMUTABLE REFERENCES:\n");

    let mut s = String::from("hello");

    let r1 = &s;  // OK - immutable
    let r2 = &s;  // OK - immutable
    println!("   r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s;  // OK - previous immutable refs out of scope
    r3.push_str(" world");
    println!("   r3: {}", r3);

    // CANNOT mix if used together:
    let r1 = &s;
    // let r2 = &mut s;  // ERROR! Cannot borrow as mutable
    // println!("{} {}", r1, r2);

    println!("   ✓ Cannot have mutable ref while immutable refs exist\n");

    // 7. DANGLING REFERENCES (Prevented by Rust!)
    println!("7. DANGLING REFERENCES (Prevented!):\n");

    // This won't compile:
    // let reference_to_nothing = dangle();

    // Correct version:
    let string = no_dangle();
    println!("   Returned string: {}", string);
    println!("   ✓ Rust prevents dangling references at compile time\n");

    // 8. DEREFERENCING
    println!("8. DEREFERENCING:\n");

    let x = 5;
    let y = &x;  // Reference to x

    println!("   x = {}, y = {} (y is reference)", x, y);

    // Dereference with *
    if x == *y {
        println!("   x == *y (true)");
    }

    // Most operators auto-dereference:
    if x == *y {
        println!("   ✓ * dereferences a reference\n");
    }

    // 9. REFERENCES AS FUNCTION PARAMETERS
    println!("9. REFERENCES AS FUNCTION PARAMETERS:\n");

    let s = String::from("hello world");

    let word = first_word(&s);  // Borrow s
    println!("   First word: {}", word);
    println!("   Original string still valid: {}", s);

    println!("   ✓ Borrowing in functions doesn't take ownership\n");

    // 10. SLICE TYPE (Special Kind of Reference)
    println!("10. SLICE TYPE (References to Sequences):\n");

    let s = String::from("hello world");

    let hello = &s[0..5];   // String slice
    let world = &s[6..11];  // Another slice

    println!("   Slices: '{}' and '{}'", hello, world);

    // Array slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];  // [2, 3, 4]
    println!("   Array slice: {:?}", slice);

    println!("   ✓ Slices reference a portion of a collection\n");

    // 11. PRACTICAL EXAMPLE: Safe Data Access
    println!("11. PRACTICAL EXAMPLE: Safe Data Access:\n");

    let mut data = vec![1, 2, 3, 4, 5];

    // Read-only access (multiple readers OK)
    let sum1 = sum_vector(&data);
    let sum2 = sum_vector(&data);
    println!("   Sum1: {}, Sum2: {}", sum1, sum2);

    // Modify data (exclusive access)
    double_vector(&mut data);
    println!("   After doubling: {:?}", data);

    println!("   ✓ Borrowing enables safe concurrent reads\n");

    // 12. BORROWING PREVENTS DATA RACES
    println!("12. BORROWING PREVENTS DATA RACES:\n");
    println!("   Multiple readers: ALLOWED (immutable borrows)");
    println!("   Single writer: ALLOWED (one mutable borrow)");
    println!("   Mixed reader/writer: NOT ALLOWED");
    println!("   ✓ Eliminates entire class of concurrency bugs!\n");

    // 13. REFERENCE COUNTING (Shared Ownership)
    println!("13. WHEN YOU NEED SHARED OWNERSHIP:\n");
    println!("   Use Rc<T> for single-threaded");
    println!("   Use Arc<T> for multi-threaded");
    println!("   (Covered in smart pointers section)\n");
}

// Borrows string (immutable reference)
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope, but doesn't drop the String (doesn't own it)

// Borrows string mutably
fn change(s: &mut String) {
    s.push_str(", world");
}

// This would create a dangling reference (doesn't compile!):
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // ERROR! Returns reference to data that will be dropped
// }

// Correct: Return owned data
fn no_dangle() -> String {
    String::from("hello")
}

// Returns first word from string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    &s[..]  // Whole string if no space
}

// Sum vector (immutable borrow)
fn sum_vector(v: &Vec<i32>) -> i32 {
    v.iter().sum()
}

// Double vector values (mutable borrow)
fn double_vector(v: &mut Vec<i32>) {
    for x in v.iter_mut() {
        *x *= 2;
    }
}

/*
Borrowing Summary:
==================

REFERENCE TYPES:
- &T          - Immutable reference (shared reference)
- &mut T      - Mutable reference (exclusive reference)

BORROWING RULES (Enforced at Compile Time):
1. Multiple immutable references are OK
2. Only ONE mutable reference at a time
3. Cannot mix mutable and immutable references
4. References must always be valid

BENEFITS:
- No null pointer errors
- No dangling pointers
- No data races
- Safe concurrent reads
- Compile-time guarantees

COMMON PATTERNS:
- Use & for read-only access
- Use &mut for write access
- Return slices instead of owned data when possible
- Prefer borrowing over cloning

WHY IT MATTERS:
- Prevents entire classes of bugs
- No runtime overhead
- Enables fearless concurrency
- Makes parallelism safe

COMPARISON:
C/C++:   Raw pointers, easy to dangle/leak
Java:    Garbage collected, no control
Python:  Reference counting, cycle issues
Rust:    Compile-time checked, zero-cost

Run this:
    cargo run

Experiment:
    - Try violating borrowing rules (uncomment errors)
    - Create multiple immutable references
    - Try mixing mutable and immutable refs
    - Write functions that borrow vs take ownership

Next:
    - Learn about LIFETIMES
    - Understand lifetime annotations
    - Master advanced borrowing patterns
*/
