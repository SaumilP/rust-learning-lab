// Example: Inspecting expressions with dbg!
//
// dbg! prints the file, line, expression, and value to standard error. It also
// returns the value, so it can be inserted into an expression temporarily.

fn discounted_price(price: f64, percent: f64) -> f64 {
    let multiplier = dbg!(1.0 - percent / 100.0);
    dbg!(price * multiplier)
}

fn main() {
    let original_price = 80.0;
    let final_price = discounted_price(original_price, 25.0);
    println!("Final price: {final_price:.2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_percentage_discount() {
        assert_eq!(discounted_price(80.0, 25.0), 60.0);
    }
}
