pub fn greeting<'a>() -> &'a str {
    let message = String::from("hello");
    &message
}
