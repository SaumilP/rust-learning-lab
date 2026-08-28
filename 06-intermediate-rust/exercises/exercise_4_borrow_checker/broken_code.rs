// BUG: The return lifetime is ambiguous and the function returns local data.
fn choose_label(primary: &str, fallback: &str) -> &str {
    let selected = if primary.is_empty() {
        fallback.to_string()
    } else {
        primary.to_string()
    };
    &selected
}

fn main() {
    println!("Selected: {}", choose_label("production", "stable"));
    println!("Selected: {}", choose_label("", "stable"));
}
