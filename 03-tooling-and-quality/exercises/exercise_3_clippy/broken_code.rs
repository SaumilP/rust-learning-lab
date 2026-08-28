// Exercise 3: Fix Clippy Warnings (BROKEN CODE)
//
// This code compiles and runs, but has many Clippy warnings.
// Your job: Fix all Clippy warnings without changing the program's behavior.
// Run: cargo clippy

fn main() {
    println!("Testing is_even:");
    println!("  2 is even: {}", is_even(2));
    println!("  3 is even: {}", is_even(3));

    println!("\nTesting find_longest:");
    let words = vec!["apple", "banana", "cherry"];
    println!("  Longest word: {}", find_longest(&words));

    println!("\nTesting sum_positives:");
    let numbers = vec![-2, 5, -1, 3, -4, 7];
    println!("  Sum of positives: {}", sum_positives(&numbers));

    println!("\nTesting format_greeting:");
    let name = String::from("World");
    println!("  Greeting: {}", format_greeting(&name));

    println!("\nAll functions work correctly!");
}

// CLIPPY WARNING 1: Unnecessary `else` block after `return`
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

// CLIPPY WARNING 2: This could be simplified to just `n % 2 == 0`
// CLIPPY WARNING 3: Comparing to boolean literal
fn is_odd(n: i32) -> bool {
    if is_even(n) == false {
        true
    } else {
        false
    }
}

// CLIPPY WARNING 4: Manual implementation of `max_by_key`
// CLIPPY WARNING 5: Could use iterator methods instead of manual loop
fn find_longest(words: &[&str]) -> &str {
    let mut longest = "";
    for word in words {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    return longest;  // CLIPPY WARNING 6: Unnecessary return
}

// CLIPPY WARNING 7: Could use filter and sum
fn sum_positives(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for n in numbers {
        if *n > 0 {
            sum = sum + *n;  // CLIPPY WARNING 8: Could use += operator
        }
    }
    sum
}

// CLIPPY WARNING 9: Unnecessary clone
fn format_greeting(name: &String) -> String {
    let name_copy = name.clone();  // Unnecessary - we only need to read it
    let greeting = format!("Hello, {}!", name_copy);
    return greeting;  // CLIPPY WARNING 10: Unnecessary return
}

// CLIPPY WARNING 11: Could take &str instead of &String
fn greet_person(name: &String) {
    println!("Greetings, {}!", name);
}

// CLIPPY WARNING 12: Loop that could be replaced with iterator
fn count_evens(numbers: &[i32]) -> usize {
    let mut count = 0;
    for n in numbers.iter() {
        if n % 2 == 0 {
            count = count + 1;  // CLIPPY WARNING 13: Could use +=
        }
    }
    count
}

// CLIPPY WARNING 14: This match could be an if let
fn get_first_even(numbers: &[i32]) -> Option<i32> {
    match numbers.iter().find(|&&x| x % 2 == 0) {
        Some(n) => Some(*n),
        None => None,
    }
}

// CLIPPY WARNING 15: Redundant pattern matching
fn double_option(opt: Option<i32>) -> Option<i32> {
    match opt {
        Some(x) => Some(x * 2),
        None => None,
    }
}

// BUGS/WARNINGS SUMMARY:
// 1. is_even: `if x { return true } else { return false }` -> just `x`
// 2. is_odd: `if x == false { true } else { false }` -> just `!x`
// 3. is_odd: compare to bool literal -> use `!is_even(n)`
// 4. find_longest: manual loop -> use `.max_by_key()`
// 5. find_longest: unnecessary return statement
// 6. sum_positives: manual loop -> use `.filter().sum()`
// 7. sum_positives: `sum = sum + x` -> `sum += x`
// 8. format_greeting: unnecessary clone
// 9. format_greeting: unnecessary return
// 10. greet_person: take &str instead of &String
// 11. count_evens: manual loop -> use `.filter().count()`
// 12. count_evens: `count = count + 1` -> `count += 1`
// 13. get_first_even: match on option -> use `.copied()`
// 14. double_option: match to transform -> use `.map()`

// To check for Clippy warnings:
// cargo clippy
//
// Expected after fixes: No warnings!
