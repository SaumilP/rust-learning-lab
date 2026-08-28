// Example: Expressions vs Statements in Functions
//
// Demonstrates:
// - Expressions return values, statements do not
// - Block expressions
// - If expressions
// - Match expressions as values
// - Implicit return via expressions

fn main() {
    println!("=== Expressions vs Statements ===\n");

    // Statement: does not return a value (ends with semicolon)
    let _x = 5; // This is a statement

    // Expression: returns a value (no semicolon at the end)
    let y = {
        let temp = 3;
        temp + 1 // Expression - no semicolon, returns value
    };
    println!("Block expression result: {}", y);

    // This would be a statement returning () (unit type)
    let z: () = {
        let _temp = 3;
        // Statement with semicolon returns ()
    };
    println!("Empty block returns unit: {:?}", z);

    println!("\n=== Function Bodies as Expressions ===\n");

    let doubled = double_value(21);
    println!("double_value(21) = {}", doubled);

    let tripled = triple_value(10);
    println!("triple_value(10) = {}", tripled);

    // Comparison: both functions return the same result
    let a = add_with_return(5, 3);
    let b = add_with_expression(5, 3);
    println!("add_with_return(5, 3) = {}", a);
    println!("add_with_expression(5, 3) = {}", b);

    println!("\n=== If Expressions ===\n");

    let condition = true;
    // if is an expression that can return a value
    let number = if condition { 5 } else { 6 };
    println!("if condition {{ 5 }} else {{ 6 }} = {}", number);

    // Using if expression in a function
    let max = maximum(10, 20);
    println!("maximum(10, 20) = {}", max);

    let sign = get_sign(-42);
    println!("get_sign(-42) = '{}'", sign);

    println!("\n=== Match Expressions ===\n");

    let dice_roll = 4;
    let movement = match dice_roll {
        1 => "move one space",
        2 => "move two spaces",
        3 => "move three spaces",
        _ => "move more spaces",
    };
    println!("dice_roll {} -> '{}'", dice_roll, movement);

    let result = classify_number(42);
    println!("classify_number(42) = '{}'", result);

    println!("\n=== Nested Block Expressions ===\n");

    let result = {
        let a = 10;
        let b = {
            let c = 5;
            c * 2 // Inner block returns 10
        };
        a + b // Outer block returns 20
    };
    println!("Nested block result: {}", result);

    println!("\n=== Loop as Expression ===\n");

    // loop can return a value with break
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return value from loop
        }
    };
    println!("loop_result = {} (10 * 2)", loop_result);

    // Using a function with loop expression
    let factorial = calculate_factorial(5);
    println!("calculate_factorial(5) = {}", factorial);

    println!("\n=== Complex Expression Chains ===\n");

    let data = vec![1, 2, 3, 4, 5];
    let result = data.iter().map(|x| x * 2).filter(|x| *x > 4).sum::<i32>(); // Entire chain is an expression
    println!("Iterator chain result: {}", result);

    println!("\n=== Expression in Different Contexts ===\n");

    // Array initialization with expressions
    let squares: [i32; 5] = [
        1 * 1,
        2 * 2,
        {
            let x = 3;
            x * x
        },
        if true { 16 } else { 0 },
        (|x: i32| x * x)(5),
    ];
    println!("squares: {:?}", squares);

    // Struct initialization with expressions
    let point = Point {
        x: {
            let base = 5;
            base * 2
        },
        y: if true { 20 } else { 0 },
    };
    println!("point: ({}, {})", point.x, point.y);

    println!("\n=== Early Return vs Expression Return ===\n");

    // Early return style
    println!("is_positive_early(5) = {}", is_positive_early(5));
    println!("is_positive_early(-5) = {}", is_positive_early(-5));

    // Expression style
    println!("is_positive_expr(5) = {}", is_positive_expr(5));
    println!("is_positive_expr(-5) = {}", is_positive_expr(-5));

    println!("\n=== Comparison: Same Logic, Different Styles ===\n");

    let grade1 = calculate_grade_statements(85);
    let grade2 = calculate_grade_expressions(85);
    println!("calculate_grade_statements(85) = '{}'", grade1);
    println!("calculate_grade_expressions(85) = '{}'", grade2);
}

// --- Expression-based function (implicit return) ---

fn double_value(x: i32) -> i32 {
    x * 2 // No semicolon - this expression is returned
}

fn triple_value(x: i32) -> i32 {
    let result = x * 3;
    result // Last expression is returned
}

// --- Comparison: explicit return vs expression ---

fn add_with_return(a: i32, b: i32) -> i32 {
    return a + b; // Explicit return statement
}

fn add_with_expression(a: i32, b: i32) -> i32 {
    a + b // Expression return (idiomatic Rust)
}

// --- If expression in function ---

fn maximum(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn get_sign(n: i32) -> &'static str {
    if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    }
}

// --- Match expression in function ---

fn classify_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1..=10 => "small",
        11..=100 => "medium",
        _ => "large",
    }
}

// --- Loop expression in function ---

fn calculate_factorial(n: u64) -> u64 {
    let mut result = 1;
    let mut counter = 1;
    loop {
        if counter > n {
            break result;
        }
        result *= counter;
        counter += 1;
    }
}

// --- Early return vs expression style ---

fn is_positive_early(n: i32) -> bool {
    if n > 0 {
        return true;
    }
    return false;
}

fn is_positive_expr(n: i32) -> bool {
    n > 0 // Simple expression
}

// --- Statement style vs expression style ---

fn calculate_grade_statements(score: i32) -> char {
    let grade: char;
    if score >= 90 {
        grade = 'A';
    } else if score >= 80 {
        grade = 'B';
    } else if score >= 70 {
        grade = 'C';
    } else if score >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }
    return grade;
}

fn calculate_grade_expressions(score: i32) -> char {
    if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}

// --- Helper struct for examples ---

struct Point {
    x: i32,
    y: i32,
}
