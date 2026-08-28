// Example: Basic Command-Line Argument Handling
//
// Demonstrates:
// - Accessing command-line arguments
// - Collecting arguments into Vec
// - Safe argument access patterns
// - Usage messages and error handling
//
// Run with: cargo run -- arg1 arg2 arg3

use std::env;

fn main() {
    println!("=== Command-Line Arguments Example ===\n");

    // Get all arguments as a Vec<String>
    let args: Vec<String> = env::args().collect();

    // Print program name
    println!("Program name: {}", args[0]);

    // Show total argument count
    println!("Total arguments: {}", args.len() - 1); // -1 to exclude program name

    // Show all arguments
    println!("\nAll arguments:");
    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            println!("  [{}] Program: {}", i, arg);
        } else {
            println!("  [{}] Argument: {}", i, arg);
        }
    }

    // Safe access pattern - check before accessing
    if args.len() > 1 {
        println!("\nFirst argument: {}", args[1]);
    } else {
        println!("\nNo arguments provided");
    }

    // Access multiple arguments safely
    if args.len() > 2 {
        println!("Second argument: {}", args[2]);
    }

    // Iterate through user arguments (skip program name)
    println!("\nProcessing user arguments:");
    for (index, arg) in args.iter().enumerate().skip(1) {
        println!("  Arg {}: {}", index, arg);
    }

    // Count arguments by type
    let user_args = &args[1..];
    let flag_count = user_args.iter().filter(|a| a.starts_with("--")).count();
    let file_count = user_args.iter().filter(|a| !a.starts_with("-")).count();

    println!("\nArgument analysis:");
    println!("  Flags (--): {}", flag_count);
    println!("  Files/values: {}", file_count);

    // Check for specific flags
    let verbose = args.contains(&"--verbose".to_string());
    let quiet = args.contains(&"--quiet".to_string());

    if verbose {
        println!("  Verbose mode: ENABLED");
    }
    if quiet {
        println!("  Quiet mode: ENABLED");
    }
    if !verbose && !quiet {
        println!("  No flags provided");
    }
}

// Example usage:
// cargo run -- file.txt
// cargo run -- --verbose data.csv
// cargo run -- --quiet input.txt output.txt
// cargo run -- --verbose --output result.txt source.txt

// Expected output:
// === Command-Line Arguments Example ===
//
// Program name: basic_args
// Total arguments: 3
//
// All arguments:
//   [0] Program: /path/to/basic_args
//   [1] Argument: file.txt
//   [2] Argument: arg2
//   [3] Argument: arg3
//
// First argument: file.txt
// Second argument: arg2
//
// Processing user arguments:
//   Arg 1: file.txt
//   Arg 2: arg2
//   Arg 3: arg3
//
// Argument analysis:
//   Flags (--): 0
//   Files/values: 3
//   No flags provided
