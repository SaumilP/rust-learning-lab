// Example: File Processing with CLI Arguments
//
// Demonstrates:
// - Argument validation
// - Error handling with exit codes
// - File operation with arguments
// - Usage message generation
//
// Run with: cargo run -- input.txt output.txt

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Validate minimum arguments
    if args.len() < 3 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    println!("=== File Processor ===\n");
    println!("Input file: {}", input_file);
    println!("Output file: {}", output_file);

    // Read input file
    match fs::read_to_string(input_file) {
        Ok(contents) => {
            println!("\nFile read successfully");
            println!("Original size: {} bytes", contents.len());

            // Process the content
            let processed = process_content(&contents);

            println!("Processed size: {} bytes", processed.len());

            // Write output file
            match fs::write(output_file, &processed) {
                Ok(_) => {
                    println!("\nFile written successfully to: {}", output_file);
                    println!("Processing complete!");
                }
                Err(e) => {
                    eprintln!("Error writing file: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    }

    // Optional: Check for flags
    let verbose = args.contains(&"--verbose".to_string());
    if verbose {
        println!("\n[Verbose] Processing complete with detailed output");
    }
}

fn process_content(content: &str) -> String {
    // Convert to uppercase and add line numbers
    content
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{:3}: {}", i + 1, line.to_uppercase()))
        .collect::<Vec<_>>()
        .join("\n")
}

fn print_usage(program_name: &str) {
    eprintln!(
        "Usage: {} <input_file> <output_file> [--verbose]",
        program_name
    );
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  input_file   - Path to input file to process");
    eprintln!("  output_file  - Path to output file");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --verbose    - Print detailed processing information");
}

// Example usage:
// First create a test file:
// echo "hello world" > input.txt
// echo "rust is great" >> input.txt
//
// Then run:
// cargo run -- input.txt output.txt
// cargo run -- input.txt output.txt --verbose
//
// Expected output:
// === File Processor ===
//
// Input file: input.txt
// Output file: output.txt
//
// File read successfully
// Original size: 27 bytes
// Processed size: 60 bytes
//
// File written successfully to: output.txt
// Processing complete!
