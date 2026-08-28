#[derive(Debug, PartialEq)]
enum OrderState {
    Pending,
    Shipped { tracking: String },
    Delivered,
}

fn description(state: &OrderState) -> String {
    match state {
        OrderState::Pending => String::from("waiting to ship"),
        OrderState::Shipped { tracking } => format!("shipped as {tracking}"),
        OrderState::Delivered => String::from("delivered"),
    }
}

fn main() {
    let states = [
        OrderState::Pending,
        OrderState::Shipped {
            tracking: String::from("RUST-42"),
        },
        OrderState::Delivered,
    ];

    for state in states {
        println!("{}", description(&state));
    }
}
