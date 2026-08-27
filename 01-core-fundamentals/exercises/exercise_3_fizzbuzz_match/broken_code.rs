// Exercise 3: FizzBuzz with Match Expressions (BROKEN CODE)
//
// This code has 3 bugs related to pattern matching, logic, and control flow
// Your job: Find and fix the bugs so it compiles and produces correct output

fn main() {
    println!("=== FizzBuzz: 1 to 15 ===\n");
    fizzbuzz_simple(15);

    println!("\n=== FizzBuzz: 1 to 30 ===\n");
    fizzbuzz_simple(30);

    println!("\n=== FizzBuzz with Description: 1 to 15 ===\n");
    fizzbuzz_with_description(15);
}

fn fizzbuzz_simple(limit: i32) {
    for i in 1..=limit {
        // Create tuple with divisibility checks
        let is_three = i % 3 == 0;
        let is_five = i % 5 == 0;

        // BUG 1: Wrong pattern order - this checks (five, three) not (three, five)
        // Also the first pattern is unreachable due to incorrect tuple structure
        match (is_three, is_five) {
            (false, false) => println!("{}", i),
            (false, true) => println!("Buzz"),
            (true, false) => println!("Fizz"),
            // BUG 2: This pattern will never be matched because the order is wrong
            // The compiler won't catch this, but it's logically incorrect
            (false, false) => println!("FizzBuzz"),
        }
    }
}

fn fizzbuzz_with_description(limit: i32) {
    for i in 1..=limit {
        let is_three = i % 3 == 0;
        let is_five = i % 5 == 0;

        // BUG 3: This match is missing a pattern arm for (true, true)
        // This will cause a compiler error about non-exhaustive patterns
        match (is_three, is_five) {
            (false, false) => println!("Number: {} → {}", i, i),
            (false, true) => println!("Number: {} → Buzz", i),
            (true, false) => println!("Number: {} → Fizz", i),
            // Missing (true, true) case!
        }
    }
}

// BUGS SUMMARY:
// 1. Pattern (false, false) appears twice - one should be (true, true)
// 2. Logic error: patterns don't cover all cases correctly
// 3. Match expression missing the (true, true) case - non-exhaustive patterns

// CORRECT PATTERN MATCHING:
// (false, false) → number
// (false, true)  → Buzz
// (true, false)  → Fizz
// (true, true)   → FizzBuzz

// EXPECTED BEHAVIOR:
// === FizzBuzz: 1 to 15 ===
//
// 1
// 2
// Fizz
// 4
// Buzz
// Fizz
// 7
// 8
// Fizz
// Buzz
// 11
// Fizz
// 13
// 14
// FizzBuzz
//
// === FizzBuzz: 1 to 30 ===
//
// 1
// 2
// Fizz
// ... (continues with same pattern)
// 30 → FizzBuzz
//
// === FizzBuzz with Description: 1 to 15 ===
//
// Number: 1 → 1
// Number: 2 → 2
// Number: 3 → Fizz
// ... (continues with pattern)
// Number: 15 → FizzBuzz
