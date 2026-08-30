fn classify(number: i32) -> &'static str {
    if number < 0 { "negative" } else if number == 0 { "zero" } else { "positive" }
}

fn main() {
    println!("{}", classify(-2));
}
