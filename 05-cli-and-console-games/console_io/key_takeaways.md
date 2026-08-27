# Console I/O - Key Takeaways

## Reading Input

```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input)
    .expect("Failed to read line");

let trimmed = input.trim();  // Remove newline
```

## Writing Output

```rust
// With newline
println!("Hello, World!");

// Without newline
print!("Enter name: ");

// Formatted output
println!("Score: {} / {}", 100, 200);

// Errors to stderr
eprintln!("Error: invalid input");
```

## Flushing Output

```rust
use std::io::Write;

print!("Enter name: ");
io::stdout().flush().unwrap();  // Force output before input

let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
```

## Parsing Input

```rust
let num: i32 = input.trim().parse()
    .expect("Not a number");

// With error handling
match input.trim().parse::<i32>() {
    Ok(num) => println!("Got: {}", num),
    Err(_) => println!("Invalid number"),
}
```

## Common I/O Patterns

| Operation | Code |
|-----------|------|
| Read line | `io::stdin().read_line(&mut s)?` |
| Print | `print!("{}", x)` |
| Print line | `println!("{}", x)` |
| Error print | `eprintln!("{}", x)` |
| Flush output | `io::stdout().flush()?` |
| Parse string | `s.parse::<T>()?` |

## Input Loop Pattern

```rust
use std::io;

loop {
    println!("Enter command:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "quit" => break,
        cmd => println!("You entered: {}", cmd),
    }
}
```

## Menu System Pattern

```rust
use std::io::{self, Write};

fn show_menu() {
    println!("=== MENU ===");
    println!("1. Play");
    println!("2. Settings");
    println!("3. Exit");
    print!("> ");
    io::stdout().flush().unwrap();
}

// Main loop
loop {
    show_menu();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => { /* play */ },
        "2" => { /* settings */ },
        "3" => break,
        _ => println!("Invalid"),
    }
}
```

## Output Formatting

```rust
// Padding
println!("{:20}", "text");      // Right-padded

// Alignment
println!("{:<10}", "left");     // Left-aligned
println!("{:^10}", "center");   // Center-aligned
println!("{:>10}", "right");    // Right-aligned

// Decimals
println!("{:.2}", 3.14159);     // 2 decimal places

// Hex/Binary
println!("0x{:x}", 255);        // Hexadecimal
println!("0b{:b}", 255);        // Binary
```

## String Input Utilities

```rust
// Read line and trim
let input = {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
};

// Parse with default
let num = input.trim().parse::<i32>().unwrap_or(0);

// Case-insensitive comparison
if input.trim().to_lowercase() == "yes" { }
```

## Common Mistakes to Avoid

1. ❌ Forgetting to trim input
   ```rust
   if input == "quit" { }  // Never matches due to \n
   ```

2. ❌ Not flushing prompt
   ```rust
   print!("Enter: ");
   // Prompt may not appear before input
   ```

3. ❌ Not handling parse errors
   ```rust
   let num: i32 = input.parse().unwrap();  // Panics on bad input
   ```

4. ❌ Printing errors to stdout
   ```rust
   println!("Error!");  // Should use eprintln!
   ```

5. ❌ Blocking indefinitely on input
   ```rust
   // In game loop - freezes everything
   io::stdin().read_line(&mut input)?;
   ```

## Important Notes

✓ Always trim input before comparing strings
✓ Flush prompts before reading (especially for print! without \n)
✓ Use `?` operator for clean error propagation
✓ Use eprintln! for error messages
✓ Handle invalid input gracefully with match/if let
✓ Use to_lowercase() for case-insensitive checks
✓ Parse early, validate thoroughly

## Quick Reference

```rust
use std::io::{self, Write};

// Read input
let mut input = String::new();
io::stdin().read_line(&mut input)?;
let trimmed = input.trim();

// Write output
println!("Output: {}", trimmed);
eprintln!("Error: {}", reason);

// Prompt with flush
print!("Enter: ");
io::stdout().flush()?;

// Parse input
match trimmed.parse::<i32>() {
    Ok(num) => println!("Got: {}", num),
    Err(e) => println!("Error: {}", e),
}

// Menu loop
loop {
    show_menu();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    match choice.trim() {
        "1" => action1(),
        "q" => break,
        _ => println!("Invalid"),
    }
}
```

## Performance Tips

✓ Create stdin/stdout once, reuse them for many operations
✓ Buffering is handled automatically by Rust
✓ Avoid String allocations in tight loops
✓ Use String::with_capacity() if you know expected size

## Related Concepts

- Game loops (integrating I/O)
- String operations (parsing, formatting)
- Error handling (Result, Option)
- Traits (Read, Write, Display)

