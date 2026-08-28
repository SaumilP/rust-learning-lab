// Example: Type Casting with the `as` Keyword
//
// Demonstrates:
// - Numeric type conversions with `as`
// - Integer to float and float to integer casting
// - Truncation and overflow behavior
// - Safe conversions with TryFrom/TryInto

fn main() {
    println!("=== Basic Numeric Casting ===\n");

    // Integer to larger integer (safe)
    let small: i8 = 42;
    let larger: i32 = small as i32;
    println!("i8 {} -> i32 {}", small, larger);

    // Integer to float
    let integer: i32 = 100;
    let float: f64 = integer as f64;
    println!("i32 {} -> f64 {}", integer, float);

    // Float to integer (truncates decimal part)
    let pi: f64 = 3.14159;
    let truncated: i32 = pi as i32;
    println!("f64 {} -> i32 {} (truncated)", pi, truncated);

    println!("\n=== Float Precision ===\n");

    // f32 to f64 (safe, more precision)
    let small_float: f32 = 3.14;
    let large_float: f64 = small_float as f64;
    println!("f32 {} -> f64 {}", small_float, large_float);

    // f64 to f32 (may lose precision)
    let precise: f64 = 3.141592653589793;
    let less_precise: f32 = precise as f32;
    println!("f64 {} -> f32 {} (precision loss)", precise, less_precise);

    println!("\n=== Signed and Unsigned ===\n");

    // Unsigned to signed (same size, no data loss if within range)
    let unsigned: u8 = 200;
    let signed: i16 = unsigned as i16;
    println!("u8 {} -> i16 {}", unsigned, signed);

    // Signed to unsigned (wraps if negative)
    let negative: i8 = -1;
    let as_unsigned: u8 = negative as u8;
    println!("i8 {} -> u8 {} (wraps around)", negative, as_unsigned);

    println!("\n=== Truncation and Overflow ===\n");

    // Larger to smaller integer (truncates)
    let big_number: i32 = 1000;
    let truncated: i8 = big_number as i8;
    println!(
        "i32 {} -> i8 {} (truncated/overflow)",
        big_number, truncated
    );

    // This is because 1000 mod 256 = 232, and as signed it's -24
    let big_unsigned: u32 = 300;
    let small_unsigned: u8 = big_unsigned as u8;
    println!(
        "u32 {} -> u8 {} (300 mod 256 = 44)",
        big_unsigned, small_unsigned
    );

    println!("\n=== Character Casting ===\n");

    // char to integer
    let letter: char = 'A';
    let ascii_value: u8 = letter as u8;
    let unicode_value: u32 = letter as u32;
    println!(
        "char '{}' -> u8 {}, u32 {}",
        letter, ascii_value, unicode_value
    );

    // Integer to char (only valid unicode scalar values)
    let number: u8 = 66;
    let as_char: char = number as char;
    println!("u8 {} -> char '{}'", number, as_char);

    println!("\n=== Bool Casting ===\n");

    // Bool to integer
    let true_val: bool = true;
    let false_val: bool = false;
    println!("bool true -> i32 {}", true_val as i32);
    println!("bool false -> i32 {}", false_val as i32);

    // Note: Cannot cast integer to bool directly
    // let num = 1;
    // let b: bool = num as bool;  // Error!

    println!("\n=== Pointer Casting ===\n");

    // Reference to raw pointer
    let value = 42;
    let reference = &value;
    let raw_pointer = reference as *const i32;
    println!("Reference {:p} -> Raw pointer {:p}", reference, raw_pointer);

    // Pointer to usize (memory address)
    let address: usize = raw_pointer as usize;
    println!("Pointer address as usize: {}", address);

    println!("\n=== Safe Conversions with TryFrom ===\n");

    use std::convert::TryFrom;
    use std::convert::TryInto;

    // TryFrom for safe conversion
    let big: i32 = 100;
    match i8::try_from(big) {
        Ok(val) => println!("i32 {} safely converted to i8 {}", big, val),
        Err(e) => println!("Conversion failed: {}", e),
    }

    let too_big: i32 = 1000;
    match i8::try_from(too_big) {
        Ok(val) => println!("i32 {} converted to i8 {}", too_big, val),
        Err(e) => println!("i32 {} -> i8 failed: {}", too_big, e),
    }

    // TryInto for method-style conversion
    let value: u32 = 50;
    let result: Result<u8, _> = value.try_into();
    match result {
        Ok(val) => println!("u32 {} -> u8 {} using TryInto", value, val),
        Err(e) => println!("Conversion failed: {}", e),
    }

    println!("\n=== From and Into Traits ===\n");

    // From trait (infallible conversion)
    let small: i8 = 42;
    let larger: i32 = i32::from(small); // Always safe
    println!("i8 {} -> i32 {} using From", small, larger);

    // Into trait (method style)
    let byte: u8 = 100;
    let bigger: u32 = byte.into();
    println!("u8 {} -> u32 {} using Into", byte, bigger);

    println!("\n=== Casting in Expressions ===\n");

    // Mixed type arithmetic requires casting
    let a: i32 = 10;
    let b: i64 = 20;
    let sum: i64 = a as i64 + b;
    println!("{} (i32) + {} (i64) = {} (i64)", a, b, sum);

    // Float and integer arithmetic
    let integer_part: i32 = 5;
    let fraction: f64 = 0.5;
    let result: f64 = integer_part as f64 + fraction;
    println!(
        "{} (i32) + {} (f64) = {} (f64)",
        integer_part, fraction, result
    );
}
