use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;

fn get_storage_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("todos.json");
    path
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    task: String,
    completed: bool,
}

#[derive(Parser)]
#[command(name = "todo", version, about = "A modern Rust Todo CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { task: String },

    /// List all the tasks
    List,

    /// Mark a task as done
    Done { id: usize },

    /// Remove a task
    Remove { id: usize },
}

fn main() {
    let cli = Cli::parse();
    let storage_path = get_storage_path();

    // Load existing todos or start fresh
    let mut todos = load_todos(&storage_path).unwrap_or_else(|_| Vec::new());

    match cli.command {
        Commands::Add { task } => {
            let id = todos.last().map(|t| t.id + 1).unwrap_or(1);
            todos.push(Todo {
                id,
                task,
                completed: false,
            });
            println!("🚀 Task added!");
        }
        Commands::List => {
            if todos.is_empty() {
                println!("Your list is empty. Take a nap! 😴");
            } else {
                for todo in &todos {
                    let status = if todo.completed { "✅" } else { "❌" };
                    println!("{} {}: {}", status, todo.id, todo.task);
                }
            }
        }
        Commands::Done { id } => {
            if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
                todo.completed = true;
                println!("🎉 Task {} marked as complete.", id);
            } else {
                println!("❗ Task with ID {} not found.", id);
            }
        }
        Commands::Remove { id } => {
            todos.retain(|todo| todo.id != id);
            println!("🗑️ Task {} removed!", id);
        }
    }

    save_todos(&storage_path, &todos).expect("Failed to save todos");
}

// --- File I/O Helpers ----

fn load_todos(path: &PathBuf) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let todos = serde_json::from_reader(reader)?;
    Ok(todos)
}

fn save_todos(path: &PathBuf, todos: &Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = serde_json::to_string_pretty(todos)?;
    fs::write(path, json_data)?;
    Ok(())
}
