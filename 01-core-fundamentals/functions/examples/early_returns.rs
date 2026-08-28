// Example: Using the `return` Keyword for Early Returns
//
// Demonstrates:
// - Early returns for guard clauses
// - Error handling patterns
// - Return from nested blocks
// - When to use explicit vs implicit returns

fn main() {
    println!("=== Basic Early Return ===\n");

    let result1 = divide(10, 2);
    let result2 = divide(10, 0);
    println!("divide(10, 2) = {:?}", result1);
    println!("divide(10, 0) = {:?}", result2);

    println!("\n=== Guard Clauses ===\n");

    println!("process_age(25):");
    process_age(25);

    println!("\nprocess_age(15):");
    process_age(15);

    println!("\nprocess_age(-5):");
    process_age(-5);

    println!("\n=== Validation with Early Return ===\n");

    let result = validate_username("alice_123");
    println!("validate_username(\"alice_123\") = {:?}", result);

    let result = validate_username("");
    println!("validate_username(\"\") = {:?}", result);

    let result = validate_username("ab");
    println!("validate_username(\"ab\") = {:?}", result);

    let result = validate_username("has space");
    println!("validate_username(\"has space\") = {:?}", result);

    println!("\n=== Early Return in Loops ===\n");

    let numbers = vec![1, 3, 5, 7, 8, 9, 11];
    println!(
        "find_first_even({:?}) = {:?}",
        numbers,
        find_first_even(&numbers)
    );

    let odd_numbers = vec![1, 3, 5, 7, 9];
    println!(
        "find_first_even({:?}) = {:?}",
        odd_numbers,
        find_first_even(&odd_numbers)
    );

    println!("\n=== Return from Nested Structures ===\n");

    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("find_in_matrix(5) = {:?}", find_in_matrix(&matrix, 5));
    println!("find_in_matrix(99) = {:?}", find_in_matrix(&matrix, 99));

    println!("\n=== Error Propagation Pattern ===\n");

    let result = process_data("42");
    println!("process_data(\"42\") = {:?}", result);

    let result = process_data("not a number");
    println!("process_data(\"not a number\") = {:?}", result);

    println!("\n=== Multiple Early Returns ===\n");

    println!("classify_age(5) = '{}'", classify_age(5));
    println!("classify_age(15) = '{}'", classify_age(15));
    println!("classify_age(25) = '{}'", classify_age(25));
    println!("classify_age(70) = '{}'", classify_age(70));

    println!("\n=== Early Return vs Match ===\n");

    // Early return style
    println!(
        "get_day_type_early(\"Saturday\") = '{}'",
        get_day_type_early("Saturday")
    );
    println!(
        "get_day_type_early(\"Monday\") = '{}'",
        get_day_type_early("Monday")
    );

    // Match style (often preferred)
    println!(
        "get_day_type_match(\"Saturday\") = '{}'",
        get_day_type_match("Saturday")
    );
    println!(
        "get_day_type_match(\"Monday\") = '{}'",
        get_day_type_match("Monday")
    );

    println!("\n=== Return in Closures ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Using early return in a closure with find
    let first_greater_than_5 = numbers.iter().find(|&&x| x > 5);
    println!("First number > 5: {:?}", first_greater_than_5);

    // Custom filter with early return logic
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|&&x| {
            if x % 2 != 0 {
                return false; // Early return in closure
            }
            true
        })
        .copied()
        .collect();
    println!("Even numbers: {:?}", evens);

    println!("\n=== Return with Complex Types ===\n");

    let user_result = create_user("alice", 25);
    println!("create_user(\"alice\", 25) = {:?}", user_result);

    let user_result = create_user("", 25);
    println!("create_user(\"\", 25) = {:?}", user_result);

    let user_result = create_user("alice", 150);
    println!("create_user(\"alice\", 150) = {:?}", user_result);
}

// --- Basic early return for error handling ---

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None; // Early return on error condition
    }
    Some(a / b) // Normal return (implicit)
}

// --- Guard clauses pattern ---

fn process_age(age: i32) {
    // Guard clause 1: invalid input
    if age < 0 {
        println!("  Error: Age cannot be negative");
        return;
    }

    // Guard clause 2: special case
    if age < 18 {
        println!("  Notice: Minor detected, restricted access");
        return;
    }

    // Main logic (only reached if all guards pass)
    println!("  Processing adult user, age: {}", age);
    println!("  Full access granted");
}

// --- Validation with early return ---

fn validate_username(username: &str) -> Result<&str, &'static str> {
    // Guard: empty username
    if username.is_empty() {
        return Err("Username cannot be empty");
    }

    // Guard: too short
    if username.len() < 3 {
        return Err("Username must be at least 3 characters");
    }

    // Guard: too long
    if username.len() > 20 {
        return Err("Username must be at most 20 characters");
    }

    // Guard: invalid characters
    if username.contains(' ') {
        return Err("Username cannot contain spaces");
    }

    // All validations passed
    Ok(username)
}

// --- Early return from loop ---

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num); // Early return when found
        }
    }
    None // Return None if no even number found
}

// --- Return from nested structures ---

fn find_in_matrix(matrix: &[Vec<i32>], target: i32) -> Option<(usize, usize)> {
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, &value) in row.iter().enumerate() {
            if value == target {
                return Some((row_idx, col_idx)); // Early return
            }
        }
    }
    None
}

// --- Error propagation with early return ---

fn process_data(input: &str) -> Result<i32, String> {
    // Parse with early return on error
    let number: i32 = match input.parse() {
        Ok(n) => n,
        Err(_) => return Err(format!("Failed to parse '{}' as number", input)),
    };

    // Additional validation with early return
    if number < 0 {
        return Err("Number must be non-negative".to_string());
    }

    // Process and return result
    Ok(number * 2)
}

// --- Multiple early returns ---

fn classify_age(age: u32) -> &'static str {
    if age < 13 {
        return "child";
    }
    if age < 20 {
        return "teenager";
    }
    if age < 60 {
        return "adult";
    }
    "senior" // Implicit return for last case
}

// --- Early return vs match comparison ---

fn get_day_type_early(day: &str) -> &'static str {
    if day == "Saturday" || day == "Sunday" {
        return "weekend";
    }
    if day == "Monday"
        || day == "Tuesday"
        || day == "Wednesday"
        || day == "Thursday"
        || day == "Friday"
    {
        return "weekday";
    }
    "unknown"
}

fn get_day_type_match(day: &str) -> &'static str {
    match day {
        "Saturday" | "Sunday" => "weekend",
        "Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" => "weekday",
        _ => "unknown",
    }
}

// --- Complex return type ---

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn create_user(name: &str, age: u32) -> Result<User, &'static str> {
    // Validate name
    if name.is_empty() {
        return Err("Name cannot be empty");
    }

    // Validate age
    if age > 120 {
        return Err("Age seems unrealistic");
    }

    // All validations passed, create user
    Ok(User {
        name: name.to_string(),
        age,
    })
}
