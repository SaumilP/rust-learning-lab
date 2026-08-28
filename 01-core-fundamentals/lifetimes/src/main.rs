fn main() {
    println!("=== Lifetimes in Rust ===\n");
    println!("Lifetimes ensure references are always valid\n");

    // 1. WHAT ARE LIFETIMES?
    println!("1. WHAT ARE LIFETIMES?\n");
    println!("   - Lifetimes are Rust's way of tracking how long references are valid");
    println!("   - Every reference has a lifetime");
    println!("   - Usually inferred by the compiler");
    println!("   - Sometimes need explicit annotations\n");

    // 2. LIFETIME SCOPE
    println!("2. LIFETIME SCOPE:\n");

    // This version would fail because `r` outlives `x`:
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r); // ERROR: x does not live long enough

    println!("   ✓ Reference cannot outlive the data it points to");
    println!("   ✓ The borrow checker prevents dangling references\n");

    // 3. VALID LIFETIMES
    println!("3. VALID LIFETIMES:\n");

    let x = 5; // ----------+-- 'a
    let r = &x; // --+-- 'b  |
                //   |       |
    println!("   r: {}", r); //   |       |
                             // --+       |
                             // ----------+

    println!("   ✓ Reference r's lifetime 'b is contained in x's lifetime 'a\n");

    // 4. LIFETIME ANNOTATIONS IN FUNCTIONS
    println!("4. LIFETIME ANNOTATIONS IN FUNCTIONS:\n");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1, string2);
    println!("   Longest: {}", result);

    println!("   ✓ Lifetime annotations tell Rust how lifetimes relate\n");

    // 5. LIFETIME ANNOTATION SYNTAX
    println!("5. LIFETIME ANNOTATION SYNTAX:\n");
    println!("   &i32        - a reference");
    println!("   &'a i32     - a reference with explicit lifetime 'a");
    println!("   &'a mut i32 - a mutable reference with explicit lifetime 'a\n");

    // 6. MULTIPLE LIFETIME PARAMETERS
    println!("6. MULTIPLE LIFETIME PARAMETERS:\n");

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("   Result: {}", result);
    }
    // result can't be used here because string2 is dropped

    println!("   ✓ Return value's lifetime is tied to shortest input lifetime\n");

    // 7. LIFETIME ELISION RULES
    println!("7. LIFETIME ELISION RULES:\n");
    println!("   Rust can infer lifetimes in many cases:");
    println!("   Rule 1: Each parameter gets its own lifetime");
    println!("   Rule 2: If one input lifetime, that's the output lifetime");
    println!("   Rule 3: If &self or &mut self, that's the output lifetime\n");

    first_word("hello world");

    // 8. STRUCTS WITH LIFETIMES
    println!("8. STRUCTS WITH LIFETIMES:\n");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find '.'");
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("   Excerpt: {}", excerpt.part);
    println!("   Importance level: {}", excerpt.level());
    println!(
        "   Returned excerpt: {}",
        excerpt.announce_and_return_part("Lifetimes connect borrowed data")
    );
    println!("   ✓ Struct holds reference, needs lifetime annotation\n");

    // 9. LIFETIME BOUNDS
    println!("9. LIFETIME BOUNDS:\n");

    let string1 = String::from("hello");
    let string2 = String::from("world");

    let announcement = "Breaking news!";
    let result = longest_with_announcement(&string1, &string2, announcement);
    println!("   Result: {}", result);

    println!("   ✓ Combine lifetime parameters with trait bounds\n");

    // 10. STATIC LIFETIME
    println!("10. STATIC LIFETIME ('static):\n");

    let s: &'static str = "I live for the entire program duration";
    println!("   Static string: {}", s);

    println!("   ✓ 'static means reference lives for entire program");
    println!("   ✓ All string literals have 'static lifetime\n");

    // 11. PRACTICAL EXAMPLE
    println!("11. PRACTICAL EXAMPLE:\n");

    let tweet = String::from("Just learning Rust lifetimes!");
    let summary = summarize(&tweet, 20);
    println!("   Summary: {}", summary);

    println!("   ✓ Functions return borrowed data safely\n");

    // 12. COMMON LIFETIME PATTERNS
    println!("12. COMMON LIFETIME PATTERNS:\n");

    example_elision("hello");
    let data = vec![1, 2, 3, 4, 5];
    let first = get_first(&data);
    println!("   First element: {:?}", first);

    println!("   ✓ Most cases don't need explicit lifetimes\n");

    // 13. WHY LIFETIMES MATTER
    println!("13. WHY LIFETIMES MATTER:\n");
    println!("   - Prevent dangling pointers (compile-time!)");
    println!("   - No garbage collector needed");
    println!("   - Zero runtime cost");
    println!("   - Memory safety guaranteed");
    println!("   - Enable safe concurrent programming\n");
}

// Explicit lifetime annotation
// The returned reference has the same lifetime as the shorter of the two inputs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime elision - compiler infers lifetimes
// Rule: single input lifetime becomes output lifetime
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }

    s
}

// Struct with lifetime annotation
// This struct holds a reference, so it needs a lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Methods with lifetimes
impl<'a> ImportantExcerpt<'a> {
    // Lifetime elision: output lifetime comes from &self
    fn level(&self) -> i32 {
        3
    }

    // Both input and output have same lifetime
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Multiple lifetime parameters with generic type and trait bound
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Practical example: summarize string
fn summarize(text: &str, max_len: usize) -> &str {
    if text.len() <= max_len {
        text
    } else {
        &text[0..max_len]
    }
}

// Example with lifetime elision
fn example_elision(s: &str) -> &str {
    // Compiler infers: fn example_elision<'a>(s: &'a str) -> &'a str
    s
}

// Return reference to first element
fn get_first(data: &[i32]) -> Option<&i32> {
    data.first()
}

/*
Lifetimes Summary:
==================

KEY CONCEPTS:
1. Every reference has a lifetime
2. Lifetimes prevent dangling references
3. Most lifetimes are inferred
4. Sometimes need explicit annotations

LIFETIME NOTATION:
- &'a T       - Reference with lifetime 'a
- &'a mut T   - Mutable reference with lifetime 'a
- <'a>        - Lifetime parameter in generics
- 'static     - Lifetime of entire program

LIFETIME ELISION RULES:
1. Each parameter gets its own lifetime
2. One input lifetime → one output lifetime
3. Method with &self → output lifetime from self

WHEN TO USE EXPLICIT LIFETIMES:
- Multiple input references
- Returning references
- Structs holding references
- When compiler can't infer

COMMON PATTERNS:
- fn foo<'a>(x: &'a str) -> &'a str
- struct Foo<'a> { field: &'a str }
- impl<'a> Foo<'a> { ... }

WHY LIFETIMES?
- Compile-time memory safety
- No runtime overhead
- Prevents dangling pointers
- Enables zero-cost abstractions

COMPARISON:
C/C++:   Manual tracking (error-prone)
Java:    Garbage collector (runtime cost)
Python:  Reference counting (cycles possible)
Rust:    Compile-time (zero cost!)

Run this:
    cargo run

Experiment:
    - Write functions with different lifetime parameters
    - Create structs that hold references
    - Try returning references from functions
    - Violate lifetime rules to see compiler messages

Tips:
    - Don't fight the borrow checker
    - Read compiler error messages carefully
    - Start with ownership, then borrowing, then lifetimes
    - Most code doesn't need explicit lifetimes

Next:
    - Learn about smart pointers (Box, Rc, RefCell)
    - Master advanced patterns
    - Explore concurrent programming
*/
