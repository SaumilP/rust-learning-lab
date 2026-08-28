fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

fn add_period(text: &mut String) {
    if !text.ends_with('.') {
        text.push('.');
    }
}

fn main() {
    let mut message = String::from("Rust makes ownership visible");
    println!("Words: {}", word_count(&message));

    add_period(&mut message);
    println!("Updated: {message}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowed_operations_leave_the_owner_usable() {
        let mut text = String::from("hello world");
        assert_eq!(word_count(&text), 2);
        add_period(&mut text);
        assert_eq!(text, "hello world.");
    }
}
