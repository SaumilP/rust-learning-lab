use multithreaded_server::{
    handlers, ConnectionCounter, Request, Response, Router, StatusCode, ThreadPool,
};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

fn main() {
    // Configuration
    let address = "127.0.0.1:7878";
    let thread_count = 4;

    println!("🦀 Starting Rust HTTP Server...");
    println!("Address: http://{}", address);
    println!("Thread pool size: {}", thread_count);
    println!("Press Ctrl+C to stop\n");

    // Create thread pool (like Java's ExecutorService)
    let pool = ThreadPool::new(thread_count);

    // Create connection counter
    let counter = ConnectionCounter::new();

    // Set up routes
    let router = Arc::new(
        Router::new()
            .get("/", |req| handlers::home_handler(req))
            .get("/hello", |req| handlers::hello_handler(req))
            .get("/sleep", |req| handlers::sleep_handler(req))
            .get("/stats", handlers::stats_handler(counter.clone()))
            .not_found(|_req| {
                Response::new(StatusCode::NotFound)
                    .content_type("text/html")
                    .body(
                        r#"
                        <html>
                        <body style="text-align: center; padding: 100px; font-family: Arial;">
                            <h1>404 - Not Found</h1>
                            <p>The page you're looking for doesn't exist.</p>
                            <p><a href="/">Go home</a></p>
                        </body>
                        </html>
                        "#
                        .to_string(),
                    )
            }),
    );

    // Start TCP listener
    let listener = TcpListener::bind(address).unwrap();

    println!("✓ Server is running!");
    println!("\nTry these URLs:");
    println!("  http://{}/", address);
    println!("  http://{}/hello", address);
    println!("  http://{}/sleep", address);
    println!("  http://{}/stats\n", address);

    // Accept connections and process them
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Connection failed: {}", e);
                continue;
            }
        };

        let router = Arc::clone(&router);
        let counter = counter.clone();

        // Execute request handling in thread pool
        pool.execute(move || {
            counter.increment();
            handle_connection(stream, &router);
        });
    }

    println!("\nShutting down.");
}

fn handle_connection(mut stream: TcpStream, router: &Router) {
    // Get peer address for logging
    let peer_addr = stream
        .peer_addr()
        .map(|addr| addr.to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // Parse the HTTP request
    let request = match Request::parse(&mut stream) {
        Ok(req) => {
            println!(
                "→ {:?} {} from {}",
                req.method, req.path, peer_addr
            );
            req
        }
        Err(e) => {
            eprintln!("Failed to parse request: {}", e);
            let response = Response::new(StatusCode::BadRequest)
                .body("Bad Request".to_string());

            if let Err(e) = response.send(&mut stream) {
                eprintln!("Failed to send error response: {}", e);
            }
            return;
        }
    };

    // Route the request to appropriate handler
    let response = router.handle(&request);

    // Send response
    match response.send(&mut stream) {
        Ok(_) => println!("← Response sent successfully"),
        Err(e) => eprintln!("Failed to send response: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_components() {
        // Test that server components can be created
        let _pool = ThreadPool::new(2);
        let _counter = ConnectionCounter::new();
        let _router = Router::new();

        // If we get here, basic initialization works
        assert!(true);
    }
}
