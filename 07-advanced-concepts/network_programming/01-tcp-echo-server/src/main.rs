use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

struct ServerStats {
    total_connections: AtomicU64,
    active_connections: AtomicU64,
    bytes_received: AtomicU64,
    bytes_sent: AtomicU64,
}

impl ServerStats {
    fn new() -> Arc<Self> {
        Arc::new(ServerStats {
            total_connections: AtomicU64::new(0),
            active_connections: AtomicU64::new(0),
            bytes_received: AtomicU64::new(0),
            bytes_sent: AtomicU64::new(0),
        })
    }

    fn connection_started(&self) {
        self.total_connections.fetch_add(1, Ordering::Relaxed);
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    fn connection_ended(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    fn record_io(&self, received: u64, sent: u64) {
        self.bytes_received.fetch_add(received, Ordering::Relaxed);
        self.bytes_sent.fetch_add(sent, Ordering::Relaxed);
    }

    fn print_stats(&self) {
        println!("\n📊 Server Statistics:");
        println!("   Total connections: {}", self.total_connections.load(Ordering::Relaxed));
        println!("   Active connections: {}", self.active_connections.load(Ordering::Relaxed));
        println!("   Bytes received: {}", self.bytes_received.load(Ordering::Relaxed));
        println!("   Bytes sent: {}", self.bytes_sent.load(Ordering::Relaxed));
    }
}

async fn handle_client(mut socket: TcpStream, addr: std::net::SocketAddr, stats: Arc<ServerStats>) {
    println!("✅ New connection from: {}", addr);
    stats.connection_started();

    let mut buf = vec![0; 1024];
    let mut local_received = 0u64;
    let mut local_sent = 0u64;

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => {
                // Connection closed
                println!("🔌 Connection closed: {}", addr);
                break;
            }
            Ok(n) => {
                local_received += n as u64;

                // Echo back the data
                if let Err(e) = socket.write_all(&buf[..n]).await {
                    eprintln!("❌ Error writing to {}: {}", addr, e);
                    break;
                }

                local_sent += n as u64;

                // Log the echoed message
                if let Ok(msg) = std::str::from_utf8(&buf[..n]) {
                    println!("📡 {} -> {}", addr, msg.trim());
                }
            }
            Err(e) => {
                eprintln!("❌ Error reading from {}: {}", addr, e);
                break;
            }
        }
    }

    stats.record_io(local_received, local_sent);
    stats.connection_ended();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    let stats = ServerStats::new();

    println!("🚀 TCP Echo Server listening on {}", addr);
    println!("📝 Connect with: telnet {} or nc {}\n", addr, addr);

    // Spawn stats printer
    let stats_clone = Arc::clone(&stats);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            stats_clone.print_stats();
        }
    });

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                let stats = Arc::clone(&stats);
                tokio::spawn(async move {
                    handle_client(socket, addr, stats).await;
                });
            }
            Err(e) => {
                eprintln!("❌ Error accepting connection: {}", e);
            }
        }
    }
}
