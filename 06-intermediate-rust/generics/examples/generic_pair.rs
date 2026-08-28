#[derive(Debug, PartialEq)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self { first, second }
    }
}

impl<T: Ord> Pair<T> {
    fn larger(&self) -> &T {
        if self.first >= self.second {
            &self.first
        } else {
            &self.second
        }
    }
}

fn first<T>(values: &[T]) -> Option<&T> {
    values.first()
}

fn main() {
    let numbers = Pair::new(12, 7);
    println!("Pair: {numbers:?}");
    println!("Larger: {}", numbers.larger());
    println!("First word: {:?}", first(&["rust", "safe", "fast"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_works_with_ordered_types() {
        assert_eq!(*Pair::new('a', 'z').larger(), 'z');
    }
}
