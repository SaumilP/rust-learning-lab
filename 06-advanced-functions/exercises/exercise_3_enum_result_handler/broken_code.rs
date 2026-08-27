/// Broken Enum Error Handler - Fix the bugs!

#[derive(Debug)]
enum CustomError {
    InvalidNumber,
    Empty,
    OutOfRange,
    DivisionByZero,
}

fn parse_number(s: &str) -> Result<i32, CustomError> {
    if s.is_empty() {
        return Err(CustomError::Empty);
    }

    match s.parse::<i32>() {
        Ok(n) => {
            // ❌ BUG 1: Range check has wrong logic
            if n < 0 || n > 1000 {
                Ok(n)  // Should be Err for out of range!
            } else {
                Ok(n)
            }
        }
        Err(_) => Err(CustomError::InvalidNumber),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, CustomError> {
    if b == 0 {
        Err(CustomError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn main() {
    let test_values = vec!["42", "abc", "", "9999"];

    for val in test_values {
        // ❌ BUG 2: Pattern matching is incomplete - missing cases
        match parse_number(val) {
            Ok(n) => println!("Parsing \"{}\": Ok({})", val, n),
            Err(CustomError::InvalidNumber) => println!("Parsing \"{}\": Err(InvalidNumber)", val),
            // Missing Empty, OutOfRange cases!
        }
    }

    println!("\nHandling division...");
    // ❌ BUG 3: Error message doesn't use error information
    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(_) => println!("Error occurred"),  // Should print error type
    }

    let result = divide(100, 2);
    match result {
        Ok(r) => println!("Division result: {}", r),
        Err(e) => println!("Error: {:?}", e),
    }
}
