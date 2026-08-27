// Example: Match Expressions
//
// Demonstrates:
// - Basic match expressions
// - Pattern matching
// - Exhaustive matching
// - Catch-all pattern (_)
// - Range patterns

fn main() {
    println!("=== Basic Match ===\n");

    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other number"),
    }

    println!("\n=== Match with Return ===\n");

    let num = 5;
    let description = match num {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "other",
    };
    println!("Number {} is '{}'", num, description);

    println!("\n=== Match with Multiple Arms ===\n");

    let day = 3;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day"),
    }

    println!("\n=== Match with OR Pattern ===\n");

    let x = 2;
    match x {
        1 | 2 => println!("One or Two"),
        3 | 4 => println!("Three or Four"),
        5 => println!("Five"),
        _ => println!("Other"),
    }

    println!("\n=== Match with Range ===\n");

    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        0..=59 => "F",
        _ => "Invalid",
    };
    println!("Score {} = Grade {}", score, grade);

    println!("\n=== Range Examples ===\n");

    for i in 1..=5 {
        println!("Range 1..=5: {}", i);
    }

    println!();

    for i in 0..3 {
        println!("Range 0..3: {}", i);
    }

    println!("\n=== Match with Binding ===\n");

    let value = 7;
    match value {
        1..=5 => println!("Value {} is in range 1-5", value),
        6..=10 => println!("Value {} is in range 6-10", value),
        11..=15 => println!("Value {} is in range 11-15", value),
        _ => println!("Value {} is out of range", value),
    }

    println!("\n=== Match Expressions in Calculation ===\n");

    let x = 1;
    let result = match x {
        0 => 0,
        1 => 1,
        _ => x * x,
    };
    println!("match x={} => {}", x, result);

    println!("\n=== Match with Multiple Arms (Blocks) ===\n");

    let number = 2;
    match number {
        1 => {
            println!("One");
            println!("The number is 1");
        }
        2 => {
            println!("Two");
            println!("The number is 2");
        }
        _ => {
            println!("Other");
            println!("Unknown number");
        }
    }

    println!("\n=== Match Strings ===\n");

    let day_str = "Monday";
    match day_str {
        "Monday" => println!("Start of week"),
        "Friday" => println!("Almost weekend"),
        "Saturday" | "Sunday" => println!("Weekend"),
        _ => println!("Midweek"),
    }

    println!("\n=== Match Boolean ===\n");

    let is_adult = true;
    match is_adult {
        true => println!("You are an adult"),
        false => println!("You are a minor"),
    }

    println!("\n=== Exhaustive Matching ===\n");

    // Must handle all cases
    let flag = true;
    let message = match flag {
        true => "Yes",
        false => "No",
        // No need for _, all cases covered
    };
    println!("flag={} => message='{}'", flag, message);
}
