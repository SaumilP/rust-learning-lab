// Exercise 2: File Search Tool (BROKEN CODE)
//
// This code has 3 bugs related to argument parsing, flag handling, and search logic
// Your job: Find and fix the bugs so it works correctly

use std::env;
use std::fs;

fn main() {
    println!("=== Search Results ===\n");

    let args: Vec<String> = env::args().collect();

    // BUG 1: Not validating argument count
    // Will panic if not enough arguments provided
    let pattern = &args[1];
    let filename = &args[2];

    // BUG 2: Flag checking not working correctly
    // Flags might be in different positions, not just at end
    let ignore_case = args[3] == "--ignore-case" || args[4] == "--ignore-case";
    let whole_word = args[3] == "--whole-word" || args[4] == "--whole-word";

    print!("Pattern: \"{}\"", pattern);
    print!("\nFile: {}", filename);

    if ignore_case {
        print!(" (case-insensitive)");
    }
    if whole_word {
        print!(" (whole-word)");
    }
    println!("\n");

    // Try to read file
    match fs::read_to_string(filename) {
        Ok(contents) => {
            let mut match_count = 0;

            // Search through lines
            for (line_num, line) in contents.lines().enumerate() {
                let matches = if ignore_case {
                    // BUG 3: Comparison logic is wrong
                    // Should compare lowercase versions
                    line.contains(&pattern.to_lowercase())
                } else {
                    line.contains(pattern)
                };

                if matches {
                    // Apply whole-word check
                    let should_print = if whole_word {
                        // Only print if pattern is whole word
                        line.split_whitespace().any(|word| word == pattern)
                    } else {
                        true
                    };

                    if should_print {
                        println!("{}: {}", line_num + 1, line);
                        match_count += 1;
                    }
                }
            }

            println!("\nMatches found: {}", match_count);
        }
        Err(e) => {
            eprintln!("Error: Cannot read file '{}': {}", filename, e);
            std::process::exit(1);
        }
    }
}

// BUGS SUMMARY:
// 1. No argument count validation - directly accesses args[1], args[2], args[3], args[4]
// 2. Flag checking assumes fixed positions - args[3] and args[4] might not exist or be flags
// 3. Case-insensitive comparison wrong - comparing lowercase pattern to original line

// EXPECTED BEHAVIOR AFTER FIXES:
// === Search Results ===
//
// Pattern: "fox"
// File: text.txt
//
// Matches found: 2
// 1: The quick brown fox
// 3: the fox runs away
