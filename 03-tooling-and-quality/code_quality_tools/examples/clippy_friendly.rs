// Example: Writing code that is easy for Clippy to check
//
// Run: rustc --edition=2021 -D warnings clippy_friendly.rs

fn average(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    Some(values.iter().sum::<f64>() / values.len() as f64)
}

fn normalized_names(names: &[&str]) -> Vec<String> {
    names
        .iter()
        .map(|name| name.trim().to_lowercase())
        .filter(|name| !name.is_empty())
        .collect()
}

fn main() {
    let scores = [8.0, 9.5, 7.5];
    println!("Average score: {:?}", average(&scores));

    let names = [" Alice ", "", " BOB"];
    println!("Normalized names: {:?}", normalized_names(&names));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_requires_at_least_one_value() {
        assert_eq!(average(&[]), None);
        assert_eq!(average(&[2.0, 4.0]), Some(3.0));
    }
}
