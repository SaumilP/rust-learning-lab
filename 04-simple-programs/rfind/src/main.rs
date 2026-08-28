use clap::Parser;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

/// Simple program to search for files with a given name in a directory and its subdirectories.
#[derive(Parser, Debug)]
#[command(name = "rfind")]
#[command(about = "Search files recursively with pattern matching", long_about = None)]
struct Args {
    /// The directory to start the search from
    #[arg(short, long, default_value = ".")]
    dir: String,

    /// The filename pattern to search for
    #[arg(short, long)]
    pattern: String,

    /// Maximum depth of recursion
    #[arg(short, long)]
    max_depth: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let dir = Path::new(&args.dir);
    if !dir.exists() {
        eprintln!("❌ Directory '{}' does not exist!", dir.display());
        std::process::exit(1);
    }

    // Start walking the directory
    let walker = if let Some(depth) = args.max_depth {
        WalkDir::new(dir).max_depth(depth)
    } else {
        WalkDir::new(dir)
    };

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if is_file_matching(&entry, &args.pattern) {
            println!("{}", entry.path().display());
        }
    }
}

/// Check if the file matches the pattern
fn is_file_matching(entry: &DirEntry, pattern: &str) -> bool {
    entry.file_type().is_file() && entry.file_name().to_string_lossy().contains(pattern)
}
