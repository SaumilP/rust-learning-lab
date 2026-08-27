/// Broken String Processor - Fix the bugs!
/// There are 3 bugs in this code

trait StringProcessor {
    fn process(&self, text: &str) -> String;
}

struct Uppercase;
struct Lowercase;
struct ReverseWords;
struct CharCounter;

impl StringProcessor for Uppercase {
    fn process(&self, text: &str) -> String {
        text.to_uppercase()  // ✓ Correct
    }
}

impl StringProcessor for Lowercase {
    fn process(&self, text: &str) -> String {
        text.to_lowercase()  // ✓ Correct
    }
}

impl StringProcessor for ReverseWords {
    fn process(&self, text: &str) -> String {
        // ❌ BUG 1: Should reverse order of words, not characters
        text.chars().rev().collect()
    }
}

impl StringProcessor for CharCounter {
    fn process(&self, text: &str) -> String {
        let letter_count = text.chars().filter(|c| c.is_alphabetic()).count();
        let space_count = text.chars().filter(|c| c.is_whitespace()).count();
        format!("Letters: {}, Spaces: {}", letter_count, space_count)
    }
}

fn main() {
    let text = "Hello World from Rust";

    println!("Original text: \"{}\"", text);
    println!("Length: {} characters\n", text.len());

    // ❌ BUG 2: These create unnecessary copies with .clone()
    // Should use references instead
    let upper = Uppercase.process(&text.clone());
    println!("Uppercase: {}", upper);

    let lower = Lowercase.process(&text.clone());
    println!("Lowercase: {}", lower);

    let reversed = ReverseWords.process(&text.clone());
    println!("Reverse words: {}", reversed);

    let counted = CharCounter.process(&text.clone());
    println!("Char counter: {}\n", counted);

    // ❌ BUG 3: Doesn't calculate statistics properly
    // Should compare before and after processing
    let stats = format!("Uppercase uses {} characters", upper.len());
    println!("Stats: {}", stats);
    println!("Note: Original text was {} chars", text.len());
}
