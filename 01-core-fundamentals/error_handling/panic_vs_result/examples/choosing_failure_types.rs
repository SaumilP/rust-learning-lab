#[derive(Debug, PartialEq)]
enum DivisionError {
    DivisionByZero,
}

fn divide(numerator: i32, denominator: i32) -> Result<i32, DivisionError> {
    if denominator == 0 {
        Err(DivisionError::DivisionByZero)
    } else {
        Ok(numerator / denominator)
    }
}

fn first(values: &[i32]) -> Option<i32> {
    values.first().copied()
}

fn require_positive(value: i32) -> i32 {
    // This assertion represents a programmer invariant, not bad user input.
    assert!(value > 0, "value must be positive");
    value
}

fn main() {
    println!("10 / 2 = {:?}", divide(10, 2));
    println!("10 / 0 = {:?}", divide(10, 0));

    println!("First value: {:?}", first(&[4, 8, 15]));
    println!("Empty slice: {:?}", first(&[]));

    let validated = require_positive(7);
    println!("Validated invariant: {validated}");
    println!("Use panic for broken invariants, Result for expected failures.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_failure_uses_result() {
        assert_eq!(divide(10, 0), Err(DivisionError::DivisionByZero));
    }

    #[test]
    fn absence_uses_option() {
        assert_eq!(first(&[]), None);
    }

    #[test]
    #[should_panic(expected = "value must be positive")]
    fn broken_invariant_panics() {
        require_positive(0);
    }
}
