// Exercise 4: Merge Sorted Files (BROKEN CODE)
//
// This code has 3 bugs related to file reading, merge algorithm, and comparison logic
// Your job: Find and fix the bugs so it works correctly

use std::env;
use std::fs;
use std::io::Write;

fn main() {
    println!("=== Merge Sorted Files ===\n");

    let args: Vec<String> = env::args().collect();

    // BUG 1: No argument validation
    // Will panic if not enough arguments
    let file1 = &args[1];
    let file2 = &args[2];
    let output = &args[3];

    println!("Input 1: {}", file1);
    println!("Input 2: {}", file2);
    println!("Output: {}", output);
    println!();

    // Read both files
    match (fs::read_to_string(file1), fs::read_to_string(file2)) {
        (Ok(content1), Ok(content2)) => {
            let lines1: Vec<&str> = content1.lines().collect();
            let lines2: Vec<&str> = content2.lines().collect();

            println!("Processing:");
            println!("  File 1 lines: {}", lines1.len());
            println!("  File 2 lines: {}", lines2.len());
            println!("  Total lines: {}", lines1.len() + lines2.len());
            println!();

            // BUG 2: Merge algorithm is wrong
            // Not correctly advancing pointers or comparing values
            let mut merged = Vec::new();
            let mut i = 0;
            let mut j = 0;

            while i < lines1.len() && j < lines2.len() {
                // BUG: Wrong comparison - should compare values, not just take from first
                // Current code always takes from first array
                if lines1[i] <= lines2[j] {
                    merged.push(lines1[i].to_string());
                    i += 1;
                } else {
                    merged.push(lines2[j].to_string());
                    j += 1;
                }
            }

            // BUG 3: Not adding remaining elements from both arrays
            // When one array is exhausted, must add rest of the other
            while i < lines1.len() {
                merged.push(lines1[i].to_string());
                // Missing i += 1
            }

            // Missing the second while loop for lines2
            while j < lines2.len() {
                merged.push(lines2[j].to_string());
                // Missing j += 1
            }

            // Write output
            match fs::File::create(output) {
                Ok(mut file) => {
                    for line in &merged {
                        let _ = writeln!(file, "{}", line);
                    }
                    println!("Merge complete!");
                    println!("Output written to: {}", output);
                }
                Err(e) => {
                    eprintln!("Error writing file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error reading input files");
            std::process::exit(1);
        }
    }
}

// BUGS SUMMARY:
// 1. No argument count validation - will panic if fewer than 4 arguments
// 2. Merge algorithm has correct comparison but missing loop logic
// 3. Not incrementing loop counter in remaining-elements loop
//    Infinite loop when adding remaining elements from arrays

// EXPECTED BEHAVIOR AFTER FIXES:
// === Merge Sorted Files ===
//
// Input 1: numbers1.txt
// Input 2: numbers2.txt
// Output: output.txt
//
// Processing:
//   File 1 lines: 5
//   File 2 lines: 5
//   Total lines: 10
//
// Merge complete!
// Output written to: output.txt
//
// Output file contains (sorted):
// 1
// 2
// 3
// 4
// 5
// 6
// 7
// 8
// 9
// 10
