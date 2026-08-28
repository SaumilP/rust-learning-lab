//! Template Method: define an algorithm skeleton with customizable steps.

trait DataPipeline {
    fn parse(&self, input: &str) -> Vec<String>;

    fn valid(&self, value: &str) -> bool {
        !value.is_empty()
    }

    fn run(&self, input: &str) -> Vec<String> {
        self.parse(input)
            .into_iter()
            .filter(|value| self.valid(value))
            .collect()
    }
}

struct CsvPipeline;
struct LinePipeline;

impl DataPipeline for CsvPipeline {
    fn parse(&self, input: &str) -> Vec<String> {
        input
            .split(',')
            .map(str::trim)
            .map(str::to_string)
            .collect()
    }
}

impl DataPipeline for LinePipeline {
    fn parse(&self, input: &str) -> Vec<String> {
        input.lines().map(str::trim).map(str::to_string).collect()
    }
}

fn main() {
    println!("CSV: {:?}", CsvPipeline.run("one, , two"));
    println!("Lines: {:?}", LinePipeline.run("one\n\ntwo"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_algorithm_uses_the_custom_parse_step() {
        assert_eq!(CsvPipeline.run("a, ,b"), vec!["a", "b"]);
    }
}
