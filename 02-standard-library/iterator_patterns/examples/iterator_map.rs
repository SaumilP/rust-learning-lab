// Example: Using the map() Iterator Method
//
// Demonstrates:
// - Basic map() transformations
// - Chaining map() with other adapters
// - Common map() patterns and use cases
// - map() vs for_each()

fn main() {
    println!("=== Basic map() Usage ===\n");

    let numbers = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", numbers);

    // Double each number
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // Square each number
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squared: {:?}", squared);

    // Add 10 to each
    let plus_ten: Vec<i32> = numbers.iter().map(|x| x + 10).collect();
    println!("Plus 10: {:?}", plus_ten);

    println!("\n=== map() with String Transformations ===\n");

    let words = vec!["hello", "world", "rust"];
    println!("Original: {:?}", words);

    // Convert to uppercase
    let upper: Vec<String> = words.iter().map(|s| s.to_uppercase()).collect();
    println!("Uppercase: {:?}", upper);

    // Get lengths
    let lengths: Vec<usize> = words.iter().map(|s| s.len()).collect();
    println!("Lengths: {:?}", lengths);

    // Add prefix
    let prefixed: Vec<String> = words.iter().map(|s| format!("prefix_{}", s)).collect();
    println!("Prefixed: {:?}", prefixed);

    println!("\n=== map() Type Conversions ===\n");

    let numbers = vec![1, 2, 3, 4, 5];

    // i32 to f64
    let floats: Vec<f64> = numbers.iter().map(|&x| x as f64).collect();
    println!("As f64: {:?}", floats);

    // Numbers to strings
    let strings: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();
    println!("As strings: {:?}", strings);

    // Parse strings to numbers
    let number_strings = vec!["1", "2", "3", "4", "5"];
    let parsed: Vec<i32> = number_strings.iter().map(|s| s.parse().unwrap()).collect();
    println!("Parsed: {:?}", parsed);

    println!("\n=== map() with Structs ===\n");

    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
    }

    let people = vec![
        Person {
            name: String::from("Alice"),
            age: 30,
        },
        Person {
            name: String::from("Bob"),
            age: 25,
        },
        Person {
            name: String::from("Charlie"),
            age: 35,
        },
    ];

    // Extract just names
    let names: Vec<&String> = people.iter().map(|p| &p.name).collect();
    println!("Names: {:?}", names);

    // Extract ages
    let ages: Vec<u32> = people.iter().map(|p| p.age).collect();
    println!("Ages: {:?}", ages);

    // Transform to tuple
    let tuples: Vec<(&String, u32)> = people.iter().map(|p| (&p.name, p.age)).collect();
    println!("As tuples: {:?}", tuples);

    // Create new struct from existing
    #[allow(dead_code)]
    #[derive(Debug)]
    struct PersonInfo {
        info: String,
    }

    let infos: Vec<PersonInfo> = people
        .iter()
        .map(|p| PersonInfo {
            info: format!("{} is {} years old", p.name, p.age),
        })
        .collect();
    println!("Person infos: {:?}", infos);

    println!("\n=== Chaining map() with Other Adapters ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", numbers);

    // Filter then map
    let even_squared: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Even numbers squared: {:?}", even_squared);

    // Map then filter
    let doubled_over_10: Vec<i32> = numbers.iter().map(|x| x * 2).filter(|&x| x > 10).collect();
    println!("Doubled, then > 10: {:?}", doubled_over_10);

    // Multiple maps
    let result: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2) // Double
        .map(|x| x + 1) // Add 1
        .map(|x| x * x) // Square
        .take(5) // First 5
        .collect();
    println!("Double, +1, square, take 5: {:?}", result);

    println!("\n=== map() with Closures Capturing Variables ===\n");

    let multiplier = 3;
    let numbers = vec![1, 2, 3, 4, 5];

    let multiplied: Vec<i32> = numbers.iter().map(|x| x * multiplier).collect();
    println!("Multiplied by {}: {:?}", multiplier, multiplied);

    let offset = 100;
    let prefix = "item_";
    let items: Vec<String> = numbers
        .iter()
        .map(|x| format!("{}{}", prefix, x + offset))
        .collect();
    println!("With offset and prefix: {:?}", items);

    println!("\n=== map() vs for_each() ===\n");

    let numbers = vec![1, 2, 3, 4, 5];

    // map() is for transforming - it's lazy and returns an iterator
    println!("Using map() (transform and collect):");
    let _doubled: Vec<i32> = numbers
        .iter()
        .map(|x| {
            let result = x * 2;
            println!("  Processing {}", x);
            result
        })
        .collect();

    // for_each() is for side effects - it consumes immediately
    println!("\nUsing for_each() (just side effects):");
    numbers.iter().for_each(|x| {
        println!("  Processing {}", x);
    });

    println!("\n=== Lazy Evaluation of map() ===\n");

    let numbers = vec![1, 2, 3, 4, 5];

    // Iterator is created but not executed
    let _iter = numbers.iter().map(|x| {
        println!("This won't print yet: {}", x);
        x * 2
    });

    println!("Iterator created - nothing executed yet");
    println!("(Notice no output from the map closure above)");

    // Only executes when consumed
    println!("\nNow collecting (executing):");
    let result: Vec<i32> = numbers
        .iter()
        .map(|x| {
            println!("  Now processing: {}", x);
            x * 2
        })
        .collect();
    println!("Result: {:?}", result);

    println!("\n=== map() with Index (using enumerate) ===\n");

    let letters = vec!['a', 'b', 'c', 'd', 'e'];

    let indexed: Vec<(usize, char)> = letters.iter().enumerate().map(|(i, &c)| (i, c)).collect();
    println!("With indices: {:?}", indexed);

    let formatted: Vec<String> = letters
        .iter()
        .enumerate()
        .map(|(i, c)| format!("{}. {}", i + 1, c))
        .collect();
    println!("Formatted list: {:?}", formatted);

    println!("\n=== Practical Examples ===\n");

    // Example 1: Temperature conversion
    let celsius = vec![0.0, 20.0, 37.0, 100.0];
    let fahrenheit: Vec<f64> = celsius.iter().map(|c| c * 9.0 / 5.0 + 32.0).collect();
    println!("Celsius: {:?}", celsius);
    println!("Fahrenheit: {:?}", fahrenheit);

    // Example 2: Clean and normalize data
    let raw_inputs = vec!["  HELLO  ", "World", "  RUST  "];
    let cleaned: Vec<String> = raw_inputs.iter().map(|s| s.trim().to_lowercase()).collect();
    println!("\nRaw: {:?}", raw_inputs);
    println!("Cleaned: {:?}", cleaned);

    // Example 3: Calculate derived values
    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }

    let rectangles = vec![
        Rectangle {
            width: 3.0,
            height: 4.0,
        },
        Rectangle {
            width: 5.0,
            height: 2.0,
        },
        Rectangle {
            width: 7.0,
            height: 3.0,
        },
    ];

    let areas: Vec<f64> = rectangles.iter().map(|r| r.width * r.height).collect();
    println!("\nRectangles: {:?}", rectangles);
    println!("Areas: {:?}", areas);
}
