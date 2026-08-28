// Example: Basic Iterator Usage
//
// Demonstrates:
// - Different iterator types (iter, iter_mut, into_iter)
// - Common iterator adapters (map, filter, etc.)
// - Collecting results
// - Iterator methods

fn main() {
    println!("=== Iterator Types ===\n");

    let v = vec![1, 2, 3, 4, 5];

    println!("iter() - Borrow:");
    for item in v.iter() {
        println!("  {}", item);
    }
    println!("v still usable: {:?}", v);

    println!("\niter_mut() - Mutable Borrow:");
    let mut v = vec![1, 2, 3];
    for item in v.iter_mut() {
        *item *= 2;
    }
    println!("After doubling: {:?}", v);

    println!("\ninto_iter() - Take Ownership:");
    let v = vec![1, 2, 3];
    for item in v.into_iter() {
        println!("  {}", item);
    }
    // println!("v: {:?}", v);  // Error: v moved

    println!("\n=== Map Adapter ===\n");

    let v = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", v);
    println!("Doubled: {:?}", doubled);

    let squared: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("Squared: {:?}", squared);

    println!("\n=== Filter Adapter ===\n");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("Original: {:?}", v);
    println!("Evens: {:?}", evens);

    let greater_than_5: Vec<i32> = v.iter().filter(|&&x| x > 5).copied().collect();
    println!("Greater than 5: {:?}", greater_than_5);

    println!("\n=== Chaining Adapters ===\n");

    let v = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = v
        .iter()
        .filter(|&&x| x > 2) // Keep > 2
        .map(|x| x * 2) // Double each
        .collect();
    println!("Original: {:?}", v);
    println!("Filtered (>2) and doubled: {:?}", result);

    println!("\n=== Take and Skip ===\n");

    let v = vec![1, 2, 3, 4, 5];

    let first_three: Vec<i32> = v.iter().take(3).copied().collect();
    println!("First 3: {:?}", first_three);

    let skip_two: Vec<i32> = v.iter().skip(2).copied().collect();
    println!("Skip 2: {:?}", skip_two);

    println!("\n=== Enumerate ===\n");

    let fruits = vec!["apple", "banana", "cherry"];
    for (i, fruit) in fruits.iter().enumerate() {
        println!("  [{}]: {}", i, fruit);
    }

    println!("\n=== Find and Position ===\n");

    let v = vec![1, 2, 3, 4, 5];

    if let Some(value) = v.iter().find(|&&x| x > 3) {
        println!("First value > 3: {}", value);
    }

    if let Some(pos) = v.iter().position(|&x| x == 3) {
        println!("Position of 3: {}", pos);
    }

    println!("\n=== Consumer Methods ===\n");

    let v = vec![1, 2, 3, 4, 5];

    println!("sum: {}", v.iter().sum::<i32>());
    println!("count: {}", v.iter().count());
    println!("all > 0: {}", v.iter().all(|&x| x > 0));
    println!("any > 4: {}", v.iter().any(|&x| x > 4));

    println!("\n=== Fold (Accumulate) ===\n");

    let v = vec![1, 2, 3, 4, 5];

    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("Sum using fold: {}", sum);

    let product = v.iter().fold(1, |acc, x| acc * x);
    println!("Product using fold: {}", product);

    println!("\n=== Collecting Different Types ===\n");

    let numbers = vec!["1", "2", "3", "4", "5"];

    let parsed: Vec<i32> = numbers.iter().filter_map(|s| s.parse().ok()).collect();
    println!("Parsed numbers: {:?}", parsed);

    let strings: Vec<String> = v.iter().map(|x| format!("num_{}", x)).collect();
    println!("Formatted: {:?}", strings);

    println!("\n=== Lazy Evaluation ===\n");

    let v = vec![1, 2, 3, 4, 5];

    // This doesn't execute yet
    let iter = v.iter().map(|x| {
        println!("  Mapping {}", x);
        x * 2
    });

    println!("Iterator created (note: no output above)");

    // Now it executes:
    println!("Collecting results:");
    let _: Vec<_> = iter.collect();
}
