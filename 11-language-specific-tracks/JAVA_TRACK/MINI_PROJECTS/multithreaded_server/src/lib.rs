// Public exports for the library
pub mod handlers;
pub mod http;
pub mod router;
pub mod thread_pool;

// Re-export commonly used items
pub use handlers::ConnectionCounter;
pub use http::{Method, Request, Response, StatusCode};
pub use router::Router;
pub use thread_pool::ThreadPool;
