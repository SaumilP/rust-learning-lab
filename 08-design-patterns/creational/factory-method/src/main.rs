//! Factory Method: let each creator choose the product used by shared logic.

trait Parser {
    fn parse(&self, input: &str) -> Vec<String>;
}

struct CsvParser;
struct LinesParser;

impl Parser for CsvParser {
    fn parse(&self, input: &str) -> Vec<String> {
        input
            .split(',')
            .map(str::trim)
            .map(str::to_string)
            .collect()
    }
}

impl Parser for LinesParser {
    fn parse(&self, input: &str) -> Vec<String> {
        input.lines().map(str::to_string).collect()
    }
}

trait Importer {
    fn create_parser(&self) -> Box<dyn Parser>;

    fn import(&self, input: &str) -> usize {
        self.create_parser().parse(input).len()
    }
}

struct CsvImporter;
struct LinesImporter;

impl Importer for CsvImporter {
    fn create_parser(&self) -> Box<dyn Parser> {
        Box::new(CsvParser)
    }
}

impl Importer for LinesImporter {
    fn create_parser(&self) -> Box<dyn Parser> {
        Box::new(LinesParser)
    }
}

fn main() {
    println!(
        "Imported {} CSV values",
        CsvImporter.import("one, two, three")
    );
    println!("Imported {} lines", LinesImporter.import("one\ntwo"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creator_selects_the_parser_used_by_shared_import_logic() {
        assert_eq!(CsvImporter.import("a,b,c"), 3);
        assert_eq!(LinesImporter.import("a\nb"), 2);
    }
}
