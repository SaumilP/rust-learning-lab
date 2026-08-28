fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

#[derive(Debug)]
struct Highlight<'a> {
    text: &'a str,
}

fn main() {
    let first = String::from("ownership");
    let second = String::from("borrowing");
    let selected = longest(&first, &second);
    let highlight = Highlight { text: selected };
    println!("Longest word: {}", highlight.text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_a_borrow_from_the_inputs() {
        assert_eq!(longest("short", "longer"), "longer");
    }
}
