fn main() {
    println!("=== Rust Control Flow ===\n");

    // 1. IF EXPRESSIONS
    println!("1. IF EXPRESSIONS:\n");

    let number = 7;

    if number < 5 {
        println!("   {} is less than 5", number);
    } else if number < 10 {
        println!("   {} is between 5 and 10", number);
    } else {
        println!("   {} is 10 or greater", number);
    }

    // if is an expression (returns a value)
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("   {} is {}", number, result);

    // No truthiness - must be explicit boolean
    let condition = true;
    if condition {
        println!("   ✓ Condition was true");
    }

    // This won't compile (no truthiness):
    // if number { }  // ERROR: expected bool, found integer

    println!();

    // 2. LOOP - Infinite loop
    println!("2. LOOP (infinite loop with break):\n");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // Return value from loop
        }
    };

    println!("   Loop result: {}", result);
    println!("   ✓ `loop` creates infinite loops");
    println!("   ✓ Use `break` to exit with optional value\n");

    // Loop labels (for nested loops)
    let mut count = 0;
    'outer: loop {
        count += 1;
        if count > 3 {
            break 'outer;  // Break outer loop
        }
    }
    println!("   Loop with label ran {} times\n", count);

    // 3. WHILE LOOPS
    println!("3. WHILE LOOPS:\n");

    let mut number = 3;

    while number != 0 {
        println!("   {}!", number);
        number -= 1;
    }

    println!("   LIFTOFF!");
    println!("   ✓ `while` loops run while condition is true\n");

    // 4. FOR LOOPS
    println!("4. FOR LOOPS:\n");

    // Iterate over array
    let array = [10, 20, 30, 40, 50];

    for element in array {
        println!("   Array element: {}", element);
    }

    // Iterate over range
    println!("\n   Countdown using range:");
    for number in (1..=5).rev() {
        println!("   {}", number);
    }

    // Common ranges
    println!("\n   Range examples:");
    for i in 0..5 {  // 0, 1, 2, 3, 4 (exclusive end)
        print!("{} ", i);
    }
    println!("(0..5)");

    for i in 0..=5 {  // 0, 1, 2, 3, 4, 5 (inclusive end)
        print!("{} ", i);
    }
    println!("(0..=5)");

    // Enumerate (index + value)
    println!("\n   Enumerate:");
    for (index, value) in array.iter().enumerate() {
        println!("   index {}: value {}", index, value);
    }

    println!("   ✓ `for` is safer than `while` for iteration\n");

    // 5. MATCH EXPRESSIONS
    println!("5. MATCH EXPRESSIONS (pattern matching):\n");

    let number = 3;

    match number {
        1 => println!("   One!"),
        2 | 3 | 5 | 7 => println!("   Prime number: {}", number),
        4..=10 => println!("   Between 4 and 10: {}", number),
        _ => println!("   Something else: {}", number),  // catch-all
    }

    // Match is exhaustive (must cover all cases)
    let boolean = true;
    let message = match boolean {
        true => "yes",
        false => "no",
        // Must cover all cases!
    };
    println!("   Boolean match: {}", message);

    // Match with return values
    let day = 3;
    let day_name = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 | 7 => "Weekend",
        _ => "Invalid day",
    };
    println!("   Day {}: {}", day, day_name);

    println!("   ✓ `match` must be exhaustive");
    println!("   ✓ Use `_` as catch-all pattern\n");

    // 6. IF LET
    println!("6. IF LET (concise pattern matching):\n");

    let some_value: Option<i32> = Some(3);

    // Instead of:
    match some_value {
        Some(3) => println!("   Match: three!"),
        _ => (),
    }

    // Use if let:
    if let Some(3) = some_value {
        println!("   If let: three!");
    }

    // With else
    let some_value = Some(5);
    if let Some(3) = some_value {
        println!("   Three");
    } else {
        println!("   If let else: Not three");
    }

    println!("   ✓ `if let` is syntax sugar for match\n");

    // 7. WHILE LET
    println!("7. WHILE LET:\n");

    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("   Popped: {}", top);
    }

    println!("   ✓ `while let` loops while pattern matches\n");

    // 8. CONTINUE AND BREAK
    println!("8. CONTINUE and BREAK:\n");

    println!("   Skip even numbers:");
    for i in 0..10 {
        if i % 2 == 0 {
            continue;  // Skip rest of iteration
        }
        print!("{} ", i);
    }
    println!();

    println!("   Break at 5:");
    for i in 0..10 {
        if i == 5 {
            break;  // Exit loop
        }
        print!("{} ", i);
    }
    println!("\n");

    // 9. NESTED CONTROL FLOW
    println!("9. NESTED CONTROL FLOW:\n");

    for i in 1..=3 {
        for j in 1..=3 {
            let product = i * j;
            if product % 2 == 0 {
                print!("{} ", product);
            }
        }
    }
    println!("(even products)\n");

    // 10. RETURN FROM FUNCTIONS
    println!("10. RETURN FROM FUNCTIONS:\n");

    fn check_positive(n: i32) -> &'static str {
        if n > 0 {
            return "positive";  // Early return
        }

        if n < 0 {
            return "negative";
        }

        "zero"  // Last expression (implicit return)
    }

    println!("   check_positive(5): {}", check_positive(5));
    println!("   check_positive(-3): {}", check_positive(-3));
    println!("   check_positive(0): {}", check_positive(0));
    println!("   ✓ Use `return` for early exit");
    println!("   ✓ Last expression is implicit return\n");

    // 11. SUMMARY TABLE
    println!("11. CONTROL FLOW SUMMARY:\n");
    println!("   if/else          - Branching with conditions");
    println!("   loop             - Infinite loop (use break)");
    println!("   while            - Loop while condition is true");
    println!("   for              - Iterate over collections/ranges");
    println!("   match            - Pattern matching (exhaustive)");
    println!("   if let           - Concise pattern matching");
    println!("   while let        - Loop while pattern matches");
    println!("   break            - Exit loop (with optional value)");
    println!("   continue         - Skip to next iteration");
    println!("   return           - Exit function with value");
}

/*
Control Flow Comparison:
========================

RUST vs PYTHON:
- No truthiness: must use explicit booleans
- match is like switch (but exhaustive)
- for loops use ranges: 0..10 instead of range(10)
- No ternary operator: use if expression

RUST vs JAVA:
- if is an expression (returns value)
- No do-while loop
- for loops are more like for-each
- match is exhaustive (must cover all cases)

RUST vs C/C++:
- No parentheses required: if x > 5 { }
- No semicolons in if expressions
- loop is infinite, not while(true)
- for loops are safer (no manual indexing)

KEY POINTS:
1. Control flow constructs are expressions
2. match must be exhaustive
3. No implicit truthiness
4. for loops are preferred over while
5. break and continue work as expected
6. Labels allow breaking outer loops

Run this:
    cargo run

Experiment:
    - Try nested loops with labels
    - Write a match with different patterns
    - Use ranges with different syntax (.. vs ..=)
    - Compare if let vs full match
*/
