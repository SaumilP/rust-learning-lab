// BUG: The required comparison bound and empty-slice handling are missing.
fn largest<T>(values: &[T]) -> &T {
    let mut largest = &values[0];
    for value in values {
        if value > largest {
            largest = value;
        }
    }
    largest
}

fn main() {
    println!("Largest number: {:?}", largest(&[12, 34, 8]));
    println!("Largest letter: {:?}", largest(&['a', 'y', 'm']));
    println!("Empty: {:?}", largest::<i32>(&[]));
}
