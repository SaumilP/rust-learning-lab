// Exercise 2: String Manipulation Challenge (BROKEN CODE)
//
// This code has 3 bugs related to string operations and ownership
// Your job: Find and fix the bugs so it compiles and produces correct output

use std::io;

fn main() {
    println!("String Manipulation Challenge");
    println!("=============================\n");

    // Test cases with different strings
    let strings = vec!["Hello Rust", "   Rust Programming   ", "The quick brown fox"];

    for test_str in strings {
        println!("Input string: \"{}\"\n", test_str);

        // BUG 1: Trying to mutate a &str directly
        // &str is immutable, need to convert to String first
        let mut original = test_str;
        original.push_str(" World");  // This won't compile - &str doesn't support push_str

        println!("Original: \"{}\"", original);
        println!("Uppercase: \"{}\"", original.to_uppercase());
        println!("Lowercase: \"{}\"", original.to_lowercase());
        println!("Length: {}", original.len());

        // Reverse string
        let reversed: String = original.chars().rev().collect();
        println!("Reversed: \"{}\"", reversed);

        // Remove spaces using filter
        let no_spaces: String = original
            .chars()
            .filter(|c| c != ' ')
            .collect();
        println!("Without spaces: \"{}\"", no_spaces);

        // Count character occurrences
        // BUG 2: Trying to pass a char but using a string literal
        let target_char = "l";  // This is a &str, not a char
        let count = original.chars().filter(|c| c == target_char).count();
        println!("Character count ({}): {}", target_char, count);

        // Split into words
        // BUG 3: Forgot to collect() the iterator result into a Vec
        let words = original.split(' ');
        println!("Words: {:?}", words);  // This will print an iterator debug output, not the values

        println!();
    }
}

// BUGS SUMMARY:
// 1. Can't mutate &str - need to convert to String using String::from() or .to_string()
// 2. Comparing char with &str - use single quotes for char literal: 'l' instead of "l"
// 3. split() returns an iterator, not a Vec - need .collect::<Vec<_>>()

// EXPECTED BEHAVIOR:
// Input string: "Hello Rust"
//
// Original: "Hello Rust"
// Uppercase: "HELLO RUST"
// Lowercase: "hello rust"
// Length: 10
// Reversed: "tsuR olleH"
// Without spaces: "HelloRust"
// Character count (l): 3
// Words: ["Hello", "Rust"]
//
// Input string: "   Rust Programming   "
//
// Original: "   Rust Programming   "
// Uppercase: "   RUST PROGRAMMING   "
// Lowercase: "   rust programming   "
// Length: 21
// Reversed: "   gnimmargorP tsuR   "
// Without spaces: "RustProgramming"
// Character count (r): 4
// Words: ["", "", "", "Rust", "Programming", "", "", ""]
//
// Input string: "The quick brown fox"
//
// Original: "The quick brown fox"
// Uppercase: "THE QUICK BROWN FOX"
// Lowercase: "the quick brown fox"
// Length: 19
// Reversed: "xof nworb kciuq ehT"
// Without spaces: "Thequickbrownfox"
// Character count (o): 4
// Words: ["The", "quick", "brown", "fox"]
