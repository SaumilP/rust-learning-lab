// Example: Primitive Data Types
//
// Demonstrates:
// - Integer types (i32, u32, i64, etc.)
// - Floating-point types (f32, f64)
// - Boolean type
// - Character type
// - Type ranges and overflow behavior

fn main() {
    println!("=== Integer Types ===\n");

    // Signed integers (can be negative)
    let i8_var: i8 = 127;
    let i16_var: i16 = 32_767;
    let i32_var: i32 = 2_147_483_647;
    let i64_var: i64 = 9_223_372_036_854_775_807;
    let isize_var: isize = 100;  // Platform-dependent

    println!("i8 ({}): {}", std::mem::size_of::<i8>() * 8, i8_var);
    println!("i16 ({}): {}", std::mem::size_of::<i16>() * 8, i16_var);
    println!("i32 ({}): {}", std::mem::size_of::<i32>() * 8, i32_var);
    println!("i64 ({}): {}", std::mem::size_of::<i64>() * 8, i64_var);
    println!("isize: {}", isize_var);

    println!("\n=== Unsigned Integer Types ===\n");

    // Unsigned integers (only non-negative)
    let u8_var: u8 = 255;
    let u16_var: u16 = 65_535;
    let u32_var: u32 = 4_294_967_295;
    let u64_var: u64 = 18_446_744_073_709_551_615;
    let usize_var: usize = 200;  // Platform-dependent

    println!("u8 ({}): {}", std::mem::size_of::<u8>() * 8, u8_var);
    println!("u16 ({}): {}", std::mem::size_of::<u16>() * 8, u16_var);
    println!("u32 ({}): {}", std::mem::size_of::<u32>() * 8, u32_var);
    println!("u64 ({}): {}", std::mem::size_of::<u64>() * 8, u64_var);
    println!("usize: {}", usize_var);

    println!("\n=== Floating-Point Types ===\n");

    let f32_var: f32 = 3.14;
    let f64_var: f64 = 3.14159265359;
    let negative: f64 = -0.5;
    let very_small: f64 = 0.0000001;
    let very_large: f64 = 1_000_000.0;

    println!("f32: {}", f32_var);
    println!("f64: {}", f64_var);
    println!("Negative: {}", negative);
    println!("Very small: {}", very_small);
    println!("Very large: {}", very_large);

    println!("\n=== Boolean Type ===\n");

    let yes: bool = true;
    let no: bool = false;

    println!("true: {}", yes);
    println!("false: {}", no);
    println!("5 > 3: {}", 5 > 3);
    println!("5 == 5: {}", 5 == 5);
    println!("5 < 3: {}", 5 < 3);

    println!("\n=== Character Type ===\n");

    let letter: char = 'A';
    let digit: char = '7';
    let space: char = ' ';
    let emoji: char = '😀';

    println!("letter: '{}' (code: {})", letter, letter as u32);
    println!("digit: '{}' (code: {})", digit, digit as u32);
    println!("space: '{}' (code: {})", space, space as u32);
    println!("emoji: '{}' (code: {})", emoji, emoji as u32);

    println!("\n=== Number Literals ===\n");

    let decimal = 98_222;  // Underscores for readability
    let hex = 0xff;        // Hexadecimal
    let octal = 0o77;      // Octal
    let binary = 0b1111_0000;  // Binary
    let byte = b'A';       // u8 as byte

    println!("decimal: {}", decimal);
    println!("hex (0xff): {}", hex);
    println!("octal (0o77): {}", octal);
    println!("binary (0b1111_0000): {}", binary);
    println!("byte (b'A'): {}", byte);

    println!("\n=== Special Floating-Point Values ===\n");

    let inf: f64 = f64::INFINITY;
    let neg_inf: f64 = f64::NEG_INFINITY;
    let not_a_number: f64 = f64::NAN;

    println!("INFINITY: {}", inf);
    println!("NEG_INFINITY: {}", neg_inf);
    println!("NAN: {}", not_a_number);
    println!("NAN == NAN: {}", not_a_number == not_a_number);  // false!

    println!("\n=== Type Sizes ===\n");

    println!("Size of i32: {} bytes", std::mem::size_of::<i32>());
    println!("Size of i64: {} bytes", std::mem::size_of::<i64>());
    println!("Size of f32: {} bytes", std::mem::size_of::<f32>());
    println!("Size of f64: {} bytes", std::mem::size_of::<f64>());
    println!("Size of bool: {} byte", std::mem::size_of::<bool>());
    println!("Size of char: {} bytes", std::mem::size_of::<char>());

    println!("\n=== Default Values ===\n");

    let default_i32: i32 = Default::default();
    let default_f64: f64 = Default::default();
    let default_bool: bool = Default::default();

    println!("Default i32: {}", default_i32);
    println!("Default f64: {}", default_f64);
    println!("Default bool: {}", default_bool);
}
