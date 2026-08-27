// Exercise 3: Text Filter with CLI Arguments (BROKEN CODE)
//
// This code has 3 bugs related to argument parsing, filter logic, and file operations
// Your job: Find and fix the bugs so it works correctly

use std::env;
use std::fs;
use std::io::Write;

fn main() {
    println!("=== Text Filter ===\n");

    let args: Vec<String> = env::args().collect();

    // BUG 1: Not checking argument count
    // Will panic if not enough arguments
    let input_file = &args[1];
    let output_file = &args[2];

    // Parse filter arguments
    // BUG 2: Incorrect index calculations for flag values
    // Looking at wrong indices for flag values
    let mut min_length = 0;
    let mut contains_pattern = String::new();
    let mut exclude_pattern = String::new();

    for i in 3..args.len() {
        match args[i].as_str() {
            "--min-length" => {
                // BUG: Looking at wrong index for the value
                if i + 1 < args.len() {
                    min_length = args[i].parse::<usize>().unwrap_or(0);
                }
            }
            "--max-length" => {
                // Similar issue
                if i + 1 < args.len() {
                    // This is accessing the flag itself, not the value
                }
            }
            "--contains" => {
                if i + 1 < args.len() {
                    contains_pattern = args[i].clone();
                }
            }
            "--exclude" => {
                if i + 1 < args.len() {
                    exclude_pattern = args[i].clone();
                }
            }
            _ => {}
        }
    }

    // Display filter info
    println!("Input: {}", input_file);
    println!("Output: {}", output_file);
    print!("Filter: ");

    if min_length > 0 {
        print!("Minimum length {}", min_length);
    }
    if !contains_pattern.is_empty() {
        print!(", Contains \"{}\"", contains_pattern);
    }
    if !exclude_pattern.is_empty() {
        print!(", Exclude \"{}\"", exclude_pattern);
    }
    println!("\n");

    // Read and filter
    match fs::read_to_string(input_file) {
        Ok(contents) => {
            let mut filtered_lines = Vec::new();
            let mut total_lines = 0;

            for line in contents.lines() {
                total_lines += 1;

                // BUG 3: Filter logic is wrong - using OR instead of AND
                // All conditions should be AND (all must be true)
                let passes_min_length = min_length == 0 || line.len() >= min_length;
                let passes_contains = contains_pattern.is_empty() || line.contains(&contains_pattern);
                let passes_exclude = exclude_pattern.is_empty() || !line.contains(&exclude_pattern);

                // This logic applies filters as OR - should be AND
                if passes_min_length || passes_contains || passes_exclude {
                    filtered_lines.push(line.to_string());
                }
            }

            // Write output
            match fs::File::create(output_file) {
                Ok(mut file) => {
                    for line in &filtered_lines {
                        let _ = writeln!(file, "{}", line);
                    }

                    let removed = total_lines - filtered_lines.len();

                    println!("Processing complete:");
                    println!("  Total lines: {}", total_lines);
                    println!("  Filtered lines: {}", filtered_lines.len());
                    println!("  Removed lines: {}", removed);
                    println!("\nOutput written to: {}", output_file);
                }
                Err(e) => {
                    eprintln!("Error writing output: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}

// BUGS SUMMARY:
// 1. No argument count validation - directly accesses args[1], args[2], etc.
// 2. Incorrect flag value parsing - using args[i] instead of args[i+1] for values
// 3. Wrong filter logic - using OR (||) instead of AND (&&) for combining conditions

// EXPECTED BEHAVIOR AFTER FIXES:
// === Text Filter ===
//
// Input: lines.txt
// Output: output.txt
// Filter: Minimum length 5, Contains "g"
//
// Processing complete:
//   Total lines: 5
//   Filtered lines: 2
//   Removed lines: 3
//
// Output written to: output.txt
