// Example: HashSet Creation and Operations
//
// Demonstrates:
// - Creating HashSets
// - insert(), remove(), contains()
// - Set operations: union, intersection, difference
// - Practical uses for unique collections

use std::collections::HashSet;

fn main() {
    println!("=== Creating HashSets ===\n");

    // Method 1: HashSet::new() - empty set
    let set1: HashSet<i32> = HashSet::new();
    println!("Empty HashSet: {:?}", set1);

    // Method 2: Create and insert
    let mut fruits = HashSet::new();
    fruits.insert("apple");
    fruits.insert("banana");
    fruits.insert("cherry");
    println!("HashSet with inserts: {:?}", fruits);

    // Method 3: From an array
    let numbers: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    println!("HashSet from array: {:?}", numbers);

    // Method 4: From a vector
    let vec = vec!["a", "b", "c", "d"];
    let set: HashSet<_> = vec.into_iter().collect();
    println!("HashSet from vec: {:?}", set);

    // Method 5: With capacity
    let _with_capacity: HashSet<String> = HashSet::with_capacity(100);
    println!("Created HashSet with capacity 100");

    println!("\n=== insert() - Adding Elements ===\n");

    let mut colors = HashSet::new();

    // insert() returns true if the value was not present
    let inserted = colors.insert("red");
    println!("Inserted 'red': {}", inserted);

    let inserted = colors.insert("green");
    println!("Inserted 'green': {}", inserted);

    let inserted = colors.insert("blue");
    println!("Inserted 'blue': {}", inserted);

    // Trying to insert duplicate returns false
    let inserted = colors.insert("red");
    println!("Inserted 'red' again: {} (already exists)", inserted);

    println!("Colors: {:?}", colors);

    println!("\n=== Uniqueness - Automatic Deduplication ===\n");

    // HashSet automatically removes duplicates
    let numbers_with_dups = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5];
    let unique: HashSet<i32> = numbers_with_dups.iter().cloned().collect();
    println!("Original with duplicates: {:?}", numbers_with_dups);
    println!("Unique set: {:?}", unique);

    // Counting unique elements
    let text = "hello world hello rust hello world";
    let words: HashSet<&str> = text.split_whitespace().collect();
    println!("Text: '{}'", text);
    println!("Unique words: {:?}", words);
    println!("Number of unique words: {}", words.len());

    println!("\n=== contains() - Checking Membership ===\n");

    let valid_extensions: HashSet<&str> = ["jpg", "png", "gif", "webp"].iter().cloned().collect();
    println!("Valid extensions: {:?}", valid_extensions);

    let files = vec!["photo.jpg", "document.pdf", "image.png", "video.mp4"];
    for file in files {
        let ext = file.split('.').last().unwrap_or("");
        if valid_extensions.contains(ext) {
            println!("  '{}' - valid image", file);
        } else {
            println!("  '{}' - not an image", file);
        }
    }

    println!("\n=== remove() - Removing Elements ===\n");

    let mut tasks: HashSet<&str> = ["task1", "task2", "task3", "task4"]
        .iter()
        .cloned()
        .collect();
    println!("Tasks: {:?}", tasks);

    // remove() returns true if element was present
    let removed = tasks.remove("task2");
    println!("Removed 'task2': {}", removed);
    println!("Tasks after remove: {:?}", tasks);

    // Removing non-existent element
    let removed = tasks.remove("task5");
    println!("Removed 'task5': {}", removed);

    // Clear all elements
    tasks.clear();
    println!("After clear: {:?}", tasks);

    println!("\n=== Set Operations: Union ===\n");

    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();
    println!("Set A: {:?}", set_a);
    println!("Set B: {:?}", set_b);

    // union() - all elements in either set
    let union: HashSet<&i32> = set_a.union(&set_b).collect();
    println!("A union B: {:?}", union);

    // Alternative: create owned union
    let union: HashSet<i32> = set_a.union(&set_b).cloned().collect();
    println!("A union B (owned): {:?}", union);

    println!("\n=== Set Operations: Intersection ===\n");

    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();
    println!("Set A: {:?}", set_a);
    println!("Set B: {:?}", set_b);

    // intersection() - elements in both sets
    let intersection: HashSet<&i32> = set_a.intersection(&set_b).collect();
    println!("A intersection B: {:?}", intersection);

    // Check if sets have common elements
    let has_common = !set_a.is_disjoint(&set_b);
    println!("Sets have common elements: {}", has_common);

    println!("\n=== Set Operations: Difference ===\n");

    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();
    println!("Set A: {:?}", set_a);
    println!("Set B: {:?}", set_b);

    // difference() - elements in A but not in B
    let diff_a: HashSet<&i32> = set_a.difference(&set_b).collect();
    println!("A - B (in A, not in B): {:?}", diff_a);

    // difference() - elements in B but not in A
    let diff_b: HashSet<&i32> = set_b.difference(&set_a).collect();
    println!("B - A (in B, not in A): {:?}", diff_b);

    println!("\n=== Set Operations: Symmetric Difference ===\n");

    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();

    // symmetric_difference() - elements in either set, but not both
    let sym_diff: HashSet<&i32> = set_a.symmetric_difference(&set_b).collect();
    println!("A symmetric_difference B: {:?}", sym_diff);

    println!("\n=== Subset and Superset ===\n");

    let all_numbers: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter().cloned().collect();
    let even_numbers: HashSet<i32> = [2, 4, 6, 8, 10].iter().cloned().collect();
    let small_evens: HashSet<i32> = [2, 4].iter().cloned().collect();

    println!("All:        {:?}", all_numbers);
    println!("Even:       {:?}", even_numbers);
    println!("Small even: {:?}", small_evens);

    // is_subset() - all elements of self are in other
    println!(
        "\nsmall_evens.is_subset(&even_numbers): {}",
        small_evens.is_subset(&even_numbers)
    );
    println!(
        "even_numbers.is_subset(&all_numbers): {}",
        even_numbers.is_subset(&all_numbers)
    );

    // is_superset() - self contains all elements of other
    println!(
        "all_numbers.is_superset(&even_numbers): {}",
        all_numbers.is_superset(&even_numbers)
    );
    println!(
        "even_numbers.is_superset(&small_evens): {}",
        even_numbers.is_superset(&small_evens)
    );

    // is_disjoint() - no common elements
    let odd_numbers: HashSet<i32> = [1, 3, 5, 7, 9].iter().cloned().collect();
    println!(
        "\neven_numbers.is_disjoint(&odd_numbers): {}",
        even_numbers.is_disjoint(&odd_numbers)
    );

    println!("\n=== Iterating Over HashSets ===\n");

    let numbers: HashSet<i32> = [5, 2, 8, 1, 9].iter().cloned().collect();

    println!("Iterating (unordered!):");
    for num in &numbers {
        println!("  {}", num);
    }

    // Note: HashSet iteration order is not guaranteed!
    println!("\nFor sorted output, collect to Vec and sort:");
    let mut sorted: Vec<&i32> = numbers.iter().collect();
    sorted.sort();
    println!("Sorted: {:?}", sorted);

    println!("\n=== Practical Examples ===\n");

    // Example 1: Track unique visitors
    let mut visitors = HashSet::new();
    let page_visits = vec!["user1", "user2", "user1", "user3", "user2", "user1"];

    for user in page_visits {
        visitors.insert(user);
    }
    println!("Unique visitors: {:?}", visitors);
    println!("Total unique: {}", visitors.len());

    // Example 2: Find common interests
    let alice_interests: HashSet<&str> = ["music", "sports", "coding", "reading"]
        .iter()
        .cloned()
        .collect();
    let bob_interests: HashSet<&str> = ["gaming", "coding", "movies", "reading"]
        .iter()
        .cloned()
        .collect();

    let common: HashSet<_> = alice_interests.intersection(&bob_interests).collect();
    println!("\nAlice's interests: {:?}", alice_interests);
    println!("Bob's interests: {:?}", bob_interests);
    println!("Common interests: {:?}", common);

    // Example 3: Remove banned words
    let text = vec!["hello", "world", "spam", "rust", "spam", "programming"];
    let banned: HashSet<&str> = ["spam", "banned"].iter().cloned().collect();

    let clean: Vec<&&str> = text.iter().filter(|w| !banned.contains(*w)).collect();
    println!("\nOriginal: {:?}", text);
    println!("Banned: {:?}", banned);
    println!("Clean: {:?}", clean);

    // Example 4: Check for duplicates
    fn has_duplicates<T: std::hash::Hash + Eq>(items: &[T]) -> bool {
        let mut seen = HashSet::new();
        for item in items {
            if !seen.insert(item) {
                return true; // Item already seen
            }
        }
        false
    }

    println!(
        "\n[1, 2, 3, 4, 5] has duplicates: {}",
        has_duplicates(&[1, 2, 3, 4, 5])
    );
    println!(
        "[1, 2, 3, 2, 5] has duplicates: {}",
        has_duplicates(&[1, 2, 3, 2, 5])
    );
}
