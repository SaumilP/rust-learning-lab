// Example: Text Processing and Manipulation
//
// Demonstrates:
// - String splitting and joining
// - Case conversion
// - Pattern matching in text
// - Line processing
// - Word frequency counting
// - Character filtering

use std::collections::HashMap;

fn main() {
    println!("=== Text Processing Examples ===\n");

    // Example 1: Basic String Methods
    println!("1. BASIC STRING METHODS");
    let text = "Hello, World!";
    println!("   Original: {}", text);
    println!("   Uppercase: {}", text.to_uppercase());
    println!("   Lowercase: {}", text.to_lowercase());
    println!("   Length: {} characters", text.chars().count());
    println!("   Byte size: {} bytes", text.len());

    // Example 2: String Searching
    println!("\n2. STRING SEARCHING");
    let sentence = "The quick brown fox jumps over the lazy dog";
    println!("   Text: {}", sentence);

    println!("   Contains 'quick': {}", sentence.contains("quick"));
    println!("   Contains 'cat': {}", sentence.contains("cat"));
    println!("   Starts with 'The': {}", sentence.starts_with("The"));
    println!("   Ends with 'dog': {}", sentence.ends_with("dog"));

    if let Some(pos) = sentence.find("fox") {
        println!("   'fox' found at position: {}", pos);
    }

    // Example 3: Trimming
    println!("\n3. TRIMMING WHITESPACE");
    let messy = "  hello world  ";
    println!("   Original: |{}|", messy);
    println!("   Trimmed: |{}|", messy.trim());
    println!("   Trim start: |{}|", messy.trim_start());
    println!("   Trim end: |{}|", messy.trim_end());

    // Example 4: Splitting by Delimiter
    println!("\n4. SPLITTING TEXT");
    let csv = "apple,banana,cherry,date";
    println!("   Original: {}", csv);

    let items: Vec<&str> = csv.split(',').collect();
    println!("   Split by comma: {:?}", items);

    for (i, item) in items.iter().enumerate() {
        println!("   [{}] {}", i, item);
    }

    // Example 5: Splitting by Whitespace
    println!("\n5. SPLITTING BY WHITESPACE");
    let text = "The quick brown fox";
    println!("   Original: {}", text);

    let words: Vec<&str> = text.split_whitespace().collect();
    println!("   Words: {:?}", words);
    println!("   Word count: {}", words.len());

    // Example 6: Line Processing
    println!("\n6. PROCESSING LINES");
    let multiline = "Line one\nLine two\nLine three";
    println!("   Text:\n{}\n", multiline);

    for (line_num, line) in multiline.lines().enumerate() {
        println!("   [{}] {}", line_num + 1, line);
    }

    // Example 7: Character Iteration
    println!("\n7. CHARACTER ITERATION");
    let text = "Rust";
    print!("   Characters: ");
    for ch in text.chars() {
        print!("[{}] ", ch);
    }
    println!();

    // Example 8: String Replacement
    println!("\n8. STRING REPLACEMENT");
    let original = "The cat sat on the mat";
    println!("   Original: {}", original);

    let replaced = original.replace("cat", "dog");
    println!("   Replace 'cat' with 'dog': {}", replaced);

    let replaced_all = original.replace("at", "ot");
    println!("   Replace 'at' with 'ot': {}", replaced_all);

    // Example 9: Word Frequency Counter
    println!("\n9. WORD FREQUENCY");
    let text = "the quick brown fox jumps over the lazy dog and the fox runs";
    println!("   Text: {}", text);

    let frequencies = count_words(text);
    println!("   Frequencies:");
    for (word, count) in frequencies.iter() {
        println!("     {}: {}", word, count);
    }

    // Example 10: Filtering Characters
    println!("\n10. FILTERING CHARACTERS");
    let text = "Hello, World! 2024";
    println!("   Original: {}", text);

    let alphanumeric: String = text
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect();
    println!("   Alphanumeric only: {}", alphanumeric);

    let letters_only: String = text.chars().filter(|c| c.is_alphabetic()).collect();
    println!("   Letters only: {}", letters_only);

    // Example 11: Case Conversion Patterns
    println!("\n11. CASE CONVERSIONS");
    let text = "Hello World";
    println!("   Original: {}", text);
    println!("   UPPERCASE: {}", text.to_uppercase());
    println!("   lowercase: {}", text.to_lowercase());

    // Title case (capitalize each word)
    let title_case: String = text
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    println!("   Title Case: {}", title_case);

    // Example 12: Line Filtering
    println!("\n12. FILTERING LINES");
    let text = "line one\n\nline three\nline four\n\nline six";
    println!("   Original (with empty lines):");
    for line in text.lines() {
        println!("   |{}|", line);
    }

    println!("\n   Non-empty lines:");
    let non_empty: Vec<&str> = text.lines().filter(|line| !line.is_empty()).collect();
    for line in non_empty {
        println!("   {}", line);
    }

    println!("\nText processing complete!");
}

// Helper function: Count word frequencies
fn count_words(text: &str) -> HashMap<String, usize> {
    let mut frequencies = HashMap::new();

    for word in text.split_whitespace() {
        let word: String = word
            .to_lowercase()
            .chars()
            .filter(|character| character.is_alphabetic())
            .collect();

        if !word.is_empty() {
            *frequencies.entry(word).or_insert(0) += 1;
        }
    }

    frequencies
}

// Expected output:
// === Text Processing Examples ===
//
// 1. BASIC STRING METHODS
//    Original: Hello, World!
//    Uppercase: HELLO, WORLD!
//    Lowercase: hello, world!
//    Length: 13 characters
//    Byte size: 13 bytes
//
// 2. STRING SEARCHING
//    Text: The quick brown fox jumps over the lazy dog
//    Contains 'quick': true
//    Contains 'cat': false
//    Starts with 'The': true
//    Ends with 'dog': true
//    'fox' found at position: 16
//
// 3. TRIMMING WHITESPACE
//    Original: |  hello world  |
//    Trimmed: |hello world|
//    Trim start: |hello world  |
//    Trim end: |  hello world|
//
// ... (more output)
//
// Text processing complete!
