fn larger(left: i32, right: i32) -> i32 {
    if left > right { left } else { right }
}

fn main() {
    println!("{}", larger(9, 4));
}
