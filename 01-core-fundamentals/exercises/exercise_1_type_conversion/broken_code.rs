// Exercise 1: Type Conversion Challenge (BROKEN CODE)
//
// This code has 3 bugs related to type conversion and handling
// Your job: Find and fix the bugs so it compiles and produces correct output

use std::io;

fn main() {
    println!("Temperature Converter");
    println!("====================\n");

    // Test cases instead of reading from user
    let temperatures = vec!["32", "98", "212"];

    for temp_str in temperatures {
        println!("Input temperature: {}", temp_str);

        // BUG 1: parse() returns a Result, but we're not handling it
        let fahrenheit: i32 = temp_str.parse();

        // Display in multiple formats
        println!("{}°F (i32)", fahrenheit);

        // BUG 2: This conversion is wrong - should convert to i64 not i32
        let fahrenheit_i64: i32 = fahrenheit as i64;
        println!("{} (i64)", fahrenheit_i64);

        // Convert to f64 for calculation
        let fahrenheit_float: f64 = fahrenheit as f64;
        println!("{}°F (f64 from conversion)", fahrenheit_float);

        // Calculate Celsius: C = (F - 32) * 5/9
        // BUG 3: Type mismatch - arithmetic with i32 and f64
        let celsius = (fahrenheit - 32) * 5.0 / 9.0;
        println!("Converted to Celsius: {}°C\n", celsius);
    }
}

// BUGS SUMMARY:
// 1. parse() returns Result<T, E> - must handle with .unwrap() or match
// 2. fahrenheit_i64 should be i64, not i32
// 3. Arithmetic between i32 and f64 - need to cast fahrenheit to f64 first

// EXPECTED BEHAVIOR:
// Input temperature: 32
// 32°F (i32)
// 32 (i64)
// 32.0°F (f64 from conversion)
// Converted to Celsius: 0.0°C
//
// Input temperature: 98
// 98°F (i32)
// 98 (i64)
// 98.0°F (f64 from conversion)
// Converted to Celsius: 36.7°C
//
// Input temperature: 212
// 212°F (i32)
// 212 (i64)
// 212.0°F (f64 from conversion)
// Converted to Celsius: 100.0°C
