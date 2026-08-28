// Example: Tracing a calculation with println!

fn running_total(values: &[i32]) -> i32 {
    let mut total = 0;

    for (index, value) in values.iter().enumerate() {
        println!("before step {index}: total={total}, next={value}");
        total += value;
        println!("after step {index}: total={total}");
    }

    total
}

fn main() {
    let values = [3, 5, -2];
    let total = running_total(&values);
    println!("Final total: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_positive_and_negative_values() {
        assert_eq!(running_total(&[3, 5, -2]), 6);
    }
}
