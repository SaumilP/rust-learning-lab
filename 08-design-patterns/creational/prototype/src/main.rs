//! Prototype: clone a configured value and then customize the copy.

#[derive(Clone, Debug, PartialEq, Eq)]
struct Report {
    title: String,
    sections: Vec<String>,
    confidential: bool,
}

fn main() {
    let template = Report {
        title: "Monthly report".to_string(),
        sections: vec!["Summary".to_string(), "Metrics".to_string()],
        confidential: true,
    };

    let mut august = template.clone();
    august.title = "August report".to_string();

    println!("Template: {}", template.title);
    println!("Copy: {}", august.title);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_preserves_the_template_and_allows_customization() {
        let template = Report {
            title: "Template".to_string(),
            sections: vec!["Summary".to_string()],
            confidential: false,
        };
        let mut copy = template.clone();
        copy.title = "Customer report".to_string();

        assert_eq!(template.title, "Template");
        assert_eq!(copy.sections, template.sections);
    }
}
