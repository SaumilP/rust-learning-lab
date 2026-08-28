// Example: Searching and Sorting Algorithms
//
// Demonstrates:
// - Linear search
// - Binary search
// - Various sorting approaches
// - Filtering and transforming
// - Algorithm complexity

fn main() {
    println!("=== Searching and Sorting Algorithms ===\n");

    // Example 1: Linear Search
    println!("1. LINEAR SEARCH");
    let numbers = vec![3, 7, 2, 9, 1, 5, 8, 4];
    println!("   Array: {:?}", numbers);

    if let Some(pos) = linear_search(&numbers, 9) {
        println!("   Found 9 at index: {}", pos);
    } else {
        println!("   9 not found");
    }

    if let Some(pos) = linear_search(&numbers, 10) {
        println!("   Found 10 at index: {}", pos);
    } else {
        println!("   10 not found");
    }

    // Example 2: Sorted Array for Binary Search
    println!("\n2. BINARY SEARCH");
    let mut sorted = numbers.clone();
    sorted.sort();
    println!("   Sorted array: {:?}", sorted);

    if let Some(pos) = binary_search(&sorted, 7) {
        println!("   Found 7 at index: {}", pos);
    } else {
        println!("   7 not found");
    }

    if let Some(pos) = binary_search(&sorted, 6) {
        println!("   Found 6 at index: {}", pos);
    } else {
        println!("   6 not found");
    }

    // Example 3: Built-in Sorting
    println!("\n3. BUILT-IN SORTING");
    let mut arr = vec![5, 2, 8, 1, 9];
    println!("   Original: {:?}", arr);

    arr.sort();
    println!("   Sorted ascending: {:?}", arr);

    arr.sort_by(|a, b| b.cmp(a));
    println!("   Sorted descending: {:?}", arr);

    // Example 4: Sorting Strings
    println!("\n4. SORTING STRINGS");
    let mut words = vec!["zebra", "apple", "mango", "banana"];
    println!("   Original: {:?}", words);

    words.sort();
    println!("   Sorted: {:?}", words);

    // Sort by length
    words.sort_by_key(|w| w.len());
    println!("   Sorted by length: {:?}", words);

    // Example 5: Filtering (keep matching elements)
    println!("\n5. FILTERING");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("   Original: {:?}", numbers);

    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("   Even numbers: {:?}", evens);

    let greater_than_five: Vec<i32> = numbers.iter().filter(|&&x| x > 5).copied().collect();
    println!("   Greater than 5: {:?}", greater_than_five);

    // Example 6: Partitioning (split into two groups)
    println!("\n6. PARTITIONING");
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.iter().partition(|&&x| x % 2 == 0);
    println!("   Even: {:?}", evens);
    println!("   Odd: {:?}", odds);

    // Example 7: Finding Max/Min
    println!("\n7. FINDING MAX/MIN");
    println!("   Array: {:?}", numbers);

    let max = numbers.iter().max().copied();
    println!("   Max: {:?}", max);

    let min = numbers.iter().min().copied();
    println!("   Min: {:?}", min);

    // Example 8: Counting
    println!("\n8. COUNTING");
    let target = 5;
    let count = numbers.iter().filter(|&&x| x == target).count();
    println!("   Count of {} in array: {}", target, count);

    let evens_count = numbers.iter().filter(|&&x| x % 2 == 0).count();
    println!("   Count of even numbers: {}", evens_count);

    // Example 9: Finding with Position
    println!("\n9. FINDING WITH POSITION");
    if let Some(pos) = numbers.iter().position(|&x| x > 7) {
        println!("   First element > 7 is at index {}: {}", pos, numbers[pos]);
    }

    // Example 10: Using Contains
    println!("\n10. CONTAINS CHECK");
    println!("   Does array contain 5? {}", numbers.contains(&5));
    println!("   Does array contain 11? {}", numbers.contains(&11));

    // Example 11: Removing Duplicates
    println!("\n11. REMOVING DUPLICATES");
    let with_dups = vec![1, 2, 2, 3, 3, 3, 4, 5, 5];
    println!("   With duplicates: {:?}", with_dups);

    let mut unique = with_dups.clone();
    unique.sort();
    unique.dedup();
    println!("   After dedup: {:?}", unique);

    // Alternative: using HashSet
    use std::collections::HashSet;
    let unique_set: HashSet<_> = with_dups.iter().collect();
    let mut unique_vec: Vec<_> = unique_set.iter().copied().collect();
    unique_vec.sort();
    println!("   Using HashSet: {:?}", unique_vec);

    println!("\nAlgorithm examples complete!");
}

// Linear search - O(n) complexity
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &value) in arr.iter().enumerate() {
        if value == target {
            return Some(i);
        }
    }
    None
}

// Binary search - O(log n) complexity (requires sorted array)
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => left = mid + 1,
            std::cmp::Ordering::Greater => right = mid,
        }
    }
    None
}

// Expected output:
// === Searching and Sorting Algorithms ===
//
// 1. LINEAR SEARCH
//    Array: [3, 7, 2, 9, 1, 5, 8, 4]
//    Found 9 at index: 3
//    9 not found
//
// 2. BINARY SEARCH
//    Sorted array: [1, 2, 3, 4, 5, 7, 8, 9]
//    Found 7 at index: 5
//    6 not found
//
// 3. BUILT-IN SORTING
//    Original: [5, 2, 8, 1, 9]
//    Sorted ascending: [1, 2, 5, 8, 9]
//    Sorted descending: [9, 8, 5, 2, 1]
//
// ... (more output)
//
// Algorithm examples complete!
