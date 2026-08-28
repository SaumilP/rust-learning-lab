// Example: Chaining Multiple Iterator Operations
//
// Demonstrates:
// - Building complex iterator pipelines
// - Combining map, filter, and other adapters
// - Real-world data processing examples
// - Performance considerations

fn main() {
    println!("=== Basic Iterator Chains ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", numbers);

    // Chain: filter -> map -> collect
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0) // Keep evens
        .map(|x| x * 2) // Double them
        .collect();
    println!("Even numbers doubled: {:?}", result);

    // Chain: map -> filter -> map -> collect
    let result: Vec<String> = numbers
        .iter()
        .map(|x| x * x) // Square
        .filter(|&x| x > 20) // Keep > 20
        .map(|x| format!("value: {}", x)) // Format
        .collect();
    println!("Squares > 20 formatted: {:?}", result);

    println!("\n=== Multi-Step Data Pipeline ===\n");

    // Raw data processing pipeline
    let raw_data = vec![" 10 ", "20", " invalid ", "30", " 40 "];
    println!("Raw data: {:?}", raw_data);

    let processed: Vec<i32> = raw_data
        .iter()
        .map(|s| s.trim()) // Step 1: Trim whitespace
        .filter(|s| !s.is_empty()) // Step 2: Remove empty
        .filter_map(|s| s.parse::<i32>().ok()) // Step 3: Parse, skip errors
        .map(|n| n * 2) // Step 4: Transform
        .collect();
    println!("Processed data: {:?}", processed);

    println!("\n=== Chaining with enumerate ===\n");

    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    println!("Letters: {:?}", letters);

    let indexed: Vec<(usize, char)> = letters
        .iter()
        .enumerate() // Add indices
        .filter(|(i, _)| i % 2 == 0) // Keep even indices
        .map(|(i, &c)| (i, c.to_ascii_uppercase())) // Transform
        .collect();
    println!("Even indices, uppercase: {:?}", indexed);

    println!("\n=== Chaining with zip ===\n");

    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];
    println!("Names: {:?}", names);
    println!("Scores: {:?}", scores);

    let report: Vec<String> = names
        .iter()
        .zip(scores.iter()) // Pair them up
        .map(|(name, score)| format!("{}: {}", name, score))
        .collect();
    println!("Report: {:?}", report);

    // Filter zipped data
    let high_achievers: Vec<(&str, i32)> = names
        .iter()
        .zip(scores.iter())
        .filter(|(_, &score)| score >= 90)
        .map(|(&name, &score)| (name, score))
        .collect();
    println!("High achievers (>=90): {:?}", high_achievers);

    println!("\n=== Chaining with flat_map ===\n");

    let nested = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    println!("Nested: {:?}", nested);

    // Flatten and process
    let flattened: Vec<i32> = nested
        .iter()
        .flat_map(|v| v.iter()) // Flatten nested vecs
        .filter(|&&x| x % 2 == 0) // Keep evens
        .copied()
        .collect();
    println!("Flattened evens: {:?}", flattened);

    // Split strings and process words
    let sentences = vec!["hello world", "rust is great", "learn iterators"];
    println!("\nSentences: {:?}", sentences);

    let words: Vec<&str> = sentences
        .iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("All words: {:?}", words);

    let long_words: Vec<&str> = sentences
        .iter()
        .flat_map(|s| s.split_whitespace())
        .filter(|w| w.len() > 4)
        .collect();
    println!("Long words (>4 chars): {:?}", long_words);

    println!("\n=== Aggregation at End of Chain ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Numbers: {:?}", numbers);

    // Chain ending with sum
    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).map(|x| x * x).sum();
    println!("Sum of squares of evens: {}", sum);

    // Chain ending with count
    let count = numbers
        .iter()
        .filter(|&&x| x > 3)
        .filter(|&&x| x < 8)
        .count();
    println!("Count of numbers 4-7: {}", count);

    // Chain ending with fold
    let product: i32 = numbers.iter().take(5).fold(1, |acc, x| acc * x);
    println!("Product of first 5: {}", product);

    println!("\n=== Complex Business Logic Pipeline ===\n");

    #[derive(Debug, Clone)]
    struct Order {
        id: u32,
        customer: String,
        items: Vec<(String, f64)>,
        shipped: bool,
    }

    let orders = vec![
        Order {
            id: 1,
            customer: "Alice".to_string(),
            items: vec![("Book".to_string(), 15.0), ("Pen".to_string(), 3.0)],
            shipped: true,
        },
        Order {
            id: 2,
            customer: "Bob".to_string(),
            items: vec![("Laptop".to_string(), 1200.0)],
            shipped: false,
        },
        Order {
            id: 3,
            customer: "Alice".to_string(),
            items: vec![("Mouse".to_string(), 25.0), ("Keyboard".to_string(), 75.0)],
            shipped: true,
        },
    ];

    // Calculate total revenue from shipped orders
    let shipped_revenue: f64 = orders
        .iter()
        .filter(|o| o.shipped) // Only shipped
        .flat_map(|o| o.items.iter()) // Get all items
        .map(|(_, price)| price) // Extract prices
        .sum();
    println!("Shipped revenue: ${:.2}", shipped_revenue);

    // Get all unique customers with shipped orders
    let shipped_customers: Vec<&String> = orders
        .iter()
        .filter(|o| o.shipped)
        .map(|o| &o.customer)
        .collect();
    println!("Customers with shipped orders: {:?}", shipped_customers);

    // Calculate order totals
    let order_totals: Vec<(u32, f64)> = orders
        .iter()
        .map(|o| {
            let total: f64 = o.items.iter().map(|(_, p)| p).sum();
            (o.id, total)
        })
        .collect();
    println!("Order totals: {:?}", order_totals);

    println!("\n=== Chaining with take and skip ===\n");

    let numbers: Vec<i32> = (1..=100).collect();

    // Pagination: page 3, 10 items per page
    let page = 3;
    let per_page = 10;
    let page_items: Vec<i32> = numbers
        .iter()
        .skip((page - 1) * per_page)
        .take(per_page)
        .copied()
        .collect();
    println!(
        "Page {} (items per page: {}): {:?}",
        page, per_page, page_items
    );

    // Take while ascending
    let data = vec![1, 2, 3, 5, 4, 6, 7]; // Not strictly ascending
    let ascending: Vec<i32> = data
        .iter()
        .scan(0, |prev, &x| {
            if x > *prev {
                *prev = x;
                Some(x)
            } else {
                None
            }
        })
        .collect();
    println!("\nData: {:?}", data);
    println!("Ascending prefix: {:?}", ascending);

    println!("\n=== inspect() for Debugging Chains ===\n");

    let numbers = vec![1, 2, 3, 4, 5];

    // Use inspect() to see intermediate values
    println!("Processing with inspect():");
    let _result: Vec<i32> = numbers
        .iter()
        .inspect(|x| println!("  original: {}", x))
        .filter(|&&x| x % 2 == 0)
        .inspect(|x| println!("  after filter: {}", x))
        .map(|x| x * 10)
        .inspect(|x| println!("  after map: {}", x))
        .collect();

    println!("\n=== Performance: Lazy Evaluation ===\n");

    // Iterators are lazy - only compute what's needed
    let numbers: Vec<i32> = (1..1000).collect();

    // This only processes until we find 3 items
    let first_three_even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * x)
        .take(3)
        .collect();
    println!("First 3 even squares: {:?}", first_three_even_squares);
    // Only processed 6 numbers (2, 4, 6), not all 1000!

    println!("\n=== Building Reusable Pipelines ===\n");

    // Create functions that return iterators
    fn process_scores(scores: &[i32]) -> impl Iterator<Item = i32> + '_ {
        scores
            .iter()
            .filter(|&&s| s >= 50) // Passing scores
            .map(|&s| s + 5) // Add curve
    }

    let test_scores = vec![45, 67, 82, 38, 91, 55];
    let curved: Vec<i32> = process_scores(&test_scores).collect();
    let average: f64 = process_scores(&test_scores).sum::<i32>() as f64
        / process_scores(&test_scores).count() as f64;

    println!("Original scores: {:?}", test_scores);
    println!("Curved passing scores: {:?}", curved);
    println!("Average of curved passing: {:.2}", average);
}
