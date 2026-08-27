use crate::http::{Method, Request, Response, StatusCode};
use std::collections::HashMap;

/// Handler function type
type Handler = Box<dyn Fn(&Request) -> Response + Send + Sync>;

/// Route matching a method and path to a handler
struct Route {
    method: Method,
    path: String,
    handler: Handler,
}

/// HTTP router for matching requests to handlers
pub struct Router {
    routes: Vec<Route>,
    not_found_handler: Handler,
}

impl Router {
    /// Create a new router
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
            not_found_handler: Box::new(|_req| {
                Response::new(StatusCode::NotFound)
                    .content_type("text/html")
                    .body("<h1>404 Not Found</h1>".to_string())
            }),
        }
    }

    /// Register a GET route
    pub fn get<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: Method::GET,
            path: path.to_string(),
            handler: Box::new(handler),
        });
        self
    }

    /// Register a POST route
    pub fn post<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.push(Route {
            method: Method::POST,
            path: path.to_string(),
            handler: Box::new(handler),
        });
        self
    }

    /// Set custom 404 handler
    pub fn not_found<F>(mut self, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.not_found_handler = Box::new(handler);
        self
    }

    /// Handle an incoming request
    pub fn handle(&self, request: &Request) -> Response {
        // Try to find matching route
        for route in &self.routes {
            if route.method == request.method && self.path_matches(&route.path, &request.path) {
                return (route.handler)(request);
            }
        }

        // No route matched, return 404
        (self.not_found_handler)(request)
    }

    /// Check if a route path matches the request path
    fn path_matches(&self, route_path: &str, request_path: &str) -> bool {
        // Simple exact match (could be extended to support path parameters)
        route_path == request_path
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

/// Route builder helper
pub struct RouteBuilder {
    routes: HashMap<(Method, String), Handler>,
}

impl RouteBuilder {
    pub fn new() -> Self {
        RouteBuilder {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(mut self, method: Method, path: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.insert(
            (method, path.to_string()),
            Box::new(handler),
        );
        self
    }

    pub fn build(self) -> Router {
        let mut router = Router::new();

        for ((method, path), handler) in self.routes {
            router.routes.push(Route {
                method,
                path,
                handler,
            });
        }

        router
    }
}

impl Default for RouteBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::Method;

    fn create_test_request(method: Method, path: &str) -> Request {
        Request {
            method,
            path: path.to_string(),
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    #[test]
    fn test_router_get_route() {
        let router = Router::new().get("/test", |_req| {
            Response::new(StatusCode::OK).body("Hello".to_string())
        });

        let request = create_test_request(Method::GET, "/test");
        let response = router.handle(&request);

        assert_eq!(response.get_body(), "Hello");
    }

    #[test]
    fn test_router_not_found() {
        let router = Router::new();

        let request = create_test_request(Method::GET, "/nonexistent");
        let response = router.handle(&request);

        assert!(response.get_body().contains("404"));
    }

    #[test]
    fn test_router_post_route() {
        let router = Router::new().post("/submit", |_req| {
            Response::new(StatusCode::OK).body("Submitted".to_string())
        });

        let request = create_test_request(Method::POST, "/submit");
        let response = router.handle(&request);

        assert_eq!(response.get_body(), "Submitted");
    }

    #[test]
    fn test_custom_not_found() {
        let router = Router::new().not_found(|_req| {
            Response::new(StatusCode::NotFound).body("Custom 404".to_string())
        });

        let request = create_test_request(Method::GET, "/missing");
        let response = router.handle(&request);

        assert_eq!(response.get_body(), "Custom 404");
    }
}
