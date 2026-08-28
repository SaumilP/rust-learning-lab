// Example: Working with Vectors
//
// Demonstrates:
// - Vector creation
// - Adding/removing elements
// - Iteration
// - Vector methods

fn main() {
    println!("=== Creating Vectors ===\n");

    let v1: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v1);

    let v2 = vec![1, 2, 3, 4, 5];
    println!("Vector from macro: {:?}", v2);

    let v3 = (1..=5).collect::<Vec<i32>>();
    println!("Vector from range: {:?}", v3);

    println!("\n=== Modifying Vectors ===\n");

    let mut numbers = vec![1, 2, 3];
    println!("Initial: {:?}", numbers);

    numbers.push(4);
    println!("After push(4): {:?}", numbers);

    numbers.push(5);
    println!("After push(5): {:?}", numbers);

    let removed = numbers.pop();
    println!("After pop: {:?}, removed: {:?}", numbers, removed);

    println!("\n=== Vector Length and Capacity ===\n");

    let mut v = Vec::with_capacity(10);
    println!("Empty vector with capacity 10:");
    println!("  len: {}, capacity: {}", v.len(), v.capacity());

    v.push(1);
    v.push(2);
    println!("After 2 pushes:");
    println!("  len: {}, capacity: {}", v.len(), v.capacity());

    println!("\n=== Accessing Elements ===\n");

    let v = vec![10, 20, 30, 40, 50];
    println!("Vector: {:?}", v);

    println!("First element [0]: {}", v[0]);
    println!("Last element [4]: {}", v[4]);

    // Safe access with get
    match v.get(2) {
        Some(value) => println!("v[2] = {}", value),
        None => println!("Index out of bounds"),
    }

    match v.get(10) {
        Some(value) => println!("v[10] = {}", value),
        None => println!("Index 10 is out of bounds"),
    }

    println!("\n=== Iterating Over Vectors ===\n");

    let v = vec![1, 2, 3, 4, 5];

    // Immutable iteration
    println!("for item in &v:");
    for item in &v {
        println!("  {}", item);
    }

    // Mutable iteration
    println!("for item in &mut v (doubled):");
    let mut v = vec![1, 2, 3];
    for item in &mut v {
        *item *= 2;
    }
    println!("  {:?}", v);

    // Enumerate
    println!("with enumerate:");
    let v = vec!["a", "b", "c"];
    for (i, item) in v.iter().enumerate() {
        println!("  [{}]: {}", i, item);
    }

    println!("\n=== Vector Methods ===\n");

    let v = vec![3, 1, 4, 1, 5, 9];
    println!("Original: {:?}", v);
    println!("len: {}", v.len());
    println!("is_empty: {}", v.is_empty());
    println!("contains(4): {}", v.contains(&4));
    println!("contains(10): {}", v.contains(&10));

    println!("\n=== Searching in Vectors ===\n");

    let v = vec![10, 20, 30, 40, 50];
    if let Some(pos) = v.iter().position(|&x| x == 30) {
        println!("Found 30 at index {}", pos);
    }

    let first_even = v.iter().find(|&&x| x % 2 == 0);
    println!("First even: {:?}", first_even);

    println!("\n=== Modifying Elements ===\n");

    let mut v = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", v);

    v[2] = 30; // Direct access
    println!("After v[2] = 30: {:?}", v);

    v.insert(2, 25); // Insert at position
    println!("After insert(2, 25): {:?}", v);

    v.remove(3); // Remove at position
    println!("After remove(3): {:?}", v);

    v.clear(); // Remove all
    println!("After clear: {:?}", v);

    println!("\n=== Slicing Vectors ===\n");

    let v = vec![1, 2, 3, 4, 5];
    let slice1 = &v[1..3];
    let slice2 = &v[2..];
    let slice3 = &v[..4];

    println!("Original: {:?}", v);
    println!("[1..3]: {:?}", slice1);
    println!("[2..]: {:?}", slice2);
    println!("[..4]: {:?}", slice3);

    println!("\n=== Vector of Different Types ===\n");

    let mut numbers: Vec<i32> = vec![1, 2, 3];
    numbers.push(4);
    println!("i32 vector: {:?}", numbers);

    let mut texts: Vec<String> = vec!["Hello".to_string(), "World".to_string()];
    texts.push("Rust".to_string());
    println!("String vector: {:?}", texts);
}
