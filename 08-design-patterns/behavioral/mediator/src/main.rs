//! Mediator: centralize communication rules between collaborating components.

struct ChatRoom {
    members: Vec<String>,
}

impl ChatRoom {
    fn new(members: &[&str]) -> Self {
        Self {
            members: members.iter().map(|name| (*name).to_string()).collect(),
        }
    }

    fn broadcast(&self, sender: &str, message: &str) -> Vec<String> {
        self.members
            .iter()
            .filter(|member| member.as_str() != sender)
            .map(|member| format!("to {member}: {sender} says {message}"))
            .collect()
    }
}

fn main() {
    let room = ChatRoom::new(&["Alice", "Bob", "Chen"]);
    for delivery in room.broadcast("Alice", "Hello") {
        println!("{delivery}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mediator_routes_messages_without_sender_knowing_recipients() {
        let room = ChatRoom::new(&["Alice", "Bob", "Chen"]);
        let deliveries = room.broadcast("Alice", "Hi");
        assert_eq!(deliveries.len(), 2);
        assert!(deliveries
            .iter()
            .all(|message| !message.starts_with("to Alice")));
    }
}
