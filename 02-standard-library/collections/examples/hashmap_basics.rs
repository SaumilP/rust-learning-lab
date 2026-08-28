// Example: HashMap Creation and Operations
//
// Demonstrates:
// - Creating HashMaps
// - insert(), get(), remove()
// - Entry API for efficient updates
// - Checking existence with contains_key()
// - Iterating over key-value pairs

use std::collections::HashMap;

fn main() {
    println!("=== Creating HashMaps ===\n");

    // Method 1: HashMap::new() - empty map
    let scores: HashMap<String, i32> = HashMap::new();
    println!("Empty HashMap: {:?}", scores);

    // Method 2: Create and insert
    let mut ages = HashMap::new();
    ages.insert("Alice", 25);
    ages.insert("Bob", 30);
    ages.insert("Charlie", 35);
    println!("HashMap with inserts: {:?}", ages);

    // Method 3: From an array of tuples
    let map: HashMap<&str, i32> = [("one", 1), ("two", 2), ("three", 3)]
        .iter()
        .cloned()
        .collect();
    println!("HashMap from array: {:?}", map);

    // Method 4: Using collect() from iterator
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map: HashMap<_, _> = pairs.into_iter().collect();
    println!("HashMap from vec: {:?}", map);

    // Method 5: With capacity
    let _with_capacity: HashMap<String, i32> = HashMap::with_capacity(100);
    println!("Created HashMap with capacity 100");

    println!("\n=== insert() - Adding Key-Value Pairs ===\n");

    let mut inventory = HashMap::new();

    // Basic insert
    inventory.insert("apples", 50);
    inventory.insert("bananas", 30);
    inventory.insert("oranges", 25);
    println!("Initial inventory: {:?}", inventory);

    // Insert returns the old value if key existed
    let old = inventory.insert("apples", 100);
    println!("Updated apples, old value was: {:?}", old);
    println!("Updated inventory: {:?}", inventory);

    // Insert new key returns None
    let old = inventory.insert("grapes", 40);
    println!("Added grapes, old value was: {:?}", old);

    println!("\n=== get() - Retrieving Values ===\n");

    let mut scores = HashMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);
    println!("Scores: {:?}", scores);

    // get() returns Option<&V>
    let alice_score = scores.get("Alice");
    println!("Alice's score: {:?}", alice_score);

    // Using match with get()
    match scores.get("Bob") {
        Some(&score) => println!("Bob's score: {}", score),
        None => println!("Bob not found"),
    }

    // Key that doesn't exist
    let unknown = scores.get("David");
    println!("David's score: {:?}", unknown);

    // Using unwrap_or for default
    let score = scores.get("David").unwrap_or(&0);
    println!("David's score (with default): {}", score);

    // Using copied() to get the value instead of reference
    let value: Option<i32> = scores.get("Alice").copied();
    println!("Alice's score (copied): {:?}", value);

    println!("\n=== remove() - Deleting Entries ===\n");

    let mut map = HashMap::new();
    map.insert("x", 10);
    map.insert("y", 20);
    map.insert("z", 30);
    println!("Before remove: {:?}", map);

    // remove() returns the removed value
    let removed = map.remove("y");
    println!("Removed 'y', value was: {:?}", removed);
    println!("After remove: {:?}", map);

    // Removing non-existent key
    let removed = map.remove("w");
    println!("Removed 'w', value was: {:?}", removed);

    // Clear all entries
    map.clear();
    println!("After clear: {:?}", map);

    println!("\n=== contains_key() - Checking Existence ===\n");

    let mut settings = HashMap::new();
    settings.insert("volume", 75);
    settings.insert("brightness", 50);
    settings.insert("contrast", 60);
    println!("Settings: {:?}", settings);

    println!(
        "contains_key('volume'): {}",
        settings.contains_key("volume")
    );
    println!(
        "contains_key('resolution'): {}",
        settings.contains_key("resolution")
    );

    // Pattern: Check before get
    if settings.contains_key("brightness") {
        println!("Brightness is set to: {}", settings["brightness"]);
    }

    println!("\n=== Entry API - Efficient Updates ===\n");

    let mut word_count: HashMap<&str, i32> = HashMap::new();

    // or_insert() - insert if not present, return mutable reference
    let words = vec!["hello", "world", "hello", "rust", "hello", "world"];
    for word in words {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:?}", word_count);

    // or_insert_with() - use closure for default value
    let mut cache: HashMap<&str, Vec<i32>> = HashMap::new();
    cache.entry("data").or_insert_with(Vec::new).push(1);
    cache.entry("data").or_insert_with(Vec::new).push(2);
    cache.entry("other").or_insert_with(Vec::new).push(10);
    println!("Cache: {:?}", cache);

    // or_default() - use Default trait
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in "hello world".chars() {
        *counts.entry(c).or_default() += 1;
    }
    println!("Character counts: {:?}", counts);

    // and_modify() - modify if exists
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 90);
    scores.entry("Alice").and_modify(|s| *s += 5); // Add bonus
    scores.entry("Bob").and_modify(|s| *s += 5); // Does nothing (key doesn't exist)
    println!("Scores after bonus: {:?}", scores);

    // Combine and_modify with or_insert
    scores.entry("Bob").and_modify(|s| *s += 5).or_insert(85);
    println!("Scores with Bob added: {:?}", scores);

    println!("\n=== Iterating Over HashMaps ===\n");

    let capital_cities: HashMap<&str, &str> = [
        ("France", "Paris"),
        ("Japan", "Tokyo"),
        ("Brazil", "Brasilia"),
        ("India", "New Delhi"),
    ]
    .iter()
    .cloned()
    .collect();

    // Iterate over key-value pairs
    println!("Capital cities:");
    for (country, city) in &capital_cities {
        println!("  {} -> {}", country, city);
    }

    // Iterate over keys only
    println!("\nCountries:");
    for country in capital_cities.keys() {
        println!("  {}", country);
    }

    // Iterate over values only
    println!("\nCapitals:");
    for city in capital_cities.values() {
        println!("  {}", city);
    }

    println!("\n=== Mutable Iteration ===\n");

    let mut prices: HashMap<&str, f64> = [("apple", 1.50), ("banana", 0.75), ("orange", 1.25)]
        .iter()
        .cloned()
        .collect();
    println!("Prices before sale: {:?}", prices);

    // Modify all values
    for (_, price) in prices.iter_mut() {
        *price *= 0.9; // 10% discount
    }
    println!("Prices after 10% discount: {:?}", prices);

    println!("\n=== HashMap with String Keys ===\n");

    // Important: String ownership with HashMap
    let mut users: HashMap<String, String> = HashMap::new();

    let key = String::from("user1");
    let value = String::from("Alice Smith");
    users.insert(key, value);
    // key and value are now owned by the HashMap

    // Using &str to query
    match users.get("user1") {
        Some(name) => println!("user1: {}", name),
        None => println!("user1 not found"),
    }

    println!("\n=== Practical Examples ===\n");

    // Example 1: Group items by first letter
    let words = vec!["apple", "apricot", "banana", "blueberry", "cherry"];
    let mut grouped: HashMap<char, Vec<&str>> = HashMap::new();

    for word in &words {
        let first_char = word.chars().next().unwrap();
        grouped
            .entry(first_char)
            .or_insert_with(Vec::new)
            .push(word);
    }
    println!("Words grouped by first letter: {:?}", grouped);

    // Example 2: Frequency counter
    let text = "the quick brown fox jumps over the lazy dog";
    let mut freq: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    println!("Word frequency: {:?}", freq);

    // Example 3: Default scores
    let names = vec!["Alice", "Bob", "Charlie"];
    let mut scores: HashMap<&str, i32> = HashMap::new();
    for name in names {
        scores.entry(name).or_insert(100); // Default score
    }
    scores.entry("Alice").and_modify(|s| *s = 95); // Update Alice
    println!("Scores: {:?}", scores);
}
