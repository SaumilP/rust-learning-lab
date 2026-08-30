pub fn reuse_after_move() {
    let message = String::from("hello");
    let stored = message.clone();
    println!("{message} {stored}");
}
