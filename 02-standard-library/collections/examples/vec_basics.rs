// Example: Vec Basics - Creation, push, pop, indexing
//
// Demonstrates:
// - Creating vectors in different ways
// - push() and pop() operations
// - Indexing and safe access with get()
// - Basic vector properties (len, capacity, is_empty)

fn main() {
    println!("=== Creating Vectors ===\n");

    // Method 1: Vec::new() - creates an empty vector
    let v1: Vec<i32> = Vec::new();
    println!("Vec::new(): {:?}", v1);

    // Method 2: vec![] macro - creates a vector with initial values
    let v2 = vec![1, 2, 3, 4, 5];
    println!("vec![1, 2, 3, 4, 5]: {:?}", v2);

    // Method 3: Vec::with_capacity() - preallocate memory
    let v3: Vec<i32> = Vec::with_capacity(10);
    println!(
        "Vec::with_capacity(10): {:?}, capacity: {}",
        v3,
        v3.capacity()
    );

    // Method 4: From a range
    let v4: Vec<i32> = (1..=5).collect();
    println!("(1..=5).collect(): {:?}", v4);

    // Method 5: Using vec! with repeated values
    let v5 = vec![0; 5]; // Five zeros
    println!("vec![0; 5]: {:?}", v5);

    println!("\n=== push() - Adding Elements ===\n");

    let mut numbers = Vec::new();
    println!("Initial empty vector: {:?}", numbers);

    // push() adds an element to the end
    numbers.push(10);
    println!("After push(10): {:?}", numbers);

    numbers.push(20);
    println!("After push(20): {:?}", numbers);

    numbers.push(30);
    println!("After push(30): {:?}", numbers);

    // Adding multiple elements
    for i in 40..=60 {
        if i % 10 == 0 {
            numbers.push(i);
        }
    }
    println!("After pushing 40, 50, 60: {:?}", numbers);

    println!("\n=== pop() - Removing Elements ===\n");

    let mut stack = vec![1, 2, 3, 4, 5];
    println!("Initial vector: {:?}", stack);

    // pop() removes and returns the last element
    let popped = stack.pop();
    println!("pop() returned: {:?}, vector is now: {:?}", popped, stack);

    let popped = stack.pop();
    println!("pop() returned: {:?}, vector is now: {:?}", popped, stack);

    // pop() returns None on empty vector
    let mut empty: Vec<i32> = Vec::new();
    let result = empty.pop();
    println!("pop() on empty vector: {:?}", result);

    println!("\n=== Indexing - Direct Access ===\n");

    let colors = vec!["red", "green", "blue", "yellow", "purple"];
    println!("colors: {:?}", colors);

    // Direct indexing (panics if out of bounds)
    println!("colors[0] = '{}'", colors[0]);
    println!("colors[2] = '{}'", colors[2]);
    println!("colors[4] = '{}'", colors[4]);

    // Access last element
    let last_index = colors.len() - 1;
    println!("colors[{}] (last) = '{}'", last_index, colors[last_index]);

    // Careful! This would panic:
    // println!("{}", colors[10]);  // Index out of bounds

    println!("\n=== get() - Safe Access ===\n");

    let numbers = vec![10, 20, 30, 40, 50];
    println!("numbers: {:?}", numbers);

    // get() returns Option<&T>, never panics
    match numbers.get(2) {
        Some(value) => println!("numbers.get(2) = Some({})", value),
        None => println!("numbers.get(2) = None"),
    }

    match numbers.get(10) {
        Some(value) => println!("numbers.get(10) = Some({})", value),
        None => println!("numbers.get(10) = None (out of bounds)"),
    }

    // Using if let for concise access
    if let Some(first) = numbers.get(0) {
        println!("First element: {}", first);
    }

    // Using unwrap_or for default value
    let value = numbers.get(100).unwrap_or(&0);
    println!("numbers.get(100).unwrap_or(&0) = {}", value);

    println!("\n=== first() and last() ===\n");

    let items = vec!["apple", "banana", "cherry"];
    println!("items: {:?}", items);

    println!("first(): {:?}", items.first());
    println!("last(): {:?}", items.last());

    let empty: Vec<i32> = vec![];
    println!("empty.first(): {:?}", empty.first());
    println!("empty.last(): {:?}", empty.last());

    println!("\n=== len(), is_empty(), capacity() ===\n");

    let v = vec![1, 2, 3, 4, 5];
    println!("v: {:?}", v);
    println!("v.len() = {} (number of elements)", v.len());
    println!("v.is_empty() = {} (true if len == 0)", v.is_empty());
    println!("v.capacity() = {} (allocated space)", v.capacity());

    let empty: Vec<i32> = Vec::new();
    println!("\nempty vector:");
    println!("empty.len() = {}", empty.len());
    println!("empty.is_empty() = {}", empty.is_empty());

    println!("\n=== Modifying Elements ===\n");

    let mut data = vec![1, 2, 3, 4, 5];
    println!("Initial: {:?}", data);

    // Modify by index
    data[0] = 100;
    println!("After data[0] = 100: {:?}", data);

    data[2] = 300;
    println!("After data[2] = 300: {:?}", data);

    // Modify last element
    let last = data.len() - 1;
    data[last] = 999;
    println!("After data[last] = 999: {:?}", data);

    println!("\n=== insert() and remove() ===\n");

    let mut letters = vec!['a', 'b', 'd', 'e'];
    println!("Initial: {:?}", letters);

    // insert at index
    letters.insert(2, 'c'); // Insert 'c' at index 2
    println!("After insert(2, 'c'): {:?}", letters);

    // remove at index
    let removed = letters.remove(4); // Remove 'e'
    println!("After remove(4): {:?}, removed: '{}'", letters, removed);

    println!("\n=== clear() and truncate() ===\n");

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Initial: {:?}", v);

    // truncate() keeps first n elements
    v.truncate(5);
    println!("After truncate(5): {:?}", v);

    // clear() removes all elements
    v.clear();
    println!("After clear(): {:?}", v);
    println!("is_empty after clear: {}", v.is_empty());

    println!("\n=== Common Patterns ===\n");

    // Pattern 1: Build a vector from computation
    let mut squares: Vec<i32> = Vec::new();
    for i in 1..=5 {
        squares.push(i * i);
    }
    println!("Squares: {:?}", squares);

    // Pattern 2: Use vector as a stack (LIFO)
    let mut stack = vec![];
    stack.push("first");
    stack.push("second");
    stack.push("third");
    println!("Stack: {:?}", stack);
    println!("Popping: {:?}", stack.pop());
    println!("Popping: {:?}", stack.pop());
    println!("Stack after pops: {:?}", stack);

    // Pattern 3: Check before access
    let data = vec![1, 2, 3];
    let index = 5;
    if index < data.len() {
        println!("data[{}] = {}", index, data[index]);
    } else {
        println!("Index {} is out of bounds (len = {})", index, data.len());
    }
}
