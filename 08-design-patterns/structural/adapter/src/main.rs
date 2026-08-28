//! Adapter: present an existing type through the interface a client expects.

trait TemperatureSource {
    fn celsius(&self) -> f64;
}

/// Imagine this type comes from an older library that cannot be changed.
struct LegacyThermometer {
    fahrenheit: f64,
}

impl LegacyThermometer {
    fn read_fahrenheit(&self) -> f64 {
        self.fahrenheit
    }
}

struct CelsiusAdapter {
    legacy: LegacyThermometer,
}

impl CelsiusAdapter {
    fn new(legacy: LegacyThermometer) -> Self {
        Self { legacy }
    }
}

impl TemperatureSource for CelsiusAdapter {
    fn celsius(&self) -> f64 {
        (self.legacy.read_fahrenheit() - 32.0) * 5.0 / 9.0
    }
}

fn display_temperature(source: &dyn TemperatureSource) -> String {
    format!("Current temperature: {:.1} °C", source.celsius())
}

fn main() {
    let legacy = LegacyThermometer { fahrenheit: 77.0 };
    let adapter = CelsiusAdapter::new(legacy);
    println!("{}", display_temperature(&adapter));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_the_legacy_reading_for_the_new_client() {
        let adapter = CelsiusAdapter::new(LegacyThermometer { fahrenheit: 77.0 });
        assert!((adapter.celsius() - 25.0).abs() < f64::EPSILON);
    }
}
