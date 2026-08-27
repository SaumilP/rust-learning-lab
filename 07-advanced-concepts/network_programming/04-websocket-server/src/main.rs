use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

struct ServerState {
    connections: AtomicU64,
    messages_received: AtomicU64,
    messages_sent: AtomicU64,
}

impl ServerState {
    fn new() -> Arc<Self> {
        Arc::new(ServerState {
            connections: AtomicU64::new(0),
            messages_received: AtomicU64::new(0),
            messages_sent: AtomicU64::new(0),
        })
    }

    fn connection_opened(&self) -> u64 {
        self.connections.fetch_add(1, Ordering::Relaxed) + 1
    }

    fn message_received(&self) {
        self.messages_received.fetch_add(1, Ordering::Relaxed);
    }

    fn message_sent(&self) {
        self.messages_sent.fetch_add(1, Ordering::Relaxed);
    }

    fn print_stats(&self) {
        println!("\n📊 Server Stats:");
        println!("   Total connections: {}", self.connections.load(Ordering::Relaxed));
        println!("   Messages received: {}", self.messages_received.load(Ordering::Relaxed));
        println!("   Messages sent: {}", self.messages_sent.load(Ordering::Relaxed));
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    state: Arc<ServerState>,
) {
    let conn_id = state.connection_opened();
    println!("✅ WebSocket connection #{} from {}", conn_id, addr);

    // Perform WebSocket handshake
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("❌ WebSocket handshake failed for {}: {}", addr, e);
            return;
        }
    };

    println!("🤝 WebSocket handshake completed for connection #{}", conn_id);

    // Split the stream for concurrent read/write
    let (mut write, mut read) = ws_stream.split();

    // Send welcome message
    let welcome_msg = Message::Text(format!(
        "Welcome! You are connection #{}. Send me messages and I'll echo them back!",
        conn_id
    ));

    if let Err(e) = write.send(welcome_msg).await {
        eprintln!("❌ Error sending welcome message: {}", e);
        return;
    }
    state.message_sent();

    // Handle incoming messages
    while let Some(msg_result) = read.next().await {
        match msg_result {
            Ok(msg) => {
                match msg {
                    Message::Text(text) => {
                        state.message_received();
                        println!("📨 Connection #{} sent: {}", conn_id, text);

                        // Echo back with prefix
                        let response = Message::Text(format!("Echo: {}", text));
                        if let Err(e) = write.send(response).await {
                            eprintln!("❌ Error sending message: {}", e);
                            break;
                        }
                        state.message_sent();
                    }
                    Message::Binary(data) => {
                        state.message_received();
                        println!("📦 Connection #{} sent {} bytes of binary data", conn_id, data.len());

                        // Echo back binary data
                        let response = Message::Binary(data);
                        if let Err(e) = write.send(response).await {
                            eprintln!("❌ Error sending binary data: {}", e);
                            break;
                        }
                        state.message_sent();
                    }
                    Message::Ping(data) => {
                        println!("🏓 Connection #{} sent ping", conn_id);
                        if let Err(e) = write.send(Message::Pong(data)).await {
                            eprintln!("❌ Error sending pong: {}", e);
                            break;
                        }
                    }
                    Message::Pong(_) => {
                        println!("🏓 Connection #{} sent pong", conn_id);
                    }
                    Message::Close(frame) => {
                        println!("👋 Connection #{} closing: {:?}", conn_id, frame);
                        break;
                    }
                    Message::Frame(_) => {
                        // Raw frames, typically not used
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error receiving message from connection #{}: {}", conn_id, e);
                break;
            }
        }
    }

    println!("🔌 Connection #{} closed", conn_id);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    let state = ServerState::new();

    println!("🚀 WebSocket Server listening on ws://{}", addr);
    println!("📝 Connect with: websocat ws://{}", addr);
    println!("📝 Or use browser: new WebSocket('ws://{}')\n", addr);

    // Spawn stats printer
    let state_clone = Arc::clone(&state);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
            state_clone.print_stats();
        }
    });

    // Accept connections
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let state = Arc::clone(&state);
                tokio::spawn(async move {
                    handle_connection(stream, addr, state).await;
                });
            }
            Err(e) => {
                eprintln!("❌ Error accepting connection: {}", e);
            }
        }
    }
}
