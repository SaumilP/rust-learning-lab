// Example: String Formatting with format! Macro
//
// Demonstrates:
// - format! macro syntax
// - Positional and named arguments
// - Formatting options: width, precision, alignment
// - Debug and display formatting

fn main() {
    println!("=== Basic format! Usage ===\n");

    // Simple string interpolation
    let name = "Alice";
    let age = 30;
    let message = format!("Hello, {}! You are {} years old.", name, age);
    println!("{}", message);

    // Multiple values
    let a = 10;
    let b = 20;
    let result = format!("{} + {} = {}", a, b, a + b);
    println!("{}", result);

    println!("\n=== Positional Arguments ===\n");

    // Reuse arguments by position
    let formatted = format!(
        "{0} said hello to {1}, then {1} waved back to {0}",
        "Alice", "Bob"
    );
    println!("{}", formatted);

    // Mix positional and regular
    let formatted = format!("{1} is {0}'s friend. {0} is {2}", "Alice", "Bob", "happy");
    println!("{}", formatted);

    println!("\n=== Named Arguments ===\n");

    let formatted = format!(
        "{name} is {age} years old and lives in {city}",
        name = "Alice",
        age = 30,
        city = "Wonderland"
    );
    println!("{}", formatted);

    // Mix named and positional
    let formatted = format!(
        "{0} works as a {role} in {1}",
        "Bob",
        "TechCorp",
        role = "developer"
    );
    println!("{}", formatted);

    println!("\n=== Width and Alignment ===\n");

    let text = "Hello";

    // Minimum width (default right-aligned)
    println!("'{:10}' - right aligned, width 10", text);

    // Left aligned
    println!("'{:<10}' - left aligned, width 10", text);

    // Center aligned
    println!("'{:^10}' - center aligned, width 10", text);

    // Right aligned (explicit)
    println!("'{:>10}' - right aligned, width 10", text);

    // Fill character
    println!("'{:*<10}' - left aligned, fill with *", text);
    println!("'{:*^10}' - center aligned, fill with *", text);
    println!("'{:*>10}' - right aligned, fill with *", text);

    println!("\n=== Number Formatting ===\n");

    let number = 42;

    // Width
    println!("'{:5}' - width 5", number);
    println!("'{:05}' - width 5, zero-padded", number);

    // Sign
    println!("'{:+}' - always show sign", number);
    println!("'{:+5}' - sign with width", number);

    // Negative number
    let negative = -42;
    println!("'{:5}' - negative with width", negative);
    println!("'{:05}' - negative zero-padded", negative);

    println!("\n=== Float Precision ===\n");

    let pi = 3.14159265358979;

    println!("Default:   {}", pi);
    println!("2 decimal: {:.2}", pi);
    println!("4 decimal: {:.4}", pi);
    println!("0 decimal: {:.0}", pi);

    // Width and precision
    println!("Width 10, 3 decimal: '{:10.3}'", pi);
    println!("Width 10, 3 decimal, zero-pad: '{:010.3}'", pi);

    // Scientific notation
    println!("Scientific: {:e}", pi);
    println!("Scientific uppercase: {:E}", pi);

    println!("\n=== Number Bases ===\n");

    let number = 255;

    println!("Decimal:     {}", number);
    println!("Binary:      {:b}", number);
    println!("Octal:       {:o}", number);
    println!("Hexadecimal: {:x}", number);
    println!("Hexadecimal: {:X}", number);

    // With prefix
    println!("Binary:      {:#b}", number);
    println!("Octal:       {:#o}", number);
    println!("Hex:         {:#x}", number);
    println!("Hex:         {:#X}", number);

    // Width with bases
    println!("Hex padded:  {:08x}", number);

    println!("\n=== Debug Formatting ===\n");

    #[allow(dead_code)]
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // {:?} - debug format
    println!("Debug: {:?}", person);

    // {:#?} - pretty debug format
    println!("Pretty debug: {:#?}", person);

    // Arrays and vectors
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Vector debug: {:?}", numbers);

    // Tuples
    let tuple = (1, "hello", 3.14);
    println!("Tuple debug: {:?}", tuple);

    println!("\n=== Pointer Formatting ===\n");

    let x = 42;
    let ptr = &x;
    println!("Pointer address: {:p}", ptr);

    println!("\n=== Escaping Braces ===\n");

    // Use {{ and }} to escape braces
    let formatted = format!("Set contains {{1, 2, 3}}");
    println!("{}", formatted);

    let value = 42;
    let formatted = format!("Value: {{{}}} in braces", value);
    println!("{}", formatted);

    println!("\n=== Dynamic Width and Precision ===\n");

    let width = 10;
    let precision = 3;
    let value = 3.14159;

    // Width from variable
    println!("'{:width$}' - dynamic width", "Hello", width = width);

    // Precision from variable
    println!(
        "'{:.precision$}' - dynamic precision",
        value,
        precision = precision
    );

    // Both
    println!(
        "'{:width$.precision$}' - both dynamic",
        value,
        width = width,
        precision = precision
    );

    println!("\n=== Practical Examples ===\n");

    // Table formatting
    println!("| {:^10} | {:^10} | {:^10} |", "Name", "Age", "City");
    println!("|{:-^12}|{:-^12}|{:-^12}|", "", "", "");
    println!("| {:^10} | {:^10} | {:^10} |", "Alice", "30", "London");
    println!("| {:^10} | {:^10} | {:^10} |", "Bob", "25", "Paris");

    // Progress bar
    fn progress_bar(percent: u32) -> String {
        let filled = (percent / 5) as usize;
        let empty = 20 - filled;
        format!(
            "[{}{}] {:>3}%",
            "=".repeat(filled),
            " ".repeat(empty),
            percent
        )
    }

    println!("\nProgress bars:");
    println!("{}", progress_bar(0));
    println!("{}", progress_bar(25));
    println!("{}", progress_bar(50));
    println!("{}", progress_bar(75));
    println!("{}", progress_bar(100));

    // Money formatting
    fn format_money(amount: f64) -> String {
        format!("${:>10.2}", amount)
    }

    println!("\nMoney formatting:");
    println!("{}", format_money(1234.5));
    println!("{}", format_money(99.99));
    println!("{}", format_money(0.5));

    // Hex dump style
    let bytes = [0x48, 0x65, 0x6c, 0x6c, 0x6f];
    print!("\nHex dump: ");
    for byte in bytes {
        print!("{:02x} ", byte);
    }
    println!();

    // Log message format
    fn log_message(level: &str, msg: &str) -> String {
        format!("[{:>5}] {}", level, msg)
    }

    println!("\nLog messages:");
    println!("{}", log_message("INFO", "Application started"));
    println!("{}", log_message("WARN", "Low memory"));
    println!("{}", log_message("ERROR", "Connection failed"));
}
