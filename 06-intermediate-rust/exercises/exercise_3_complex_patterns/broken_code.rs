enum Event {
    Start,
    Move { x: i32, y: i32 },
    Message(String),
    Stop(i32),
}

fn describe_event(event: Event) -> String {
    // BUG: This match is not exhaustive and discards useful variant data.
    match event {
        Event::Start => String::from("started"),
        Event::Move { .. } => String::from("moved"),
    }
}

fn main() {
    let events = [
        Event::Start,
        Event::Move { x: 4, y: 7 },
        Event::Message(String::from("ready")),
        Event::Stop(0),
    ];

    for event in events {
        println!("{}", describe_event(event));
    }
}
