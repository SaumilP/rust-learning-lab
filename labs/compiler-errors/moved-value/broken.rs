pub fn reuse_after_move() {
    let message = String::from("hello");
    let stored = message;
    println!("{message} {stored}");
}
