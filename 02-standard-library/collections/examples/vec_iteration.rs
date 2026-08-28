// Example: Iterating Over Vectors
//
// Demonstrates:
// - Iterating with for loops
// - iter(), iter_mut(), into_iter()
// - enumerate() for index + value
// - Iterator adapters and collecting

fn main() {
    println!("=== Basic for Loop Iteration ===\n");

    let numbers = vec![1, 2, 3, 4, 5];

    // Simple iteration (borrows the vector)
    println!("Iterating over {:?}:", numbers);
    for num in &numbers {
        println!("  {}", num);
    }
    // numbers is still usable here
    println!("Vector still usable: {:?}", numbers);

    println!("\n=== iter() - Immutable References ===\n");

    let fruits = vec!["apple", "banana", "cherry"];

    // iter() returns an iterator over &T (references)
    println!("Using iter():");
    for fruit in fruits.iter() {
        println!("  {} (length: {})", fruit, fruit.len());
    }
    println!("fruits is still owned: {:?}", fruits);

    println!("\n=== iter_mut() - Mutable References ===\n");

    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Before modification: {:?}", numbers);

    // iter_mut() returns an iterator over &mut T
    for num in numbers.iter_mut() {
        *num *= 2; // Double each element
    }
    println!("After doubling: {:?}", numbers);

    // Another example: increment all elements
    let mut scores = vec![85, 90, 78, 92, 88];
    println!("\nScores before bonus: {:?}", scores);
    for score in scores.iter_mut() {
        *score += 5; // Add bonus points
    }
    println!("Scores after +5 bonus: {:?}", scores);

    println!("\n=== into_iter() - Consuming the Vector ===\n");

    let names = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
    ];
    println!("Before into_iter(): {:?}", names);

    // into_iter() consumes the vector, returning owned values
    for name in names.into_iter() {
        println!("  Processing: {}", name);
    }
    // names is no longer usable - it was consumed
    // println!("{:?}", names);  // ERROR: value borrowed after move

    println!("\n=== enumerate() - Index and Value ===\n");

    let letters = vec!['a', 'b', 'c', 'd', 'e'];

    // enumerate() adds an index to each item
    println!("Using enumerate():");
    for (index, letter) in letters.iter().enumerate() {
        println!("  [{}] = '{}'", index, letter);
    }

    // Finding index of specific element
    let target = 'c';
    for (i, &letter) in letters.iter().enumerate() {
        if letter == target {
            println!("Found '{}' at index {}", target, i);
            break;
        }
    }

    println!("\n=== Reverse Iteration ===\n");

    let countdown = vec![5, 4, 3, 2, 1];

    println!("Countdown:");
    for num in countdown.iter().rev() {
        println!("  {}...", num);
    }
    println!("  Liftoff!");

    println!("\n=== Iteration with zip() ===\n");

    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];

    // zip() combines two iterators into pairs
    println!("People and ages:");
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("  {} is {} years old", name, age);
    }

    println!("\n=== Using Iterator Adapters ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("numbers: {:?}", numbers);

    // filter() - keep elements matching a condition
    println!("\nEven numbers (filter):");
    for num in numbers.iter().filter(|&&x| x % 2 == 0) {
        print!("{} ", num);
    }
    println!();

    // map() - transform each element
    println!("\nSquares (map):");
    for square in numbers.iter().map(|x| x * x) {
        print!("{} ", square);
    }
    println!();

    // Chaining: filter then map
    println!("\nSquares of even numbers:");
    for value in numbers.iter().filter(|&&x| x % 2 == 0).map(|x| x * x) {
        print!("{} ", value);
    }
    println!();

    println!("\n=== Collecting Iterator Results ===\n");

    let original = vec![1, 2, 3, 4, 5];

    // Collect doubled values into new vector
    let doubled: Vec<i32> = original.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", original);
    println!("Doubled:  {:?}", doubled);

    // Filter and collect
    let evens: Vec<&i32> = original.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Evens:    {:?}", evens);

    // Transform strings
    let words = vec!["hello", "world", "rust"];
    let upper: Vec<String> = words.iter().map(|s| s.to_uppercase()).collect();
    println!("Uppercase: {:?}", upper);

    println!("\n=== Aggregation Methods ===\n");

    let numbers = vec![1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);

    // sum() - add all elements
    let total: i32 = numbers.iter().sum();
    println!("sum(): {}", total);

    // product() - multiply all elements
    let product: i32 = numbers.iter().product();
    println!("product(): {}", product);

    // count() - count elements
    let count = numbers.iter().count();
    println!("count(): {}", count);

    // min() and max()
    println!("min(): {:?}", numbers.iter().min());
    println!("max(): {:?}", numbers.iter().max());

    println!("\n=== find() and position() ===\n");

    let values = vec![10, 20, 30, 40, 50];
    println!("values: {:?}", values);

    // find() - first element matching condition
    let found = values.iter().find(|&&x| x > 25);
    println!("First > 25: {:?}", found);

    // position() - index of first match
    let pos = values.iter().position(|&x| x == 30);
    println!("Position of 30: {:?}", pos);

    println!("\n=== any() and all() ===\n");

    let numbers = vec![2, 4, 6, 8, 10];
    println!("numbers: {:?}", numbers);

    // any() - true if any element matches
    let has_odd = numbers.iter().any(|&x| x % 2 != 0);
    println!("any odd? {}", has_odd);

    // all() - true if all elements match
    let all_even = numbers.iter().all(|&x| x % 2 == 0);
    println!("all even? {}", all_even);

    println!("\n=== take() and skip() ===\n");

    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("items: {:?}", items);

    // take() - first n elements
    let first_three: Vec<&i32> = items.iter().take(3).collect();
    println!("take(3): {:?}", first_three);

    // skip() - skip first n elements
    let after_three: Vec<&i32> = items.iter().skip(3).collect();
    println!("skip(3): {:?}", after_three);

    // Combine: skip then take (pagination-like)
    let middle: Vec<&i32> = items.iter().skip(3).take(4).collect();
    println!("skip(3).take(4): {:?}", middle);

    println!("\n=== Practical Example: Processing Data ===\n");

    #[derive(Debug)]
    struct Student {
        name: String,
        score: i32,
    }

    let students = vec![
        Student {
            name: String::from("Alice"),
            score: 85,
        },
        Student {
            name: String::from("Bob"),
            score: 92,
        },
        Student {
            name: String::from("Charlie"),
            score: 78,
        },
        Student {
            name: String::from("Diana"),
            score: 95,
        },
    ];

    // Find students with score >= 90
    println!("High scorers (>= 90):");
    for student in students.iter().filter(|s| s.score >= 90) {
        println!("  {} - {}", student.name, student.score);
    }

    // Calculate average
    let total: i32 = students.iter().map(|s| s.score).sum();
    let average = total as f64 / students.len() as f64;
    println!("Average score: {:.1}", average);

    // Get just the names
    let names: Vec<&String> = students.iter().map(|s| &s.name).collect();
    println!("All students: {:?}", names);
}
