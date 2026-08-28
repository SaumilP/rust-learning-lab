//! Iterator: traverse a sequence without exposing its storage details.

struct Countdown {
    next: u8,
}

impl Countdown {
    fn new(start: u8) -> Self {
        Self { next: start }
    }
}

impl Iterator for Countdown {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == 0 {
            None
        } else {
            let current = self.next;
            self.next -= 1;
            Some(current)
        }
    }
}

fn main() {
    let values: Vec<_> = Countdown::new(3).collect();
    println!("Countdown: {values:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_yields_values_until_exhausted() {
        assert_eq!(Countdown::new(3).collect::<Vec<_>>(), vec![3, 2, 1]);
    }
}
