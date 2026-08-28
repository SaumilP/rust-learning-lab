// Example: Using the filter() Iterator Method
//
// Demonstrates:
// - Basic filter() usage with predicates
// - Combining filter() with other adapters
// - filter_map() for filtering and transforming
// - take_while() and skip_while()

fn main() {
    println!("=== Basic filter() Usage ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", numbers);

    // Filter even numbers
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);

    // Filter odd numbers
    let odds: Vec<&i32> = numbers.iter().filter(|&x| x % 2 != 0).collect();
    println!("Odd numbers: {:?}", odds);

    // Filter greater than 5
    let greater: Vec<&i32> = numbers.iter().filter(|&&x| x > 5).collect();
    println!("Greater than 5: {:?}", greater);

    // Filter in range
    let in_range: Vec<&i32> = numbers.iter().filter(|&&x| x >= 3 && x <= 7).collect();
    println!("Between 3 and 7: {:?}", in_range);

    println!("\n=== filter() with Strings ===\n");

    let words = vec!["hello", "world", "rust", "is", "great", "programming", "a"];
    println!("Words: {:?}", words);

    // Filter by length
    let long_words: Vec<&&str> = words.iter().filter(|s| s.len() > 4).collect();
    println!("Words longer than 4 chars: {:?}", long_words);

    // Filter by starting character
    let starts_with_r: Vec<&&str> = words.iter().filter(|s| s.starts_with('r')).collect();
    println!("Words starting with 'r': {:?}", starts_with_r);

    // Filter by containing character
    let contains_o: Vec<&&str> = words.iter().filter(|s| s.contains('o')).collect();
    println!("Words containing 'o': {:?}", contains_o);

    println!("\n=== filter() with Structs ===\n");

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u32,
        active: bool,
    }

    let people = vec![
        Person {
            name: String::from("Alice"),
            age: 30,
            active: true,
        },
        Person {
            name: String::from("Bob"),
            age: 17,
            active: true,
        },
        Person {
            name: String::from("Charlie"),
            age: 25,
            active: false,
        },
        Person {
            name: String::from("Diana"),
            age: 45,
            active: true,
        },
        Person {
            name: String::from("Eve"),
            age: 19,
            active: false,
        },
    ];

    // Filter adults
    let adults: Vec<&Person> = people.iter().filter(|p| p.age >= 18).collect();
    println!("Adults: {:?}", adults);

    // Filter active users
    let active: Vec<&Person> = people.iter().filter(|p| p.active).collect();
    println!("Active users: {:?}", active);

    // Complex filter: active adults
    let active_adults: Vec<&Person> = people.iter().filter(|p| p.age >= 18 && p.active).collect();
    println!("Active adults: {:?}", active_adults);

    println!("\n=== Chaining filter() with map() ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", numbers);

    // Filter then map
    let even_doubled: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("Even numbers doubled: {:?}", even_doubled);

    // Map then filter
    let squared_gt_20: Vec<i32> = numbers.iter().map(|x| x * x).filter(|&x| x > 20).collect();
    println!("Squares greater than 20: {:?}", squared_gt_20);

    println!("\n=== filter_map() - Filter and Transform ===\n");

    // Parse strings, keeping only valid numbers
    let inputs = vec!["1", "two", "3", "four", "5", "6"];
    println!("Inputs: {:?}", inputs);

    let valid_numbers: Vec<i32> = inputs.iter().filter_map(|s| s.parse().ok()).collect();
    println!("Valid numbers: {:?}", valid_numbers);

    // Extract and transform Option values
    let maybe_numbers: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    println!("\nMaybe numbers: {:?}", maybe_numbers);

    let actual_numbers: Vec<i32> = maybe_numbers.iter().filter_map(|x| *x).collect();
    println!("Actual numbers: {:?}", actual_numbers);

    // Equivalent using flatten() with map()
    let flattened: Vec<i32> = maybe_numbers.into_iter().flatten().collect();
    println!("Using flatten: {:?}", flattened);

    println!("\n=== Multiple Filters ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    println!("Original: {:?}", numbers);

    // Chain multiple filters
    let result: Vec<&i32> = numbers
        .iter()
        .filter(|&&x| x > 3) // Greater than 3
        .filter(|&&x| x < 10) // Less than 10
        .filter(|&&x| x % 2 == 0) // Even
        .collect();
    println!("4-9, even: {:?}", result);

    // Or combine in one filter
    let result2: Vec<&i32> = numbers
        .iter()
        .filter(|&&x| x > 3 && x < 10 && x % 2 == 0)
        .collect();
    println!("Combined filter: {:?}", result2);

    println!("\n=== take_while() and skip_while() ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", numbers);

    // take_while: take elements while condition is true
    let small: Vec<&i32> = numbers.iter().take_while(|&&x| x < 5).collect();
    println!("take_while(x < 5): {:?}", small);

    // skip_while: skip elements while condition is true
    let from_five: Vec<&i32> = numbers.iter().skip_while(|&&x| x < 5).collect();
    println!("skip_while(x < 5): {:?}", from_five);

    // Note: once condition becomes false, take_while stops
    // This is different from filter!
    let mixed = vec![1, 2, 10, 3, 4]; // 10 is > 5
    println!("\nMixed: {:?}", mixed);
    println!(
        "take_while(x < 5): {:?}",
        mixed.iter().take_while(|&&x| x < 5).collect::<Vec<_>>()
    );
    println!(
        "filter(x < 5): {:?}",
        mixed.iter().filter(|&&x| x < 5).collect::<Vec<_>>()
    );

    println!("\n=== Negating Filters with ! ===\n");

    let words = vec!["hello", "", "world", "", "rust"];
    println!("Words with empties: {:?}", words);

    // Filter out empty strings
    let non_empty: Vec<&&str> = words.iter().filter(|s| !s.is_empty()).collect();
    println!("Non-empty: {:?}", non_empty);

    // Keep only empties (for demonstration)
    let empties: Vec<&&str> = words.iter().filter(|s| s.is_empty()).collect();
    println!("Empty strings: {:?}", empties);

    println!("\n=== partition() - Split by Predicate ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Original: {:?}", numbers);

    // Partition into evens and odds
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.iter().partition(|&&x| x % 2 == 0);
    println!("Evens: {:?}", evens);
    println!("Odds: {:?}", odds);

    println!("\n=== find() - First Match ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Numbers: {:?}", numbers);

    // find() returns Option of first match
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("First even: {:?}", first_even);

    let first_gt_100 = numbers.iter().find(|&&x| x > 100);
    println!("First > 100: {:?}", first_gt_100);

    // find_map() combines find and map
    let strings = vec!["a", "b", "42", "c", "100"];
    let first_number: Option<i32> = strings.iter().find_map(|s| s.parse().ok());
    println!(
        "First parseable number in {:?}: {:?}",
        strings, first_number
    );

    println!("\n=== Practical Examples ===\n");

    // Example 1: Filter valid emails
    let emails = vec![
        "alice@example.com",
        "invalid",
        "bob@test.org",
        "not.an.email",
        "charlie@domain.net",
    ];
    let valid_emails: Vec<&&str> = emails
        .iter()
        .filter(|e| e.contains('@') && e.contains('.'))
        .collect();
    println!("Valid emails: {:?}", valid_emails);

    // Example 2: Filter and count
    let scores = vec![85, 92, 78, 95, 88, 72, 90, 83];
    let high_scores: Vec<&i32> = scores.iter().filter(|&&s| s >= 90).collect();
    println!("\nAll scores: {:?}", scores);
    println!("High scores (>=90): {:?}", high_scores);
    println!("Number of high scores: {}", high_scores.len());

    // Example 3: Complex data filtering
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Order {
        id: u32,
        amount: f64,
        shipped: bool,
    }

    let orders = vec![
        Order {
            id: 1,
            amount: 150.0,
            shipped: true,
        },
        Order {
            id: 2,
            amount: 50.0,
            shipped: false,
        },
        Order {
            id: 3,
            amount: 200.0,
            shipped: true,
        },
        Order {
            id: 4,
            amount: 75.0,
            shipped: false,
        },
    ];

    let large_unshipped: Vec<&Order> = orders
        .iter()
        .filter(|o| o.amount > 100.0 && !o.shipped)
        .collect();
    println!("\nLarge unshipped orders: {:?}", large_unshipped);

    let total_shipped: f64 = orders.iter().filter(|o| o.shipped).map(|o| o.amount).sum();
    println!("Total shipped amount: ${:.2}", total_shipped);
}
