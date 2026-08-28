// Example: String Methods - split, trim, to_uppercase, etc.
//
// Demonstrates:
// - Common string manipulation methods
// - Transformations: uppercase, lowercase, etc.
// - Whitespace handling: trim, split_whitespace
// - Substring operations

fn main() {
    println!("=== Case Transformations ===\n");

    let text = "Hello, World!";
    println!("Original: '{}'", text);

    // to_uppercase() - convert to uppercase
    let upper = text.to_uppercase();
    println!("to_uppercase(): '{}'", upper);

    // to_lowercase() - convert to lowercase
    let lower = text.to_lowercase();
    println!("to_lowercase(): '{}'", lower);

    // Works with Unicode
    let german = "Gru\u{00DF}"; // "Gru?" (German)
    println!("\nGerman text: '{}'", german);
    println!("to_uppercase(): '{}'", german.to_uppercase());

    println!("\n=== Trimming Whitespace ===\n");

    let messy = "   Hello, World!   \n";
    println!("Original: '{}'", messy);

    // trim() - remove leading and trailing whitespace
    let trimmed = messy.trim();
    println!("trim(): '{}'", trimmed);

    // trim_start() / trim_end()
    let start_trimmed = messy.trim_start();
    println!("trim_start(): '{}'", start_trimmed);

    let end_trimmed = messy.trim_end();
    println!("trim_end(): '{}'", end_trimmed);

    // Trim specific characters
    let text = "###Hello###";
    let trimmed = text.trim_matches('#');
    println!("\n'{}' trim_matches('#'): '{}'", text, trimmed);

    println!("\n=== Splitting Strings ===\n");

    // split() - split by delimiter
    let csv = "apple,banana,cherry,date";
    println!("CSV: '{}'", csv);
    println!("split(','):");
    for item in csv.split(',') {
        println!("  '{}'", item);
    }

    // split_whitespace() - split by any whitespace
    let text = "Hello   World\tRust\nProgramming";
    println!("\nText with mixed whitespace: {:?}", text);
    println!("split_whitespace():");
    for word in text.split_whitespace() {
        println!("  '{}'", word);
    }

    // splitn() - split at most n times
    let path = "/home/user/documents/file.txt";
    println!("\nPath: '{}'", path);
    println!("splitn(3, '/'):");
    for part in path.splitn(3, '/') {
        println!("  '{}'", part);
    }

    // lines() - split by newlines
    let multiline = "Line 1\nLine 2\nLine 3";
    println!("\nMultiline text:");
    for (i, line) in multiline.lines().enumerate() {
        println!("  {}: '{}'", i + 1, line);
    }

    println!("\n=== Finding and Checking ===\n");

    let text = "Hello, World! Hello, Rust!";
    println!("Text: '{}'", text);

    // contains() - check if substring exists
    println!("contains('World'): {}", text.contains("World"));
    println!("contains('Python'): {}", text.contains("Python"));

    // starts_with() / ends_with()
    println!("starts_with('Hello'): {}", text.starts_with("Hello"));
    println!("ends_with('!'): {}", text.ends_with("!"));

    // find() - find position of substring
    println!("find('World'): {:?}", text.find("World"));
    println!("find('Missing'): {:?}", text.find("Missing"));

    // rfind() - find from the end
    println!("rfind('Hello'): {:?}", text.rfind("Hello"));

    println!("\n=== Replacing ===\n");

    let text = "Hello, World! Hello, Rust!";
    println!("Original: '{}'", text);

    // replace() - replace all occurrences
    let replaced = text.replace("Hello", "Hi");
    println!("replace('Hello', 'Hi'): '{}'", replaced);

    // replacen() - replace first n occurrences
    let replaced = text.replacen("Hello", "Hi", 1);
    println!("replacen('Hello', 'Hi', 1): '{}'", replaced);

    // Replace characters
    let kebab = "hello-world-rust";
    let snake = kebab.replace('-', "_");
    println!("'{}' -> '{}'", kebab, snake);

    println!("\n=== String Length and Characters ===\n");

    let text = "Hello";
    println!("Text: '{}'", text);

    // len() - byte length
    println!("len() (bytes): {}", text.len());

    // chars().count() - character count
    println!("chars().count(): {}", text.chars().count());

    // Unicode example
    let emoji = "Hello";
    println!("\nEmoji text: '{}'", emoji);
    println!("len() (bytes): {}", emoji.len());
    println!("chars().count(): {}", emoji.chars().count());

    // is_empty()
    let empty = "";
    println!("\nEmpty string is_empty(): {}", empty.is_empty());
    println!("'hello' is_empty(): {}", "hello".is_empty());

    println!("\n=== Repeating ===\n");

    let pattern = "ab";
    let repeated = pattern.repeat(5);
    println!("'{}' repeated 5 times: '{}'", pattern, repeated);

    let line = "-".repeat(20);
    println!("Separator: {}", line);

    println!("\n=== Concatenation ===\n");

    // Using + operator (takes ownership of first string)
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 is moved here
    println!("Concatenated: '{}'", s3);
    // println!("{}", s1);  // ERROR: s1 was moved

    // Using format! macro (doesn't take ownership)
    let greeting = "Hello";
    let name = "World";
    let message = format!("{}, {}!", greeting, name);
    println!("Format: '{}'", message);

    // push_str() for mutable strings
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("push_str result: '{}'", s);

    println!("\n=== Character Operations ===\n");

    let text = "Hello, World!";
    println!("Text: '{}'", text);

    // Get first character
    if let Some(first) = text.chars().next() {
        println!("First character: '{}'", first);
    }

    // Get last character
    if let Some(last) = text.chars().last() {
        println!("Last character: '{}'", last);
    }

    // Get nth character
    if let Some(fifth) = text.chars().nth(4) {
        println!("5th character: '{}'", fifth);
    }

    // Iterate over characters
    println!("Characters:");
    for (i, c) in text.chars().enumerate() {
        if i > 4 {
            break;
        }
        println!("  [{}] = '{}'", i, c);
    }

    println!("\n=== Practical Examples ===\n");

    // Example 1: Clean user input
    fn clean_input(input: &str) -> String {
        input.trim().to_lowercase()
    }
    let user_input = "  HELLO WORLD  ";
    println!("Raw input: '{}'", user_input);
    println!("Clean input: '{}'", clean_input(user_input));

    // Example 2: Word count
    fn word_count(text: &str) -> usize {
        text.split_whitespace().count()
    }
    let text = "The quick brown fox jumps over the lazy dog";
    println!("\nWord count in '{}': {}", text, word_count(text));

    // Example 3: Capitalize first letter
    fn capitalize_first(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().to_string() + chars.as_str(),
        }
    }
    println!("capitalize_first('hello'): '{}'", capitalize_first("hello"));

    // Example 4: Title case
    fn title_case(s: &str) -> String {
        s.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().to_string() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    println!("title_case('hello world'): '{}'", title_case("hello world"));
}
