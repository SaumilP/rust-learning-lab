//! Decorator: wrap a value to add behaviour without changing the wrapped type.

trait Notifier {
    fn message(&self, text: &str) -> String;
}

struct PlainNotifier;

impl Notifier for PlainNotifier {
    fn message(&self, text: &str) -> String {
        text.to_string()
    }
}

struct Urgent<N> {
    inner: N,
}

impl<N: Notifier> Notifier for Urgent<N> {
    fn message(&self, text: &str) -> String {
        self.inner.message(&format!("URGENT: {text}"))
    }
}

struct WithSignature<N> {
    inner: N,
    sender: String,
}

impl<N: Notifier> Notifier for WithSignature<N> {
    fn message(&self, text: &str) -> String {
        format!("{}\n-- {}", self.inner.message(text), self.sender)
    }
}

fn main() {
    let notifier = WithSignature {
        inner: Urgent {
            inner: PlainNotifier,
        },
        sender: "Operations".to_string(),
    };

    println!("{}", notifier.message("Database is unavailable"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decorators_can_be_composed() {
        let notifier = WithSignature {
            inner: Urgent {
                inner: PlainNotifier,
            },
            sender: "Team".to_string(),
        };

        assert_eq!(notifier.message("Deploy"), "URGENT: Deploy\n-- Team");
    }
}
