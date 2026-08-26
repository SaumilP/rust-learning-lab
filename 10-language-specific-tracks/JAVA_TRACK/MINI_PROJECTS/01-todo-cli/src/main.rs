use clap::{Parser, Subcommand};
use colored::*;
use std::process;
use todo_cli::{TodoApp, TodoFilter};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple todo CLI application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add { title: String },

    /// List all todos
    List {
        /// Filter by status (all, pending, completed)
        #[arg(short, long, default_value = "all")]
        filter: String,
    },

    /// Mark a todo as complete
    Complete { index: usize },

    /// Delete a todo
    Delete { index: usize },

    /// Clear all completed todos
    Clear,
}

fn main() {
    let cli = Cli::parse();

    // Load existing todos or create new app
    let mut app = TodoApp::load().unwrap_or_else(|_| {
        println!("{}", "Creating new todo list...".yellow());
        TodoApp::new()
    });

    match cli.command {
        Commands::Add { title } => {
            app.add_todo(title);
            if let Err(e) = app.save() {
                eprintln!("{} Failed to save: {}", "Error:".red(), e);
                process::exit(1);
            }
            println!("{} Todo added successfully", "✓".green());
        }

        Commands::List { filter } => {
            let filter = match filter.as_str() {
                "pending" => TodoFilter::Pending,
                "completed" => TodoFilter::Completed,
                _ => TodoFilter::All,
            };

            if app.is_empty() {
                println!("{}", "No todos yet! Add one with 'todo add \"Buy milk\"'".yellow());
                return;
            }

            println!("\n{}", "Your Todos:".bold().underline());
            for (index, todo) in app.list_todos(filter).iter().enumerate() {
                let status = if todo.completed {
                    "[✓]".green()
                } else {
                    "[ ]".white()
                };

                let title = if todo.completed {
                    todo.title.strikethrough()
                } else {
                    todo.title.normal()
                };

                println!("{} {} {}", index, status, title);
            }
            println!();
        }

        Commands::Complete { index } => {
            match app.complete_todo(index) {
                Ok(_) => {
                    if let Err(e) = app.save() {
                        eprintln!("{} Failed to save: {}", "Error:".red(), e);
                        process::exit(1);
                    }
                    println!("{} Todo marked as complete", "✓".green());
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Delete { index } => {
            match app.delete_todo(index) {
                Ok(_) => {
                    if let Err(e) = app.save() {
                        eprintln!("{} Failed to save: {}", "Error:".red(), e);
                        process::exit(1);
                    }
                    println!("{} Todo deleted", "✓".green());
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red(), e);
                    process::exit(1);
                }
            }
        }

        Commands::Clear => {
            let count = app.clear_completed();
            if let Err(e) = app.save() {
                eprintln!("{} Failed to save: {}", "Error:".red(), e);
                process::exit(1);
            }
            println!("{} Cleared {} completed todo(s)", "✓".green(), count);
        }
    }
}
