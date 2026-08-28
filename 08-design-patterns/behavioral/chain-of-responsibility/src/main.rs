//! Chain of Responsibility: pass a request through ordered handlers.

trait Handler {
    fn handle(&self, request: &str) -> Option<String>;
}

struct Authentication;
struct RateLimit;
struct Application;

impl Handler for Authentication {
    fn handle(&self, request: &str) -> Option<String> {
        (!request.contains("token=")).then(|| "rejected: missing token".to_string())
    }
}

impl Handler for RateLimit {
    fn handle(&self, request: &str) -> Option<String> {
        request
            .contains("blocked=true")
            .then(|| "rejected: rate limit".to_string())
    }
}

impl Handler for Application {
    fn handle(&self, _request: &str) -> Option<String> {
        Some("request handled".to_string())
    }
}

fn process(request: &str, handlers: &[Box<dyn Handler>]) -> String {
    handlers
        .iter()
        .find_map(|handler| handler.handle(request))
        .unwrap_or_else(|| "unhandled".to_string())
}

fn main() {
    let chain: Vec<Box<dyn Handler>> = vec![
        Box::new(Authentication),
        Box::new(RateLimit),
        Box::new(Application),
    ];
    println!("{}", process("token=abc", &chain));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chain() -> Vec<Box<dyn Handler>> {
        vec![
            Box::new(Authentication),
            Box::new(RateLimit),
            Box::new(Application),
        ]
    }

    #[test]
    fn first_matching_handler_stops_the_chain() {
        assert_eq!(
            process("no credentials", &chain()),
            "rejected: missing token"
        );
        assert_eq!(process("token=abc", &chain()), "request handled");
    }
}
