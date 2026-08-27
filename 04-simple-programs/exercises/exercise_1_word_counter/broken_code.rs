// Exercise 1: Word Counter CLI Tool (BROKEN CODE)
//
// This code has 3 bugs related to argument handling, file operations, and data processing
// Your job: Find and fix the bugs so it works correctly

use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    println!("=== Word Counter ===\n");

    // BUG 1: Not checking argument count before accessing
    // Will panic if no arguments provided
    let filename = &env::args().collect::<Vec<_>>()[1];

    // Try to read file
    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("File: {}", filename);

            // Count words
            let words: Vec<&str> = contents.split_whitespace().collect();
            println!("Total words: {}", words.len());

            // Count unique words and frequencies
            let mut frequencies = HashMap::new();

            for word in &words {
                let word_lower = word.to_lowercase();
                // BUG 2: Not handling punctuation removal
                // Words like "fox," and "fox" are counted separately
                *frequencies.entry(word_lower).or_insert(0) += 1;
            }

            println!("Unique words: {}", frequencies.len());

            // BUG 3: Not sorting before displaying top words
            // Displaying frequencies without sorting by count
            println!("\nTop 5 most common words:");
            let mut count = 0;
            for (word, freq) in frequencies.iter() {
                println!("{}. {} ({} occurrences)", count + 1, word, freq);
                count += 1;
                if count >= 5 {
                    break;
                }
            }
        }
        Err(e) => {
            eprintln!("Error: Cannot read file '{}': {}", filename, e);
            std::process::exit(1);
        }
    }
}

// BUGS SUMMARY:
// 1. No argument count check - accessing args[1] without verifying length
// 2. Punctuation not removed - "fox," and "fox" are different words
// 3. Words not sorted by frequency - just displays in HashMap order (not consistent)

// EXPECTED BEHAVIOR AFTER FIXES:
// === Word Counter ===
//
// File: sample.txt
// Total words: 16
// Unique words: 10
//
// Top 5 most common words:
// 1. the (3 occurrences)
// 2. fox (2 occurrences)
// 3. quick (1 occurrence)
// 4. brown (1 occurrence)
// 5. jumps (1 occurrence)
