use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "A simple calculator CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add two numbers
    Add { a: f64, b: f64 },

    /// Susbstrct second number from the first
    Sub { a: f64, b: f64 },

    /// Multiply given two numbers
    Mul { a: f64, b: f64 },

    /// Divide first number by the second
    Div { a: f64, b: f64 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { a, b } => {
            println!("Result: {}", a + b);
        }

        Commands::Sub { a, b } => {
            println!("Result: {}", a - b);
        }

        Commands::Mul { a, b } => {
            println!("Result: {}", a * b);
        }

        Commands::Div { a, b } => {
            if b == 0.0 {
                eprintln!("Error: Division by zero is not allowed.");
                std::process::exit(1);
            }
            println!("Result: {}", a /b);
        }
    }
}