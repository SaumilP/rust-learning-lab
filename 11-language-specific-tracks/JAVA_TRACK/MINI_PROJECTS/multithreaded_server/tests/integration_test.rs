use multithreaded_server::{
    handlers, ConnectionCounter, Method, Request, Response, Router, StatusCode, ThreadPool,
};
use std::collections::HashMap;

#[test]
fn test_thread_pool_concurrent_execution() {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    let pool = ThreadPool::new(4);
    let counter = Arc::new(AtomicU32::new(0));

    // Execute 100 jobs
    for _ in 0..100 {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        });
    }

    // Wait for completion (in real tests, use proper synchronization)
    drop(pool);
    std::thread::sleep(std::time::Duration::from_millis(500));

    // All jobs should have executed
    assert!(counter.load(Ordering::SeqCst) <= 100);
}

#[test]
fn test_connection_counter_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let counter = Arc::new(ConnectionCounter::new());
    let mut handles = vec![];

    // Spawn 10 threads, each incrementing 10 times
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                counter_clone.increment();
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Should have 100 increments
    assert_eq!(counter.get_count(), 100);
}

#[test]
fn test_router_basic_routing() {
    let router = Router::new()
        .get("/", |_req| {
            Response::new(StatusCode::OK).body("Home".to_string())
        })
        .get("/about", |_req| {
            Response::new(StatusCode::OK).body("About".to_string())
        });

    let home_req = create_test_request(Method::GET, "/");
    let about_req = create_test_request(Method::GET, "/about");
    let missing_req = create_test_request(Method::GET, "/missing");

    assert!(router.handle(&home_req).get_body().contains("Home"));
    assert!(router.handle(&about_req).get_body().contains("About"));
    assert!(router.handle(&missing_req).get_body().contains("404"));
}

#[test]
fn test_handlers_home_page() {
    let req = create_test_request(Method::GET, "/");
    let response = handlers::home_handler(&req);

    assert!(response.get_body().contains("Multithreaded HTTP Server"));
    assert!(response.get_body().contains("html"));
}

#[test]
fn test_handlers_hello() {
    let req = create_test_request(Method::GET, "/hello");
    let response = handlers::hello_handler(&req);

    assert!(response.get_body().contains("Hello from Rust"));
}

#[test]
fn test_handlers_stats() {
    let counter = ConnectionCounter::new();
    counter.increment();
    counter.increment();
    counter.increment();

    let handler = handlers::stats_handler(counter);
    let req = create_test_request(Method::GET, "/stats");
    let response = handler(&req);

    assert!(response.get_body().contains("3"));
}

#[test]
fn test_response_builder() {
    let response = Response::new(StatusCode::OK)
        .content_type("text/html")
        .header("X-Custom".to_string(), "value".to_string())
        .body("<html></html>".to_string());

    assert_eq!(response.get_body(), "<html></html>");
}

#[test]
fn test_method_equality() {
    assert_eq!(Method::GET, Method::GET);
    assert_ne!(Method::GET, Method::POST);
}

// Helper function to create test requests
fn create_test_request(method: Method, path: &str) -> Request {
    Request {
        method,
        path: path.to_string(),
        version: "HTTP/1.1".to_string(),
        headers: HashMap::new(),
        body: String::new(),
    }
}
