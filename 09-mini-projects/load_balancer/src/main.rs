mod config;
mod health;
mod lb;
mod plugins;

use crate::config::load_config;
use crate::lb::LoadBalancer;
use crate::plugins::{HeaderPlugin, LoggingPlugin};
use arc_swap::ArcSwap;
use hyper::server::conn::http1;
use hyper_util::client::legacy::Client;
use hyper_util::rt::{TokioExecutor, TokioIo};
use notify::{RecursiveMode, Watcher};
use std::fmt;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

#[derive(Debug)]
struct ServiceError(Box<dyn std::error::Error + Send + Sync + 'static>);

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Service error: {}", self.0)
    }
}

impl std::error::Error for ServiceError {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "config.yaml";

    // 1. Initial Configuration Setup
    // We wrap the config in ArcSwap to allow lock-free hot reloading
    let mut raw_conf = load_config(config_path);

    // Initialize live_backends with all backends until first health check completes
    raw_conf.live_backends = raw_conf.all_backends.clone();
    let shared_config = Arc::new(ArcSwap::from_pointee(raw_conf));

    // 2. Setup Shared HTTP Client (for backend communication)
    // Reusing this client is critical for TCP connection pooling
    let client = Client::builder(TokioExecutor::new()).build_http();

    // 3. Initialize the Load Balancer Engine
    let lb = Arc::new(LoadBalancer {
        config: Arc::clone(&shared_config),
        client,
        plugins: vec![
            Box::new(LoggingPlugin),
            Box::new(HeaderPlugin {
                key: "X-Proxy-Powered-By".to_string(),
                value: "Rust-Hyper-1.0".to_string(),
            }),
        ],
        rr_counter: AtomicUsize::new(0),
    });

    // 4. Spawn File Watcher Task (Hot-Reloading)
    let config_for_watcher = Arc::clone(&shared_config);
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(event) = res {
            if event.kind.is_modify() {
                println!("🔄 Config file modified. Reloading backends...");
                let mut new_conf = load_config("config.yaml");
                // Reset live_backends to full list on reload; health check will prune it
                new_conf.live_backends = new_conf.all_backends.clone();
                config_for_watcher.store(Arc::new(new_conf));
            }
        }
    })?;
    watcher.watch(config_path.as_ref(), RecursiveMode::NonRecursive)?;

    // 5. Spawn Active Health Checker Task
    let config_for_health = Arc::clone(&shared_config);
    tokio::spawn(async move {
        println!("🏥 Health checker started.");
        health::start_health_checks(config_for_health, Duration::from_secs(10)).await;
    });

    // 6. Start the Server Binding
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await?;
    println!(
        "🚀 High-performance Load Balancer listening on http://{}",
        addr
    );

    loop {
        // Accept incoming TCP connections
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        // Clone the LB pointer for the task
        let lb_ref = Arc::clone(&lb);

        // Spawn a green thread (Tokio task) for every connection
        tokio::task::spawn(async move {
            let service = hyper::service::service_fn(move |req| {
                let lb = Arc::clone(&lb_ref);
                async move {
                    match lb.proxy(req).await {
                        Ok(resp) => Ok::<_, ServiceError>(resp),
                        Err(e) => Err::<hyper::Response<http_body_util::Full<bytes::Bytes>>, _>(
                            ServiceError(e),
                        ),
                    }
                }
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("❌ Error serving connection: {:?}", err);
            }
        });
    }
}
