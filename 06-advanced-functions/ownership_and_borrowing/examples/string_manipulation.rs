/// String Manipulation - Ownership and Borrowing Example
///
/// Demonstrates:
/// - Ownership transfer and borrowing patterns
/// - Mutable borrowing for modification
/// - Lifetimes with borrowed data
/// - Zero-copy string operations
///
/// Run with: cargo run --example string_manipulation

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_characters(text: &str) -> usize {
    text.chars().count()
}

fn to_uppercase_copy(text: &str) -> String {
    text.to_uppercase()
}

fn reverse_string(text: &mut String) {
    let chars: Vec<char> = text.chars().collect();
    text.clear();
    for ch in chars.iter().rev() {
        text.push(*ch);
    }
}

fn find_longest_word(text: &str) -> Option<&str> {
    text.split_whitespace()
        .max_by_key(|word| word.len())
}

fn word_frequency(text: &str) -> std::collections::HashMap<&str, usize> {
    use std::collections::HashMap;

    let mut freq = HashMap::new();
    for word in text.split_whitespace() {
        let word_lower = word.to_lowercase();
        *freq.entry(word_lower).or_insert(0) += 1;
    }

    // Note: This works with owned Strings in real code
    // For simplicity, we return with &str keys here
    text.split_whitespace()
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word).or_insert(0) += 1;
            map
        })
}

struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> Self {
        TextAnalyzer { text }
    }

    fn analyze(&self) {
        println!("\n╔═══════════════════════════════╗");
        println!("║ TEXT ANALYSIS REPORT          ║");
        println!("╠═══════════════════════════════╣");

        let words = count_words(self.text);
        let chars = count_characters(self.text);
        let lines = self.text.lines().count();

        println!("║ Characters: {:<18} ║", chars);
        println!("║ Words: {:<21} ║", words);
        println!("║ Lines: {:<21} ║", lines);

        if let Some(longest) = find_longest_word(self.text) {
            println!("║ Longest word: {:<16} ║", longest);
        }

        println!("╚═══════════════════════════════╝");
    }

    fn word_stats(&self) {
        println!("\n╔═══════════════════════════════╗");
        println!("║ WORD FREQUENCY                ║");
        println!("╠═══════════════════════════════╣");

        let mut freq = word_frequency(self.text);
        let mut words: Vec<_> = freq.iter().collect();
        words.sort_by_key(|(_, &count)| std::cmp::Reverse(count));

        for (word, count) in words.iter().take(10) {
            println!("║ {:<18}: {:<6} ║", word, count);
        }

        println!("╚═══════════════════════════════╝");
    }

    fn get_text(&self) -> &str {
        self.text
    }
}

fn demo_ownership_transfer() {
    println!("\n--- Ownership Transfer Demo ---");

    let s1 = String::from("Hello");
    println!("s1: {}", s1);

    // Ownership moved to s2
    let s2 = s1;
    println!("s2: {}", s2);
    // println!("s1: {}", s1);  // ERROR: s1 no longer owns the value

    // s1 is now invalid, but we can use s2
    println!("s2 length: {}", s2.len());
}

fn demo_borrowing() {
    println!("\n--- Borrowing Demo ---");

    let text = String::from("The quick brown fox jumps over the lazy dog");

    // Immutable borrow - s still owns the value
    let word_count = count_words(&text);
    println!("Text: {}", text);
    println!("Word count: {}", word_count);
    println!("Text still valid: {}", text);

    let char_count = count_characters(&text);
    println!("Character count: {}", char_count);
}

fn demo_mutable_borrowing() {
    println!("\n--- Mutable Borrowing Demo ---");

    let mut text = String::from("Hello World");
    println!("Original: {}", text);

    // Mutable borrow allows modification
    reverse_string(&mut text);
    println!("Reversed: {}", text);

    // Can borrow again after mutation
    let word_count = count_words(&text);
    println!("Word count: {}", word_count);
}

fn demo_cloning() {
    println!("\n--- Cloning Demo ---");

    let original = String::from("Original Text");
    println!("Original: {}", original);

    // Clone creates independent copy
    let copy = original.clone();
    println!("Clone: {}", copy);

    // Both are valid
    println!("Original still valid: {}", original);
    println!("Copy still valid: {}", copy);
}

fn main() {
    println!("╔════════════════════════════════╗");
    println!("║ OWNERSHIP & BORROWING DEMO     ║");
    println!("╚════════════════════════════════╝");

    demo_ownership_transfer();
    demo_borrowing();
    demo_mutable_borrowing();
    demo_cloning();

    // Text Analysis with borrowed data
    println!("\n--- Text Analysis Demo ---");

    let sample_text = "The Rust language emphasizes memory safety and \
        correctness. Rust allows you to catch many bugs at compile time, \
        before your program even runs. Rust is blazingly fast and memory-efficient.";

    let analyzer = TextAnalyzer::new(sample_text);
    analyzer.analyze();
    analyzer.word_stats();

    // Demonstrate lifetime
    println!("\n--- Lifetime Example ---");
    println!("Text (borrowed): {}", analyzer.get_text());
    println!("This shows that the analyzer holds a reference with lifetime 'a");
}
