// Example: String Types (&str vs String)
//
// Demonstrates:
// - String slices (&str)
// - Owned String type
// - String methods
// - Converting between types

fn main() {
    println!("=== String Slices (&str) ===\n");

    let hello: &str = "Hello"; // String literal (static)
    let world: &str = "world";

    println!("hello: '{}'", hello);
    println!("world: '{}'", world);
    println!("Combined: '{} {}'", hello, world);

    // String slices are immutable
    let msg: &str = "Fixed string";
    println!("Message: '{}'", msg);
    // msg.push('!');  // Error: can't modify &str

    println!("\n=== Owned String Type ===\n");

    // Create String from literal
    let mut greeting = String::from("Hello");
    println!("greeting: '{}'", greeting);

    greeting.push_str(" Rust");
    println!("After push_str: '{}'", greeting);

    greeting.push('!');
    println!("After push: '{}'", greeting);

    println!("\n=== Creating Strings ===\n");

    // Different ways to create strings
    let s1 = String::new();
    println!("Empty String: '{}'", s1);

    let s2 = String::from("Hello");
    println!("From literal: '{}'", s2);

    let s3 = "World".to_string();
    println!("to_string(): '{}'", s3);

    let s4 = format!("Hello, {}!", "Rust");
    println!("format!: '{}'", s4);

    println!("\n=== String Length ===\n");

    let text = "Hello";
    println!("Text: '{}'", text);
    println!("Length: {}", text.len());
    println!("Is empty: {}", text.is_empty());

    let utf8_text = "café";
    println!("UTF-8 text: '{}'", utf8_text);
    println!("Byte length: {}", utf8_text.len()); // 5 bytes!
    println!("Character count: {}", utf8_text.chars().count()); // 4 chars

    println!("\n=== String Modification ===\n");

    let mut msg = String::from("Hello");
    println!("Original: '{}'", msg);

    msg.push_str(" World");
    println!("After push_str: '{}'", msg);

    msg.push('!');
    println!("After push: '{}'", msg);

    msg.insert(6, ' ');
    println!("After insert at 6: '{}'", msg);

    println!("\n=== String Methods ===\n");

    let text = "Hello World";

    println!("Text: '{}'", text);
    println!("uppercase: '{}'", text.to_uppercase());
    println!("lowercase: '{}'", text.to_lowercase());
    println!("trimmed: '{}'", "  spaces  ".trim());
    println!("starts_with 'Hello': {}", text.starts_with("Hello"));
    println!("contains 'World': {}", text.contains("World"));

    println!("\n=== String Slicing ===\n");

    let s = String::from("Hello");
    let slice1 = &s[0..2]; // "He"
    let slice2 = &s[2..5]; // "llo"

    println!("Full string: '{}'", s);
    println!("Slice [0..2]: '{}'", slice1);
    println!("Slice [2..5]: '{}'", slice2);

    println!("\n=== String Splitting ===\n");

    let csv = "one,two,three,four";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("CSV: '{}'", csv);
    println!("Parts: {:?}", parts);

    for (i, part) in parts.iter().enumerate() {
        println!("  [{}]: '{}'", i, part);
    }

    println!("\n=== Converting Between Types ===\n");

    let str_slice: &str = "123";
    let owned_string: String = str_slice.to_string();
    let back_to_slice: &str = &owned_string;

    println!("str_slice: '{}' (&str)", str_slice);
    println!("owned_string: '{}' (String)", owned_string);
    println!("back_to_slice: '{}' (&str)", back_to_slice);

    println!("\n=== String Capacity ===\n");

    let mut s = String::with_capacity(10);
    println!("Capacity: {}", s.capacity());
    println!("Length: {}", s.len());

    s.push_str("Hello");
    println!("After 'Hello':");
    println!("  Capacity: {}", s.capacity());
    println!("  Length: {}", s.len());

    println!("\n=== Multiline Strings ===\n");

    let multiline = "Line 1
Line 2
Line 3";
    println!("Multiline:\n{}", multiline);

    let raw = r"Raw string with \n escapes";
    println!("Raw: {}", raw);
}
