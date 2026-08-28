// Example: loop, while, for Loops
//
// Demonstrates:
// - Infinite loop with break
// - while loops with conditions
// - for loops with ranges and iterators
// - Returning values from loops
// - Loop labels for nested loops

fn main() {
    println!("=== Basic loop ===\n");

    // loop runs forever until break is called
    let mut counter = 0;
    loop {
        counter += 1;
        println!("loop iteration: {}", counter);
        if counter >= 3 {
            break;
        }
    }
    println!("Loop ended after {} iterations", counter);

    println!("\n=== loop with Return Value ===\n");

    // Loops can return values using break
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // Return 20
        }
    };
    println!("Loop returned: {}", result);

    println!("\n=== Basic while Loop ===\n");

    let mut number = 5;
    while number > 0 {
        println!("while: number = {}", number);
        number -= 1;
    }
    println!("Liftoff!");

    println!("\n=== while with Condition ===\n");

    let mut sum = 0;
    let mut i = 1;
    while sum < 20 {
        sum += i;
        println!("Adding {}: sum = {}", i, sum);
        i += 1;
    }
    println!("Final sum: {}", sum);

    println!("\n=== for Loop with Range ===\n");

    // Exclusive range (1 to 4, not including 5)
    println!("Exclusive range (1..5):");
    for i in 1..5 {
        println!("  i = {}", i);
    }

    // Inclusive range (1 to 5, including 5)
    println!("\nInclusive range (1..=5):");
    for i in 1..=5 {
        println!("  i = {}", i);
    }

    println!("\n=== for Loop Reverse ===\n");

    // Reverse iteration
    println!("Countdown:");
    for number in (1..=5).rev() {
        println!("  {}", number);
    }
    println!("Blast off!");

    println!("\n=== for Loop with Array ===\n");

    let numbers = [10, 20, 30, 40, 50];

    // Iterate over array elements
    println!("Array elements:");
    for num in numbers {
        println!("  {}", num);
    }

    // Iterate with index using enumerate
    println!("\nWith indices:");
    for (index, num) in numbers.iter().enumerate() {
        println!("  [{}] = {}", index, num);
    }

    println!("\n=== for Loop with Vector ===\n");

    let fruits = vec!["apple", "banana", "cherry"];

    // Iterate by reference
    println!("Fruits (by reference):");
    for fruit in &fruits {
        println!("  {}", fruit);
    }
    println!("Vector still usable: {:?}", fruits);

    // Iterate by value (consumes the vector)
    let colors = vec!["red", "green", "blue"];
    println!("\nColors (by value):");
    for color in colors {
        println!("  {}", color);
    }
    // colors is no longer usable here

    println!("\n=== for Loop with Step ===\n");

    // Using step_by for custom increments
    println!("Even numbers from 0 to 10:");
    for num in (0..=10).step_by(2) {
        print!("{} ", num);
    }
    println!();

    println!("\n=== Nested Loops ===\n");

    println!("Multiplication table (1-3):");
    for i in 1..=3 {
        for j in 1..=3 {
            print!("{:4}", i * j);
        }
        println!();
    }

    println!("\n=== Loop Labels ===\n");

    // Using labels to break/continue outer loops
    'outer: for i in 1..=3 {
        println!("outer loop: i = {}", i);
        for j in 1..=3 {
            println!("  inner loop: j = {}", j);
            if j == 2 && i == 2 {
                println!("  Breaking outer loop!");
                break 'outer;
            }
        }
    }
    println!("After labeled break");

    println!("\n=== Continue with Labels ===\n");

    'row: for row in 1..=3 {
        for col in 1..=3 {
            if col == 2 {
                println!("Skipping rest of row {}", row);
                continue 'row; // Continue to next outer iteration
            }
            print!("({},{}) ", row, col);
        }
        println!();
    }

    println!("\n=== while let Pattern ===\n");

    // while let for pattern matching in loops
    let mut stack = vec![1, 2, 3];
    println!("Stack: {:?}", stack);

    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    println!("Stack is now empty: {:?}", stack);

    println!("\n=== Iterating with iter_mut ===\n");

    let mut values = vec![1, 2, 3, 4, 5];
    println!("Before: {:?}", values);

    for value in values.iter_mut() {
        *value *= 2;
    }
    println!("After doubling: {:?}", values);

    println!("\n=== Loop Patterns ===\n");

    // Sum using loop
    let numbers = [1, 2, 3, 4, 5];
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    println!("Sum of {:?} = {}", numbers, sum);

    // Find element
    let items = ["apple", "banana", "cherry"];
    let mut found = false;
    for item in items {
        if item == "banana" {
            found = true;
            break;
        }
    }
    println!("Found 'banana': {}", found);

    // Count occurrences
    let text = "hello world";
    let mut count = 0;
    for ch in text.chars() {
        if ch == 'l' {
            count += 1;
        }
    }
    println!("Count of 'l' in '{}': {}", text, count);

    println!("\n=== Infinite Loop with Condition ===\n");

    // Simulating a game loop (limited for demo)
    let mut game_running = true;
    let mut frame = 0;

    while game_running {
        frame += 1;
        println!("Frame {}: Processing...", frame);

        // Exit condition
        if frame >= 3 {
            game_running = false;
            println!("Game over!");
        }
    }
}
