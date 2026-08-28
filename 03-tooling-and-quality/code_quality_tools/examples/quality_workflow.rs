// Example: A small program suitable for the fmt/check/test workflow

#[derive(Debug, PartialEq)]
struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn from_celsius(celsius: f64) -> Self {
        Self { celsius }
    }

    fn fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }
}

fn main() {
    let freezing = Temperature::from_celsius(0.0);
    println!("{freezing:?}");
    println!("Freezing point: {:.1}°F", freezing.fahrenheit());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_celsius_to_fahrenheit() {
        let temperature = Temperature::from_celsius(100.0);
        assert_eq!(temperature.fahrenheit(), 212.0);
    }
}
