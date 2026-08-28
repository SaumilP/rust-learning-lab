// Key-Value Store in Rust
// Demonstrates HashMap, file I/O, serialization, and REPL

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct KeyValueStore {
    data: HashMap<String, String>,
    file_path: String,
}

impl KeyValueStore {
    fn new(file_path: &str) -> Self {
        KeyValueStore {
            data: HashMap::new(),
            file_path: file_path.to_string(),
        }
    }

    fn load(file_path: &str) -> io::Result<Self> {
        match fs::read_to_string(file_path) {
            Ok(contents) => {
                let data: HashMap<String, String> = serde_json::from_str(&contents)?;
                Ok(KeyValueStore {
                    data,
                    file_path: file_path.to_string(),
                })
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(KeyValueStore::new(file_path)),
            Err(e) => Err(e),
        }
    }

    fn save(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    fn values(&self) -> Vec<&String> {
        self.data.values().collect()
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn clear(&mut self) {
        self.data.clear();
    }

    fn list(&self) -> Vec<(&String, &String)> {
        self.data.iter().collect()
    }
}

fn main() {
    println!("=== Key-Value Store ===\n");

    let file_path = "kvstore.json";
    let mut store = KeyValueStore::load(file_path).expect("Failed to load store");

    println!("Loaded store from {}", file_path);
    println!("Current entries: {}\n", store.len());

    println!("Commands:");
    println!("  set <key> <value>  - Set a key-value pair");
    println!("  get <key>          - Get value for key");
    println!("  delete <key>       - Delete a key");
    println!("  exists <key>       - Check if key exists");
    println!("  keys               - List all keys");
    println!("  values             - List all values");
    println!("  list               - List all key-value pairs");
    println!("  count              - Show number of entries");
    println!("  clear              - Clear all entries");
    println!("  save               - Save to disk");
    println!("  quit               - Exit\n");

    // REPL loop
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.splitn(3, ' ').collect();
        let command = parts[0];

        match command {
            "set" => {
                if parts.len() < 3 {
                    println!("Usage: set <key> <value>");
                    continue;
                }
                let key = parts[1].to_string();
                let value = parts[2].to_string();
                store.set(key.clone(), value.clone());
                println!("Set: {} = {}", key, value);
            }

            "get" => {
                if parts.len() < 2 {
                    println!("Usage: get <key>");
                    continue;
                }
                let key = parts[1];
                match store.get(key) {
                    Some(value) => println!("{}", value),
                    None => println!("Key '{}' not found", key),
                }
            }

            "delete" | "del" => {
                if parts.len() < 2 {
                    println!("Usage: delete <key>");
                    continue;
                }
                let key = parts[1];
                if store.delete(key) {
                    println!("Deleted '{}'", key);
                } else {
                    println!("Key '{}' not found", key);
                }
            }

            "exists" => {
                if parts.len() < 2 {
                    println!("Usage: exists <key>");
                    continue;
                }
                let key = parts[1];
                if store.exists(key) {
                    println!("'{}' exists", key);
                } else {
                    println!("'{}' does not exist", key);
                }
            }

            "keys" => {
                let keys = store.keys();
                if keys.is_empty() {
                    println!("No keys");
                } else {
                    println!("Keys ({}):", keys.len());
                    for key in keys {
                        println!("  - {}", key);
                    }
                }
            }

            "values" => {
                let values = store.values();
                if values.is_empty() {
                    println!("No values");
                } else {
                    println!("Values ({}):", values.len());
                    for value in values {
                        println!("  - {}", value);
                    }
                }
            }

            "list" | "ls" => {
                let entries = store.list();
                if entries.is_empty() {
                    println!("No entries");
                } else {
                    println!("Entries ({}):", entries.len());
                    for (key, value) in entries {
                        println!("  {} = {}", key, value);
                    }
                }
            }

            "count" | "len" => {
                println!("Total entries: {}", store.len());
            }

            "clear" => {
                print!("Are you sure? (yes/no): ");
                io::stdout().flush().unwrap();
                let mut confirm = String::new();
                io::stdin().read_line(&mut confirm).unwrap();
                if confirm.trim() == "yes" {
                    store.clear();
                    println!("Cleared all entries");
                } else {
                    println!("Cancelled");
                }
            }

            "save" => match store.save() {
                Ok(_) => println!("Saved to {}", file_path),
                Err(e) => println!("Failed to save: {}", e),
            },

            "quit" | "exit" => {
                print!("Save before exiting? (yes/no): ");
                io::stdout().flush().unwrap();
                let mut save_choice = String::new();
                io::stdin().read_line(&mut save_choice).unwrap();
                if save_choice.trim() == "yes" {
                    if let Err(e) = store.save() {
                        println!("Failed to save: {}", e);
                    } else {
                        println!("Saved to {}", file_path);
                    }
                }
                println!("Goodbye!");
                break;
            }

            "help" => {
                println!("Available commands:");
                println!("  set, get, delete, exists, keys, values");
                println!("  list, count, clear, save, quit, help");
            }

            "" => {}

            _ => {
                println!("Unknown command: '{}'. Type 'help' for commands.", command);
            }
        }
    }
}

/*
Key-Value Store Summary:
========================

FEATURES:
- In-memory HashMap storage
- Persistent JSON storage
- REPL interface
- Full CRUD operations
- List all keys/values
- Atomic save operation
- Interactive CLI

CONCEPTS DEMONSTRATED:
- HashMap usage
- File I/O (read/write)
- JSON serialization (serde)
- REPL implementation
- Error handling
- User input handling

OPERATIONS SUPPORTED:
- set    - Create/update entry
- get    - Retrieve value
- delete - Remove entry
- exists - Check existence
- keys   - List all keys
- values - List all values
- list   - Show all entries
- count  - Number of entries
- clear  - Remove all
- save   - Persist to disk

DATA PERSISTENCE:
- JSON format (human-readable)
- Automatic load on startup
- Manual save command
- Optional save on exit

IMPROVEMENTS FOR PRODUCTION:
- Transactions
- TTL (time-to-live)
- Namespaces/databases
- Bulk operations
- Search/filtering
- Indexing
- Compression
- Binary format (bincode)
- Network protocol
- Client-server architecture
- Replication
- Sharding

RUN THIS:
    cargo run

TRY:
    > set name "Rust"
    > set version "1.75"
    > get name
    > list
    > keys
    > save
    > quit

DATA FILE:
    kvstore.json (created automatically)

KEY LEARNINGS:
- HashMap as in-memory store
- JSON serialization/deserialization
- File I/O patterns
- REPL design
- Command parsing

COMPARISON TO REDIS:
Redis:  In-memory, network protocol, advanced data structures
This:   Simple, local, educational

COMPARISON:
Redis:     Network server, advanced features
memcached: Network cache
SQLite:    SQL database
This:      Simple KV store, JSON persistence

USE CASES:
- Configuration storage
- Cache layer
- Session storage
- Simple database
- Application state
- Feature flags

NEXT STEPS:
- Add network protocol
- Implement TTL
- Add transactions
- Support multiple data types
- Build a client library
- Add replication

CODE STRUCTURE:
- KeyValueStore: Core data structure
- REPL: Command-line interface
- Serialization: JSON persistence
- Commands: Full CRUD + utilities
*/
