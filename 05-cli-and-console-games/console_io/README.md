# Console I/O

## Overview

Console input/output is the primary interface for text-based games and interactive programs. Rust's `std::io` module provides powerful tools for reading from stdin and writing to stdout/stderr. Mastering console I/O enables building responsive, user-friendly terminal applications.

## Theory

### Input/Output Streams

- **stdin** - Standard input (keyboard)
- **stdout** - Standard output (display)
- **stderr** - Standard error (error messages)
- **Buffering** - Performance optimization for I/O

### Common I/O Patterns

1. **Reading Lines** - Get user input
2. **Writing Output** - Display information
3. **Formatting** - Control output appearance
4. **Buffering** - Manage I/O efficiency

## Syntax

### Reading User Input

```rust
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    println!("You entered: {}", input.trim());
}
```

### Writing Output

```rust
fn main() {
    // Println! with auto-newline
    println!("Hello, World!");

    // Print without newline
    print!("Enter command: ");

    // Write formatted output
    println!("Score: {} / {}", 100, 200);
}
```

### Error Output

```rust
fn main() {
    // Print to stderr
    eprintln!("Error: File not found");

    // Use for errors, not regular output
}
```

### Immediate Output with Flushing

```rust
use std::io::{self, Write};

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Hello, {}!", name.trim());
}
```

## Common Patterns

### Pattern 1: Simple Input Loop

```rust
use std::io;

fn main() {
    loop {
        println!("Enter command (or 'quit' to exit):");
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read");

        match input.trim() {
            "quit" => break,
            _ => println!("You entered: {}", input.trim()),
        }
    }
}
```

### Pattern 2: Parsing User Input

```rust
use std::io;

fn main() {
    loop {
        println!("Enter a number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(num) => println!("You entered: {}", num),
            Err(_) => println!("Invalid number"),
        }
    }
}
```

### Pattern 3: Menu System

```rust
fn display_menu() {
    println!("\n=== MENU ===");
    println!("1. Play");
    println!("2. Settings");
    println!("3. Exit");
    print!("> ");
    std::io::stdout().flush().unwrap();
}

fn main() {
    loop {
        display_menu();

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => println!("Starting game..."),
            "2" => println!("Opening settings..."),
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }
}
```

### Pattern 4: Formatted Output

```rust
fn main() {
    // Padding
    println!("Name: {:20} Level: {}", "Hero", 5);

    // Alignment
    println!("{:>10} {:^10} {:<10}", "Right", "Center", "Left");

    // Decimals
    println!("Health: {:.2}%", 99.5);

    // Hex, binary
    println!("0x{:x} 0b{:b}", 255, 255);
}
```

### Pattern 5: Clear Screen and Formatting

```rust
fn clear_screen() {
    println!("\x1B[2J\x1B[H");  // ANSI escape codes
}

fn main() {
    clear_screen();
    println!("Welcome to the Game!");
}
```

## Common Mistakes

### Mistake 1: Not Trimming Input

```rust
// ❌ WRONG - Newline included in comparison
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
if input == "quit" { }  // Never matches due to newline

// ✅ CORRECT
if input.trim() == "quit" { }
```

### Mistake 2: Not Flushing Prompt

```rust
// ❌ WRONG - Prompt may not appear before input
print!("Enter name: ");

// ✅ CORRECT
print!("Enter name: ");
io::stdout().flush().unwrap();
```

### Mistake 3: Not Handling Parse Errors

```rust
// ❌ WRONG - Panics on invalid input
let num: i32 = input.parse().unwrap();

// ✅ CORRECT
match input.parse::<i32>() {
    Ok(num) => println!("Entered: {}", num),
    Err(_) => println!("Invalid number"),
}
```

### Mistake 4: Mixing stdout and stderr

```rust
// ❌ WRONG - Errors to stdout
println!("Error: invalid input");

// ✅ CORRECT - Errors to stderr
eprintln!("Error: invalid input");
```

### Mistake 5: Blocking without Timeout

```rust
// ❌ WRONG - Game loop freezes waiting for input
loop {
    io::stdin().read_line(&mut input)?;  // Blocks indefinitely
    // Rest of game logic never runs
}

// ✅ CORRECT - Use timeout or event-driven approach
// See game loop basics for proper handling
```

## Real-World Examples

### Example 1: Simple Quiz Game

```rust
use std::io::{self, Write};

fn main() {
    let questions = vec![
        ("What is 2+2?", "4"),
        ("What is the capital of France?", "paris"),
    ];

    let mut score = 0;

    for (question, answer) in questions {
        println!("\n{}", question);
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() == answer {
            println!("Correct!");
            score += 1;
        } else {
            println!("Wrong! Answer was: {}", answer);
        }
    }

    println!("\nFinal score: {}", score);
}
```

### Example 2: Interactive Calculator

```rust
use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter expression (or 'exit'): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split(' ').collect();
        if parts.len() == 3 {
            let a: i32 = parts[0].parse().unwrap_or(0);
            let op = parts[1];
            let b: i32 = parts[2].parse().unwrap_or(0);

            let result = match op {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => if b != 0 { a / b } else { 0 },
                _ => 0,
            };

            println!("Result: {}", result);
        }
    }
}
```

### Example 3: Player Stats Display

```rust
fn display_stats(name: &str, level: i32, health: i32, max_health: i32) {
    println!("\n╔════════════════════╗");
    println!("║ {} Level: {} ", name, level);
    println!("║ Health: {}/{} ", health, max_health);
    println!("╚════════════════════╝");
}

fn main() {
    display_stats("Hero", 5, 95, 100);
}
```

## Related Concepts

### Prerequisites
- Module 01: Basic Rust (strings, loops)
- Module 04: CLI basics
- Understanding of buffering concepts

### Follow-ups
- Game Loop Basics (integrating I/O)
- Simple Game Logic (processing input)
- Advanced formatting (rich terminal UI)

## Best Practices

1. **Always trim input** - Remove newlines
2. **Flush prompts** - Ensure prompts display
3. **Handle errors** - Don't panic on bad input
4. **Use stderr for errors** - Keep stdout clean
5. **Validate input** - Check before using
6. **Clear feedback** - Tell user what happened
7. **Clear formatting** - Make output readable

## Summary

Console I/O is the bridge between player and game. Proper input handling ensures responsive gameplay, while formatted output creates engaging experiences. Understanding buffering and string handling prevents common I/O bugs.

## Practice Exercise Ideas

1. Create an interactive story game
2. Build a menu-driven character creator
3. Implement a turn-based combat system
4. Create a number guessing game with scoring
5. Build a simple text adventure interface

