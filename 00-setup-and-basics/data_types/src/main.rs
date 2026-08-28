fn main() {
    println!("=== Rust Data Types ===\n");

    // 1. SCALAR TYPES
    println!("1. SCALAR TYPES (single values):\n");

    // INTEGERS
    println!("   A. Integers:");
    let int8: i8 = -128; // 8-bit signed (-128 to 127)
    let int16: i16 = 32_767; // 16-bit signed
    let int32: i32 = 2_147_483_647; // 32-bit signed (default)
    let int64: i64 = 9_223_372_036_854_775_807; // 64-bit signed
    let int128: i128 = 1_000_000; // 128-bit signed

    let uint8: u8 = 255; // 8-bit unsigned (0 to 255)
    let uint16: u16 = 65_535; // 16-bit unsigned
    let uint32: u32 = 4_294_967_295; // 32-bit unsigned
    let uint64: u64 = 18_446_744_073_709_551_615; // 64-bit unsigned

    let arch_int: isize = 100; // size depends on architecture (32 or 64 bit)
    let arch_uint: usize = 100; // unsigned arch-dependent

    println!("      Signed: i8={int8}, i16={int16}, i32={int32}, i64={int64}, i128={int128}");
    println!("      Unsigned: u8={uint8}, u16={uint16}, u32={uint32}, u64={uint64}");
    println!("      isize = {}, usize = {}", arch_int, arch_uint);

    // Integer literals
    let decimal = 98_222; // Decimal
    let hex = 0xff; // Hexadecimal
    let octal = 0o77; // Octal
    let binary = 0b1111_0000; // Binary
    let byte = b'A'; // Byte (u8 only)

    println!(
        "      Decimal: {}, Hex: {}, Octal: {}, Binary: {}",
        decimal, hex, octal, binary
    );
    println!("      Byte 'A': {}\n", byte);

    // FLOATING POINT
    println!("   B. Floating Point:");
    let float32: f32 = std::f32::consts::PI;
    let float64: f64 = std::f64::consts::E;

    println!("      f32 = {}, f64 = {}", float32, float64);
    println!("      Addition: {}", 5.5 + 2.3);
    println!("      Division: {}\n", 10.0 / 3.0);

    // BOOLEAN
    println!("   C. Boolean:");
    let is_true: bool = true;
    let is_false = false; // Type inferred

    println!("      true: {}, false: {}", is_true, is_false);
    println!("      1 < 2: {}", 1 < 2);
    let expected = 5;
    let actual = 5;
    println!("      expected == actual: {}\n", expected == actual);

    // CHARACTER
    println!("   D. Character:");
    let char_a = 'a';
    let char_emoji = '😎';
    let char_chinese = '中';

    println!("      char 'a': {}", char_a);
    println!("      emoji: {}", char_emoji);
    println!("      Chinese: {}", char_chinese);
    println!("      ✓ char is 4 bytes (Unicode scalar value)\n");

    // 2. COMPOUND TYPES
    println!("\n2. COMPOUND TYPES (group multiple values):\n");

    // TUPLES
    println!("   A. Tuples:");
    let tuple: (i32, f64, char) = (500, 6.4, 'x');

    // Destructuring
    let (x, y, z) = tuple;
    println!("      Destructured: x={}, y={}, z={}", x, y, z);

    // Indexing
    let first = tuple.0;
    let second = tuple.1;
    println!("      Indexed: first={}, second={}", first, second);

    // Unit tuple (empty tuple)
    let unit: () = ();
    println!("      Unit type: {:?} (represents empty value)\n", unit);

    // ARRAYS
    println!("   B. Arrays:");
    let array = [1, 2, 3, 4, 5];
    let first_element = array[0];
    let last_element = array[4];

    println!("      Array: {:?}", array);
    println!("      First: {}, Last: {}", first_element, last_element);
    println!("      Length: {}", array.len());

    // Array with type annotation
    let typed_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("      Typed [i32; 5]: {:?}", typed_array);

    // Array with same value
    let fives = [5; 3]; // Same as [5, 5, 5]
    println!("      Repeated [5; 3]: {:?}", fives);

    // Arrays are FIXED SIZE
    println!("      ✓ Arrays have fixed size (known at compile time)\n");

    // 3. STRING TYPES
    println!("\n3. STRING TYPES:\n");

    // String slice (borrowed)
    let string_slice: &str = "Hello, world!";
    println!("   String slice (&str): {}", string_slice);

    // String (owned)
    let mut owned_string: String = String::from("Hello");
    owned_string.push_str(", Rust!");
    println!("   Owned String: {}", owned_string);
    println!("   ✓ &str is immutable, String is growable\n");

    // 4. TYPE CONVERSION
    println!("4. TYPE CONVERSION:\n");

    let int_value = 42;
    let float_value = int_value as f64; // Casting
    println!("   i32 {} as f64: {}", int_value, float_value);

    let char_value = 'A';
    let ascii_value = char_value as u8;
    println!("   char '{}' as u8: {}", char_value, ascii_value);

    // Parsing strings
    let string_num = "42";
    let parsed: i32 = string_num.parse().expect("Not a number");
    println!("   Parsed \"42\": {}\n", parsed);

    // 5. TYPE ALIASES
    println!("5. TYPE ALIASES:\n");

    type Kilometers = i32;
    let distance: Kilometers = 100;
    println!("   Distance: {} km", distance);
    println!("   ✓ Type aliases improve readability\n");

    // 6. OVERFLOW BEHAVIOR
    println!("6. INTEGER OVERFLOW:\n");
    println!("   In debug mode: panics on overflow");
    println!("   In release mode: wraps around (modulo)");
    println!("   Use: wrapping_*, checked_*, saturating_*, overflowing_*\n");

    let num: u8 = 255;
    let wrapped = num.wrapping_add(1); // Wraps to 0
    println!("   255.wrapping_add(1) = {}", wrapped);

    match 200u8.checked_add(100) {
        Some(val) => println!("   200 + 100 = {}", val),
        None => println!("   200 + 100 = Overflow!"),
    }

    // 7. SIZE OF TYPES
    println!("\n7. SIZE OF TYPES (in bytes):\n");
    println!("   i8/u8: {} byte", std::mem::size_of::<i8>());
    println!("   i32/u32: {} bytes", std::mem::size_of::<i32>());
    println!("   i64/u64: {} bytes", std::mem::size_of::<i64>());
    println!("   f32: {} bytes", std::mem::size_of::<f32>());
    println!("   f64: {} bytes", std::mem::size_of::<f64>());
    println!("   char: {} bytes", std::mem::size_of::<char>());
    println!("   bool: {} byte", std::mem::size_of::<bool>());
    println!(
        "   usize/isize: {} bytes (on this machine)",
        std::mem::size_of::<usize>()
    );
}

/*
Rust Data Type Summary:
=======================

SCALAR TYPES:
- Integers: i8, i16, i32, i64, i128, isize (signed)
           u8, u16, u32, u64, u128, usize (unsigned)
- Floats:   f32, f64
- Boolean:  bool (true/false)
- Character: char (4 bytes, Unicode)

COMPOUND TYPES:
- Tuples:   (T1, T2, ...) - fixed size, different types
- Arrays:   [T; N] - fixed size, same type

STRING TYPES:
- &str:     String slice (borrowed, immutable)
- String:   Owned, growable string

KEY POINTS:
- Default integer is i32
- Default float is f64
- Arrays are fixed size (use Vec for dynamic)
- Strings have two types: &str and String
- Type inference works most of the time
- Use `as` for explicit type casting

Run this:
    cargo run

Experiment:
    - Try different number bases (hex, binary, octal)
    - Create your own tuples and arrays
    - Try overflow with checked_add, wrapping_add
*/
