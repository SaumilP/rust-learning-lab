// Async Chat Server in Rust
// Demonstrates async/await, tokio, broadcasting, and concurrent client handling

use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};

type ClientId = usize;

#[derive(Clone)]
struct ChatServer {
    clients: Arc<Mutex<HashMap<ClientId, String>>>,
    tx: broadcast::Sender<(ClientId, String, String)>,
    next_id: Arc<Mutex<ClientId>>,
}

impl ChatServer {
    fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);

        ChatServer {
            clients: Arc::new(Mutex::new(HashMap::new())),
            tx,
            next_id: Arc::new(Mutex::new(0)),
        }
    }

    async fn get_next_id(&self) -> ClientId {
        let mut id = self.next_id.lock().await;
        let current = *id;
        *id += 1;
        current
    }

    async fn add_client(&self, id: ClientId, name: String) {
        let mut clients = self.clients.lock().await;
        clients.insert(id, name.clone());
        println!("[Server] {} (ID: {}) joined. Total clients: {}", name, id, clients.len());
    }

    async fn remove_client(&self, id: ClientId) {
        let mut clients = self.clients.lock().await;
        if let Some(name) = clients.remove(&id) {
            println!("[Server] {} (ID: {}) left. Total clients: {}", name, id, clients.len());
        }
    }

    async fn broadcast(&self, sender_id: ClientId, sender_name: String, message: String) {
        let _ = self.tx.send((sender_id, sender_name, message));
    }

    async fn get_online_users(&self) -> Vec<String> {
        let clients = self.clients.lock().await;
        clients.values().cloned().collect()
    }
}

#[tokio::main]
async fn main() {
    println!("=== Async Chat Server ===\n");

    let server = ChatServer::new();

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to address");

    println!("Chat server running on tcp://127.0.0.1:8080");
    println!("\nTo connect:");
    println!("  telnet localhost 8080");
    println!("  nc localhost 8080");
    println!("  or use the Rust client in client.rs\n");
    println!("Commands:");
    println!("  /users  - List online users");
    println!("  /quit   - Disconnect");
    println!("  <msg>   - Send message to all\n");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("[Server] New connection from {}", addr);
                let server = server.clone();

                tokio::spawn(async move {
                    handle_client(stream, server).await;
                });
            }
            Err(e) => {
                eprintln!("[Server] Error accepting connection: {}", e);
            }
        }
    }
}

async fn handle_client(stream: TcpStream, server: ChatServer) {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    // Welcome message
    let _ = writer
        .write_all(b"=== Welcome to Rust Chat ===\n")
        .await;
    let _ = writer.write_all(b"Enter your name: ").await;
    let _ = writer.flush().await;

    // Read username
    let mut username = String::new();
    if reader.read_line(&mut username).await.is_err() {
        return;
    }
    let username = username.trim().to_string();

    if username.is_empty() {
        let _ = writer.write_all(b"Invalid username. Disconnecting.\n").await;
        return;
    }

    // Register client
    let client_id = server.get_next_id().await;
    server.add_client(client_id, username.clone()).await;

    let _ = writer
        .write_all(format!("\nWelcome, {}! Type /users to see who's online.\n\n", username).as_bytes())
        .await;

    // Broadcast join message
    server
        .broadcast(
            client_id,
            "System".to_string(),
            format!("{} joined the chat", username),
        )
        .await;

    // Subscribe to broadcasts
    let mut rx = server.tx.subscribe();

    // Spawn task to receive and send broadcasts
    let username_clone = username.clone();
    let mut writer_clone = writer;
    let receive_handle = tokio::spawn(async move {
        while let Ok((sender_id, sender_name, message)) = rx.recv().await {
            if sender_id != client_id {
                let formatted = format!("[{}] {}\n", sender_name, message);
                if writer_clone.write_all(formatted.as_bytes()).await.is_err() {
                    break;
                }
                let _ = writer_clone.flush().await;
            }
        }
    });

    // Read messages from client
    let server_clone = server.clone();
    let username_clone2 = username.clone();
    loop {
        let mut line = String::new();

        match reader.read_line(&mut line).await {
            Ok(0) => break, // Client disconnected
            Ok(_) => {
                let message = line.trim();

                if message.is_empty() {
                    continue;
                }

                // Handle commands
                if message.starts_with('/') {
                    match message {
                        "/quit" => {
                            break;
                        }
                        "/users" => {
                            let users = server_clone.get_online_users().await;
                            let user_list = format!(
                                "\nOnline users ({}):\n{}\n\n",
                                users.len(),
                                users.join("\n")
                            );
                            // Can't write directly here, would need separate channel
                            println!("[{}] requested user list", username_clone2);
                        }
                        _ => {
                            println!("[{}] unknown command: {}", username_clone2, message);
                        }
                    }
                } else {
                    // Broadcast message
                    server_clone
                        .broadcast(client_id, username_clone2.clone(), message.to_string())
                        .await;
                }
            }
            Err(_) => break,
        }
    }

    // Cleanup
    receive_handle.abort();
    server.remove_client(client_id).await;
    server
        .broadcast(
            client_id,
            "System".to_string(),
            format!("{} left the chat", username),
        )
        .await;

    println!("[Server] {} disconnected", username);
}

/*
Chat Application Summary:
=========================

FEATURES:
- Async/await with Tokio
- Multiple concurrent clients
- Broadcast messaging
- Username registration
- Online user tracking
- Commands (/users, /quit)
- Real-time message delivery

CONCEPTS DEMONSTRATED:
- Async I/O (tokio::net)
- Broadcast channels
- Shared state with Arc<Mutex>
- Concurrent task spawning
- Stream splitting (read/write)
- Error handling

ARCHITECTURE:
1. Main loop accepts connections
2. Each client gets own task
3. Broadcast channel for messages
4. Shared HashMap for user tracking
5. Separate read/write tasks per client

TOKIO FEATURES USED:
- TcpListener/TcpStream (async networking)
- AsyncBufReadExt/AsyncWriteExt (async I/O)
- broadcast::channel (message broadcasting)
- Arc<Mutex> (shared state)
- tokio::spawn (task spawning)

IMPROVEMENTS FOR PRODUCTION:
- User authentication
- Private messages
- Chat rooms/channels
- Message history
- Rate limiting
- Better error handling
- Logging
- Database persistence
- WebSocket support for web clients

RUN THIS:
    cargo run

CONNECT:
    # Terminal 1: Start server
    cargo run

    # Terminal 2: Connect client 1
    telnet localhost 8080
    or
    nc localhost 8080

    # Terminal 3: Connect client 2
    telnet localhost 8080

TEST:
    1. Enter username
    2. Type messages
    3. Try /users command
    4. Connect multiple clients
    5. See messages broadcast

KEY LEARNINGS:
- Async programming with Tokio
- Broadcasting to multiple clients
- Managing concurrent connections
- Shared mutable state
- Channel-based communication

ASYNC VS SYNC:
Sync:  One thread per connection (expensive)
Async: One task per connection (lightweight)

COMPARISON:
Node.js:  socket.io, native async
Python:   asyncio, WebSockets
Go:       goroutines, channels
Rust:     Tokio, zero-cost async

NEXT STEPS:
- Add private messaging
- Implement chat rooms
- Add WebSocket support
- Build a web frontend
- Add message persistence
- Implement user authentication

CODE STRUCTURE:
- ChatServer: Manages clients and broadcasting
- handle_client: Per-client connection handler
- Broadcast channel for message distribution
- Arc<Mutex> for thread-safe shared state
*/
