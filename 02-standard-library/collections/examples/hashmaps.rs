// Example: Working with HashMaps
//
// Demonstrates:
// - HashMap creation
// - Inserting and retrieving values
// - Entry API
// - Iteration
// - Common HashMap operations

use std::collections::HashMap;

fn main() {
    println!("=== Creating HashMaps ===\n");

    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 92);

    println!("Scores: {:?}", scores);

    println!("\n=== Inserting and Updating ===\n");

    let mut map = HashMap::new();
    map.insert("key1", "value1");
    println!("After first insert: {:?}", map);

    map.insert("key1", "updated"); // Overwrites
    println!("After update: {:?}", map);

    map.insert("key2", "value2");
    println!("After second insert: {:?}", map);

    println!("\n=== Retrieving Values ===\n");

    let map = [("a", 1), ("b", 2), ("c", 3)]
        .iter()
        .cloned()
        .collect::<HashMap<_, _>>();

    match map.get("b") {
        Some(value) => println!("Found 'b': {}", value),
        None => println!("'b' not found"),
    }

    match map.get("z") {
        Some(value) => println!("Found 'z': {}", value),
        None => println!("'z' not found"),
    }

    println!("\n=== Checking Existence ===\n");

    let map: HashMap<&str, i32> = [("x", 10), ("y", 20)].iter().cloned().collect();

    println!("contains_key('x'): {}", map.contains_key("x"));
    println!("contains_key('z'): {}", map.contains_key("z"));

    println!("\n=== Entry API (Efficient Updates) ===\n");

    let mut word_count: HashMap<&str, i32> = HashMap::new();

    for word in &["the", "quick", "brown", "fox", "the", "brown", "brown"] {
        *word_count.entry(word).or_insert(0) += 1;
    }

    println!("Word counts: {:?}", word_count);

    println!("\n=== Iterating Over HashMaps ===\n");

    let map: HashMap<&str, i32> = [("one", 1), ("two", 2), ("three", 3)]
        .iter()
        .cloned()
        .collect();

    println!("Iterating:");
    for (key, value) in &map {
        println!("  {} => {}", key, value);
    }

    println!("\n=== Mutable Iteration ===\n");

    let mut map: HashMap<&str, i32> = [("a", 1), ("b", 2), ("c", 3)].iter().cloned().collect();

    for (_, value) in &mut map {
        *value *= 2;
    }

    println!("After doubling values: {:?}", map);

    println!("\n=== HashMap Methods ===\n");

    let map: HashMap<i32, &str> = [(1, "one"), (2, "two"), (3, "three")]
        .iter()
        .cloned()
        .collect();

    println!("len: {}", map.len());
    println!("is_empty: {}", map.is_empty());

    let empty_map: HashMap<i32, i32> = HashMap::new();
    println!("empty_map.is_empty: {}", empty_map.is_empty());

    println!("\n=== Removing Entries ===\n");

    let mut map: HashMap<&str, i32> = [("a", 1), ("b", 2), ("c", 3)].iter().cloned().collect();

    println!("Original: {:?}", map);

    map.remove("b");
    println!("After remove('b'): {:?}", map);

    map.clear();
    println!("After clear: {:?}", map);

    println!("\n=== Creating from Vec ===\n");

    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map: HashMap<_, _> = pairs.into_iter().collect();
    println!("HashMap from pairs: {:?}", map);

    println!("\n=== String Keys ===\n");

    let mut users: HashMap<String, String> = HashMap::new();

    users.insert("alice".to_string(), "Alice Smith".to_string());
    users.insert("bob".to_string(), "Bob Jones".to_string());

    match users.get("alice") {
        Some(name) => println!("alice: {}", name),
        None => println!("alice not found"),
    }

    println!("\n=== Counting Occurrences ===\n");

    let text = "hello world hello rust hello";
    let words = text.split_whitespace();
    let mut counts: HashMap<&str, i32> = HashMap::new();

    for word in words {
        *counts.entry(word).or_insert(0) += 1;
    }

    println!("Word frequency: {:?}", counts);

    println!("\n=== HashMap of Vectors ===\n");

    let mut groups: HashMap<char, Vec<i32>> = HashMap::new();

    groups.entry('a').or_insert_with(Vec::new).push(1);
    groups.entry('b').or_insert_with(Vec::new).push(2);
    groups.entry('a').or_insert_with(Vec::new).push(3);

    println!("Groups: {:?}", groups);
}
