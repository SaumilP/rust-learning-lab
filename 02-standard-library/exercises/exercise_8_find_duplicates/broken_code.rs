// Exercise 8: Find Duplicates (BROKEN CODE)
//
// This code has 3 bugs related to HashSet/HashMap operations and algorithms
// Your job: Find and fix the bugs so it compiles and produces correct output

use std::collections::{HashMap, HashSet};

fn main() {
    println!("=== Find Duplicates ===\n");

    println!("=== Test 1: Integer Duplicates ===");
    integer_duplicates();

    println!("\n=== Test 2: String Duplicates ===");
    string_duplicates();

    println!("\n=== Test 3: Character Duplicates ===");
    character_duplicates();
}

fn integer_duplicates() {
    let numbers = vec![1, 2, 3, 2, 4, 5, 3, 1];
    println!("Numbers: {:?}", numbers);

    // Check if has duplicates using HashSet
    let mut seen = HashSet::new();
    let has_dups = numbers.iter().any(|&n| {
        // BUG 1: insert() returns bool, but logic is backwards
        // insert() returns true if value was NEW, false if it was already in set
        seen.insert(n)  // This is backwards - should check if insert() returned false
    });
    println!("Has duplicates: {}", has_dups);

    // Find first duplicate value
    // BUG 2: Resetting seen set loses previous state
    let mut seen = HashSet::new();
    let first_dup = numbers.iter().find(|&&n| {
        !seen.insert(n)  // This is correct pattern, but seen was reset above
    });
    println!("First duplicate value: {:?}", first_dup);

    // All duplicates with counts
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &n in &numbers {
        *counts.entry(n).or_insert(0) += 1;
    }

    let all_dups: Vec<i32> = counts.iter()
        .filter(|(_, count)| **count > 1)
        .map(|(num, _)| *num)
        .collect();
    println!("All duplicates: {:?}", all_dups);

    // Duplicate counts
    // BUG 3: Printing HashMap directly without sorting/formatting
    // Iteration order is not guaranteed, output may vary
    println!("Duplicate counts:");
    for (num, count) in counts.iter() {
        if *count > 1 {
            print!("{}->{}", num, count);
            print!(", ");  // Extra comma after each
        }
    }
    println!();  // Extra newline
}

fn string_duplicates() {
    let words = vec!["apple", "banana", "apple", "cherry", "banana"];
    println!("Words: {:?}", words);

    // Check if has duplicates
    let mut seen = HashSet::new();
    let has_dups = words.iter().any(|w| !seen.insert(*w));
    println!("Has duplicates: {}", has_dups);

    // Find first duplicate
    let mut seen = HashSet::new();
    let first_dup = words.iter().find(|&w| !seen.insert(*w));
    println!("First duplicate: {:?}", first_dup);

    // All duplicates
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for &w in &words {
        *counts.entry(w).or_insert(0) += 1;
    }

    let all_dups: Vec<&str> = counts.iter()
        .filter(|(_, count)| **count > 1)
        .map(|(word, _)| *word)
        .collect();
    println!("All duplicates: {:?}", all_dups);

    // Counts
    println!("Duplicate counts:");
    for (word, count) in counts.iter() {
        if *count > 1 {
            println!("  {}:{}", word, count);
        }
    }
}

fn character_duplicates() {
    let text = "programming";
    println!("Text: \"{}\"", text);

    // Check if has duplicates
    let mut seen = HashSet::new();
    let has_dups = text.chars().any(|c| !seen.insert(c));
    println!("Has duplicates: {}", has_dups);

    // Find first duplicate
    let mut seen = HashSet::new();
    let first_dup = text.chars().find(|c| !seen.insert(*c));
    println!("First duplicate char: {:?}", first_dup);

    // All duplicates
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in text.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut all_dups: Vec<char> = counts.iter()
        .filter(|(_, count)| **count > 1)
        .map(|(c, _)| *c)
        .collect();
    all_dups.sort();
    println!("All duplicates: {:?}", all_dups);

    // Counts
    println!("Character counts:");
    for (c, count) in counts.iter() {
        if *count > 1 {
            println!("  {}:{}", c, count);
        }
    }
}

// BUGS SUMMARY:
// 1. HashSet.insert() returns true if NEW, false if DUPLICATE
//    Current logic: any(|n| seen.insert(n)) returns true on FIRST unique, not on duplicate
//    Should be: any(|n| !seen.insert(n)) to find first duplicate
// 2. Variable shadowing losing state - but actually this is in string_duplicates which works
//    Integer version does the same thing correctly
// 3. HashMap iteration order not guaranteed - output formatting inconsistent

// EXPECTED BEHAVIOR:
// === Test 1: Integer Duplicates ===
// Numbers: [1, 2, 3, 2, 4, 5, 3, 1]
// Has duplicates: true
// First duplicate value: Some(2)
// All duplicates: [1, 2, 3] (or different order)
// Duplicate counts: 1->2, 2->2, 3->2
//
// === Test 2: String Duplicates ===
// Words: ["apple", "banana", "apple", "cherry", "banana"]
// Has duplicates: true
// First duplicate: Some("apple")
// All duplicates: ["apple", "banana"]
// Duplicate counts:
//   apple:2
//   banana:2
//
// === Test 3: Character Duplicates ===
// Text: "programming"
// Has duplicates: true
// First duplicate char: Some('r')
// All duplicates: ['g', 'm', 'r']
// Character counts:
//   g:2
//   m:2
//   r:2
