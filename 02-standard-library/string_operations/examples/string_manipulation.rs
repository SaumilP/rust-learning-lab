// Example: String Manipulation - chars, bytes, slicing
//
// Demonstrates:
// - Iterating over characters and bytes
// - String slicing and indexing
// - Building strings character by character
// - Unicode handling

fn main() {
    println!("=== chars() - Character Iterator ===\n");

    let text = "Hello";
    println!("Text: '{}'", text);

    // Iterate over characters
    print!("chars(): ");
    for c in text.chars() {
        print!("'{}' ", c);
    }
    println!();

    // Collect characters into a vector
    let chars: Vec<char> = text.chars().collect();
    println!("As vector: {:?}", chars);

    // Count characters
    println!("Character count: {}", text.chars().count());

    // Unicode example
    let emoji = "Hi";
    println!("\nUnicode text: '{}'", emoji);
    print!("chars(): ");
    for c in emoji.chars() {
        print!("'{}' ", c);
    }
    println!();
    println!("Character count: {}", emoji.chars().count());
    println!("Byte length: {}", emoji.len());

    println!("\n=== bytes() - Byte Iterator ===\n");

    let text = "Hello";
    println!("Text: '{}'", text);

    // Iterate over bytes
    print!("bytes(): ");
    for b in text.bytes() {
        print!("{} ", b);
    }
    println!();

    // As hexadecimal
    print!("bytes (hex): ");
    for b in text.bytes() {
        print!("{:02x} ", b);
    }
    println!();

    // Collect bytes
    let bytes: Vec<u8> = text.bytes().collect();
    println!("As vector: {:?}", bytes);

    // Unicode bytes (multi-byte characters)
    let unicode = "Hola";
    println!("\nUnicode '{}' bytes:", unicode);
    for b in unicode.bytes() {
        print!("{:02x} ", b);
    }
    println!();
    println!("Byte count: {}", unicode.len());
    println!("Char count: {}", unicode.chars().count());

    println!("\n=== char_indices() ===\n");

    let text = "Hello";
    println!("Text: '{}'", text);

    // char_indices gives (byte_index, char)
    println!("char_indices():");
    for (i, c) in text.char_indices() {
        println!("  byte {} -> '{}'", i, c);
    }

    println!("\n=== String Slicing ===\n");

    let text = "Hello, World!";
    println!("Text: '{}'", text);

    // Slice by byte indices (must be valid UTF-8 boundaries!)
    let hello = &text[0..5];
    println!("&text[0..5] = '{}'", hello);

    let world = &text[7..12];
    println!("&text[7..12] = '{}'", world);

    // From start
    let start = &text[..5];
    println!("&text[..5] = '{}'", start);

    // To end
    let end = &text[7..];
    println!("&text[7..] = '{}'", end);

    // Careful with Unicode! Must slice at char boundaries
    let safe_unicode = "Hola";
    // &safe_unicode[0..2] would panic! 'o' is a multi-byte character

    // Safe way to get first n characters
    fn first_n_chars(s: &str, n: usize) -> &str {
        let end = s.char_indices().nth(n).map(|(i, _)| i).unwrap_or(s.len());
        &s[..end]
    }

    println!("\nSafe Unicode slicing:");
    println!(
        "first_n_chars('{}', 4) = '{}'",
        safe_unicode,
        first_n_chars(safe_unicode, 4)
    );

    println!("\n=== Building Strings ===\n");

    // Using String::new() and push
    let mut s = String::new();
    s.push('H');
    s.push('e');
    s.push('l');
    s.push('l');
    s.push('o');
    println!("Built with push: '{}'", s);

    // Using push_str
    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("Built with push_str: '{}'", s);

    // Using extend
    let mut s = String::new();
    s.extend(['H', 'e', 'l', 'l', 'o']);
    println!("Built with extend: '{}'", s);

    // From iterator
    let chars = ['R', 'u', 's', 't'];
    let s: String = chars.iter().collect();
    println!("Built from iterator: '{}'", s);

    println!("\n=== Character Access ===\n");

    let text = "Hello";

    // Get first character
    let first = text.chars().next();
    println!("First char: {:?}", first);

    // Get last character
    let last = text.chars().last();
    println!("Last char: {:?}", last);

    // Get nth character
    let third = text.chars().nth(2);
    println!("Third char (index 2): {:?}", third);

    // Check if empty
    println!("Is empty: {}", text.chars().next().is_none());

    println!("\n=== Modifying Characters ===\n");

    let text = "Hello";
    println!("Original: '{}'", text);

    // Reverse a string
    let reversed: String = text.chars().rev().collect();
    println!("Reversed: '{}'", reversed);

    // Filter characters
    let only_consonants: String = text
        .chars()
        .filter(|c| !"aeiouAEIOU".contains(*c))
        .collect();
    println!("Consonants only: '{}'", only_consonants);

    // Map characters
    let shifted: String = text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                ((c as u8 + 1 - b'A') % 26 + b'A') as char
            } else {
                c
            }
        })
        .collect();
    println!("Caesar cipher (+1): '{}'", shifted);

    println!("\n=== String and Bytes Conversion ===\n");

    // String to bytes
    let text = "Hello";
    let bytes = text.as_bytes();
    println!("'{}' as bytes: {:?}", text, bytes);

    // Bytes to String (fallible - must be valid UTF-8)
    let bytes = vec![72, 101, 108, 108, 111];
    match String::from_utf8(bytes.clone()) {
        Ok(s) => println!("Bytes {:?} as string: '{}'", bytes, s),
        Err(e) => println!("Invalid UTF-8: {}", e),
    }

    // Lossy conversion (replaces invalid bytes)
    let invalid_bytes = vec![72, 101, 0xFF, 108, 111];
    let lossy = String::from_utf8_lossy(&invalid_bytes);
    println!("Lossy conversion: '{}'", lossy);

    println!("\n=== Working with Lines ===\n");

    let multiline = "Line 1\nLine 2\nLine 3";
    println!("Multi-line text:");
    println!("---");
    println!("{}", multiline);
    println!("---");

    println!("\nProcessing lines:");
    for (i, line) in multiline.lines().enumerate() {
        println!("{}: '{}'", i + 1, line);
    }

    // Join lines back
    let joined = multiline.lines().collect::<Vec<_>>().join(" | ");
    println!("Joined: '{}'", joined);

    println!("\n=== Practical Examples ===\n");

    // Example 1: Check if palindrome
    fn is_palindrome(s: &str) -> bool {
        let cleaned: String = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        cleaned == cleaned.chars().rev().collect::<String>()
    }

    println!("Is 'racecar' palindrome? {}", is_palindrome("racecar"));
    println!(
        "Is 'A man a plan a canal Panama' palindrome? {}",
        is_palindrome("A man a plan a canal Panama")
    );

    // Example 2: Count vowels
    fn count_vowels(s: &str) -> usize {
        s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
    }

    println!("\nVowels in 'Hello World': {}", count_vowels("Hello World"));

    // Example 3: Truncate with ellipsis
    fn truncate(s: &str, max_chars: usize) -> String {
        if s.chars().count() <= max_chars {
            s.to_string()
        } else {
            let truncated: String = s.chars().take(max_chars - 3).collect();
            format!("{}...", truncated)
        }
    }

    let long_text = "This is a very long string that needs truncation";
    println!("Truncated to 20: '{}'", truncate(long_text, 20));

    // Example 4: Extract initials
    fn initials(name: &str) -> String {
        name.split_whitespace()
            .filter_map(|word| word.chars().next())
            .map(|c| c.to_ascii_uppercase())
            .collect::<String>()
    }

    println!(
        "Initials of 'John Doe Smith': '{}'",
        initials("John Doe Smith")
    );
}
