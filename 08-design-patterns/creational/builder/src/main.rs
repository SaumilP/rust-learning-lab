struct Greeter {
    message: String,
}

struct GreetBuilder {
    greeting: String,
}

impl GreetBuilder {
    fn new() -> GreetBuilder {
        GreetBuilder {
            greeting: "".to_string(),
        }
    }

    fn set_greeting(mut self, greeting: String) -> GreetBuilder {
        self.greeting = greeting;
        self
    }

    fn build(self) -> Greeter {
        Greeter {
            message: self.greeting,
        }
    }
}

fn main() {
    let g = GreetBuilder::new()
        .set_greeting("Greeting Fellow Rustacean!!!".to_string())
        .build();
    println!("Result: {}", g.message);
}
