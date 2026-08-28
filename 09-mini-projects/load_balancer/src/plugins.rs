use hyper::body::Incoming;
use hyper::header::{HeaderName, HeaderValue};
use hyper::Request;
use std::str::FromStr;

/// The core trait for all Load Balancer extensions/plugins
/// We require Send + Sync to ensure plugins can be shared across Tokio threads
pub trait Plugin: Send + Sync {
    // Returns the human-readable name of the plugin for debugging/logging purpose
    fn name(&self) -> &str;

    // Plugins can modify the request before it hits the backend
    fn on_request(&self, _req: &mut Request<Incoming>);
}

// --- Header Injector ---

pub struct HeaderPlugin {
    pub key: String,
    pub value: String,
}

impl Plugin for HeaderPlugin {
    fn name(&self) -> &str {
        "HeaderInjector"
    }

    fn on_request(&self, req: &mut Request<Incoming>) {
        if let (Ok(name), Ok(val)) = (
            HeaderName::from_str(&self.key),
            HeaderValue::from_str(&self.value),
        ) {
            req.headers_mut().insert(name, val);
        }
    }
}

// --- Logging Plugin ---
pub struct LoggingPlugin;

impl Plugin for LoggingPlugin {
    fn name(&self) -> &str {
        "RequestLogger"
    }

    fn on_request(&self, req: &mut Request<Incoming>) {
        println!(
            "[LOG] {} Incoming request: {} {}",
            self.name(),
            req.method(),
            req.uri()
        );
    }
}
