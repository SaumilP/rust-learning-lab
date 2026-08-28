// Exercise 4: Find Duplicates in Vector (BROKEN CODE)
//
// This code has 4 bugs related to HashSet/HashMap operations
// Your job: Find and fix the bugs so it compiles and produces correct output

use std::collections::{HashMap, HashSet};

fn main() {
    println!("=== Integer Duplicates ===");
    integer_duplicates();

    println!("\n=== No Duplicates Case ===");
    no_duplicates_case();

    println!("\n=== String/Word Duplicates ===");
    string_duplicates();

    println!("\n=== Character Duplicates ===");
    character_duplicates();
}

fn integer_duplicates() {
    let numbers = vec![1, 2, 3, 2, 4, 5, 3, 1];
    println!("Numbers: {:?}", numbers);

    // Check if has duplicates
    let has_dups = has_duplicates(&numbers);
    println!("Has duplicates: {}", has_dups);

    // Find first duplicate
    let first_dup = find_first_duplicate(&numbers);
    println!("First duplicate: {:?}", first_dup);

    // Get all duplicates with counts
    println!("All duplicates with counts:");
    let dup_counts = get_duplicate_counts(&numbers);
    for (num, count) in &dup_counts {
        println!("  {} appears {} times", num, count);
    }

    // Get unique values
    let unique = get_unique_values(&numbers);
    println!("Unique values: {:?}", unique);
}

fn no_duplicates_case() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers: {:?}", numbers);
    println!("Has duplicates: {}", has_duplicates(&numbers));
    println!("First duplicate: {:?}", find_first_duplicate(&numbers));
}

fn string_duplicates() {
    let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
    println!("Words: {:?}", words);

    let has_dups = has_duplicates_generic(&words);
    println!("Has duplicates: {}", has_dups);

    let first_dup = find_first_duplicate_generic(&words);
    println!("First duplicate: {:?}", first_dup);

    println!("Duplicate counts:");
    let counts = get_duplicate_counts_generic(&words);
    for (word, count) in &counts {
        println!("  \"{}\" appears {} times", word, count);
    }
}

fn character_duplicates() {
    let text = "programming";
    println!("Text: \"{}\"", text);

    let chars: Vec<char> = text.chars().collect();
    println!("Has duplicate chars: {}", has_duplicates_generic(&chars));

    let first_dup_char = find_first_duplicate_generic(&chars);
    println!("First duplicate char: {:?}", first_dup_char);

    println!("Character counts for duplicates:");
    let char_counts = get_duplicate_counts_generic(&chars);
    for (ch, count) in &char_counts {
        println!("  '{}' appears {} times", ch, count);
    }
}

// BUG 1: HashSet::insert returns bool, not Result
// The function logic is correct but the return is inverted
fn has_duplicates(numbers: &[i32]) -> bool {
    let mut seen = HashSet::new();
    for &num in numbers {
        // insert() returns false if element was already present
        if seen.insert(num) == false {
            return false;  // Should return true when duplicate found!
        }
    }
    true  // Should return false when no duplicates
}

// BUG 2: The return logic is wrong
fn find_first_duplicate(numbers: &[i32]) -> Option<i32> {
    let mut seen = HashSet::new();
    for &num in numbers {
        if !seen.insert(num) {
            return None;  // Should return Some(num)!
        }
    }
    Some(0)  // Should return None when no duplicate found
}

fn get_duplicate_counts(numbers: &[i32]) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();
    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }
    // Keep only duplicates (count > 1)
    counts.into_iter().filter(|&(_, count)| count > 1).collect()
}

fn get_unique_values(numbers: &[i32]) -> Vec<i32> {
    let unique: HashSet<i32> = numbers.iter().cloned().collect();
    let mut result: Vec<i32> = unique.into_iter().collect();
    result.sort();
    result
}

// Generic version for any type that implements Eq + Hash + Clone
// BUG 3: Trait bounds are incomplete - need Hash trait
fn has_duplicates_generic<T: Eq + Clone>(items: &[T]) -> bool {
    let mut seen = HashSet::new();
    for item in items {
        if !seen.insert(item.clone()) {
            return true;
        }
    }
    false
}

// BUG 4: Return type doesn't match - returning reference but should return owned
fn find_first_duplicate_generic<T: Eq + std::hash::Hash + Clone>(items: &[T]) -> Option<T> {
    let mut seen = HashSet::new();
    for item in items {
        if !seen.insert(item.clone()) {
            return Some(&item);  // Returning reference, should be owned value
        }
    }
    None
}

fn get_duplicate_counts_generic<T: Eq + std::hash::Hash + Clone>(items: &[T]) -> HashMap<T, i32> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item.clone()).or_insert(0) += 1;
    }
    counts.into_iter().filter(|&(_, count)| count > 1).collect()
}

// BUGS SUMMARY:
// 1. has_duplicates: Return values are inverted
//    - return true when duplicate found (!seen.insert())
//    - return false at end when no duplicates
// 2. find_first_duplicate: Return values are inverted
//    - return Some(num) when duplicate found
//    - return None at end when no duplicates
// 3. has_duplicates_generic: Missing Hash trait bound
//    - HashSet requires elements to implement Hash
// 4. find_first_duplicate_generic: Return type mismatch
//    - Some(&item) returns Option<&T>, should be Option<T>
//    - Fix: Some(item.clone())

// EXPECTED OUTPUT:
// === Integer Duplicates ===
// Numbers: [1, 2, 3, 2, 4, 5, 3, 1]
// Has duplicates: true
// First duplicate: Some(2)
// All duplicates with counts:
//   1 appears 2 times
//   2 appears 2 times
//   3 appears 2 times
// Unique values: [1, 2, 3, 4, 5]
//
// === No Duplicates Case ===
// Numbers: [1, 2, 3, 4, 5]
// Has duplicates: false
// First duplicate: None
//
// === String/Word Duplicates ===
// Words: ["apple", "banana", "apple", "cherry", "banana", "apple"]
// Has duplicates: true
// First duplicate: Some("apple")
// Duplicate counts:
//   "apple" appears 3 times
//   "banana" appears 2 times
//
// === Character Duplicates ===
// Text: "programming"
// Has duplicate chars: true
// First duplicate char: Some('r')
// Character counts for duplicates:
//   'r' appears 2 times
//   'g' appears 2 times
//   'm' appears 2 times
