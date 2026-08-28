// Example: if, else if, else Expressions
//
// Demonstrates:
// - Basic if/else statements
// - else if chains
// - if as an expression (returning values)
// - Nested conditions
// - Boolean conditions and comparisons

fn main() {
    println!("=== Basic if Statement ===\n");

    let number = 7;

    if number > 5 {
        println!("{} is greater than 5", number);
    }

    if number < 5 {
        println!("{} is less than 5", number);
    }

    println!("\n=== if/else Statement ===\n");

    let age = 20;

    if age >= 18 {
        println!("You are an adult (age {})", age);
    } else {
        println!("You are a minor (age {})", age);
    }

    println!("\n=== else if Chains ===\n");

    let score = 85;

    if score >= 90 {
        println!("Grade: A (score: {})", score);
    } else if score >= 80 {
        println!("Grade: B (score: {})", score);
    } else if score >= 70 {
        println!("Grade: C (score: {})", score);
    } else if score >= 60 {
        println!("Grade: D (score: {})", score);
    } else {
        println!("Grade: F (score: {})", score);
    }

    println!("\n=== if as Expression ===\n");

    let condition = true;
    // if is an expression - it returns a value
    let result = if condition { 5 } else { 10 };
    println!("if {} then 5 else 10 = {}", condition, result);

    // Both branches must return the same type
    let x = 42;
    let description = if x > 50 {
        "large"
    } else if x > 20 {
        "medium"
    } else {
        "small"
    };
    println!("{} is {}", x, description);

    println!("\n=== Comparing Values ===\n");

    let a = 10;
    let b = 20;

    if a == b {
        println!("{} equals {}", a, b);
    } else if a > b {
        println!("{} is greater than {}", a, b);
    } else {
        println!("{} is less than {}", a, b);
    }

    // All comparison operators
    let x = 5;
    let y = 10;
    println!("x = {}, y = {}", x, y);
    println!("x == y: {}", x == y);
    println!("x != y: {}", x != y);
    println!("x < y: {}", x < y);
    println!("x > y: {}", x > y);
    println!("x <= y: {}", x <= y);
    println!("x >= y: {}", x >= y);

    println!("\n=== Boolean Operators ===\n");

    let has_key = true;
    let has_permission = false;

    // AND operator (&&)
    if has_key && has_permission {
        println!("Access granted");
    } else {
        println!("Access denied (need both key AND permission)");
    }

    // OR operator (||)
    if has_key || has_permission {
        println!("Partial access (have key OR permission)");
    } else {
        println!("No access at all");
    }

    // NOT operator (!)
    let is_blocked = false;
    if !is_blocked {
        println!("User is not blocked");
    }

    println!("\n=== Complex Conditions ===\n");

    let age = 25;
    let has_license = true;
    let is_insured = true;

    if age >= 18 && has_license && is_insured {
        println!("Can rent a car");
    } else if age >= 18 && has_license {
        println!("Need insurance to rent");
    } else if age >= 18 {
        println!("Need license and insurance");
    } else {
        println!("Too young to rent");
    }

    println!("\n=== Nested if Statements ===\n");

    let number = 15;

    if number > 0 {
        println!("{} is positive", number);
        if number % 2 == 0 {
            println!("  and even");
        } else {
            println!("  and odd");
        }
        if number > 10 {
            println!("  and greater than 10");
        }
    } else if number < 0 {
        println!("{} is negative", number);
    } else {
        println!("number is zero");
    }

    println!("\n=== if let Pattern ===\n");

    // if let is useful for single pattern matching
    let favorite_color: Option<&str> = Some("blue");

    if let Some(color) = favorite_color {
        println!("Favorite color is {}", color);
    } else {
        println!("No favorite color");
    }

    // Equivalent to match
    let number: Option<i32> = Some(42);
    if let Some(n) = number {
        println!("The number is {}", n);
    }

    println!("\n=== if in Function Context ===\n");

    println!("is_even(4) = {}", is_even(4));
    println!("is_even(7) = {}", is_even(7));

    println!("abs(-5) = {}", abs(-5));
    println!("abs(5) = {}", abs(5));

    println!("max(10, 20) = {}", max(10, 20));
    println!("clamp(5, 0, 10) = {}", clamp(5, 0, 10));
    println!("clamp(15, 0, 10) = {}", clamp(15, 0, 10));
    println!("clamp(-5, 0, 10) = {}", clamp(-5, 0, 10));

    println!("\n=== Chained Conditions in Expressions ===\n");

    let temperature = 25;
    let weather = if temperature > 30 {
        "hot"
    } else if temperature > 20 {
        "warm"
    } else if temperature > 10 {
        "cool"
    } else if temperature > 0 {
        "cold"
    } else {
        "freezing"
    };
    println!("{}C is {}", temperature, weather);

    println!("\n=== if with Multiple Conditions ===\n");

    let year = 2024;

    // Check if leap year
    let is_leap_year = if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    };

    println!("{} is a leap year: {}", year, is_leap_year);
}

// --- Helper functions demonstrating if in functions ---

fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
    // Note: could simply be: n % 2 == 0
}

fn abs(n: i32) -> i32 {
    if n < 0 {
        -n
    } else {
        n
    }
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
