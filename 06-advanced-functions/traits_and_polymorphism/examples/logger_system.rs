/// Logger System - Traits and Polymorphism Example
///
/// Demonstrates:
/// - Defining traits with methods
/// - Implementing traits for different types
/// - Static dispatch with generic trait bounds
/// - Dynamic dispatch with trait objects
///
/// Run with: cargo run --example logger_system

use std::fs::OpenOptions;
use std::io::Write;

/// Trait that defines logging behavior
trait Logger {
    fn log(&self, message: &str);

    fn error(&self, message: &str) {
        self.log(&format!("[ERROR] {}", message));
    }

    fn warn(&self, message: &str) {
        self.log(&format!("[WARN] {}", message));
    }

    fn info(&self, message: &str) {
        self.log(&format!("[INFO] {}", message));
    }
}

/// Console logger - prints to stdout
struct ConsoleLogger {
    prefix: String,
}

impl ConsoleLogger {
    fn new(prefix: &str) -> Self {
        ConsoleLogger {
            prefix: prefix.to_string(),
        }
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, message: &str) {
        println!("[{}] {}", self.prefix, message);
    }
}

/// File logger - writes to file
struct FileLogger {
    path: String,
    prefix: String,
}

impl FileLogger {
    fn new(path: &str, prefix: &str) -> Self {
        FileLogger {
            path: path.to_string(),
            prefix: prefix.to_string(),
        }
    }
}

impl Logger for FileLogger {
    fn log(&self, message: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
        {
            let _ = writeln!(file, "[{}] {}", self.prefix, message);
        }
    }
}

/// Dual logger - logs to both console and file
struct DualLogger {
    console: ConsoleLogger,
    file: FileLogger,
}

impl DualLogger {
    fn new(file_path: &str, prefix: &str) -> Self {
        DualLogger {
            console: ConsoleLogger::new(prefix),
            file: FileLogger::new(file_path, prefix),
        }
    }
}

impl Logger for DualLogger {
    fn log(&self, message: &str) {
        self.console.log(message);
        self.file.log(message);
    }
}

/// Demonst generic function with trait bound (static dispatch)
fn run_application<L: Logger>(logger: &L, app_name: &str) {
    logger.info(&format!("Starting application: {}", app_name));

    logger.info("Loading configuration...");
    logger.info("Initializing modules...");

    logger.warn("Demo warning message");

    logger.info("Application ready");
    logger.error("Demo error message");

    logger.info(&format!("Shutting down: {}", app_name));
}

/// Function using trait objects (dynamic dispatch)
fn run_with_dynamic_logger(logger: &dyn Logger, app_name: &str) {
    logger.info(&format!("Running with dynamic dispatch: {}", app_name));
    logger.info("This uses runtime polymorphism");
    logger.error("Dynamic dispatch error example");
}

fn demo_static_dispatch() {
    println!("\n╔════════════════════════════════╗");
    println!("║ STATIC DISPATCH (Generic)      ║");
    println!("╚════════════════════════════════╝");

    println!("\n--- With Console Logger ---");
    let console_logger = ConsoleLogger::new("APP");
    run_application(&console_logger, "StaticApp");

    println!("\n--- With File Logger ---");
    let file_logger = FileLogger::new("/tmp/rust_log.txt", "FAPP");
    run_application(&file_logger, "FileApp");
}

fn demo_dynamic_dispatch() {
    println!("\n╔════════════════════════════════╗");
    println!("║ DYNAMIC DISPATCH (Trait Obj)   ║");
    println!("╚════════════════════════════════╝");

    let console: Box<dyn Logger> = Box::new(ConsoleLogger::new("DYN"));
    let file: Box<dyn Logger> = Box::new(FileLogger::new("/tmp/dyn_log.txt", "DYNF"));

    println!("\n--- Collection of Different Loggers ---");
    let loggers: Vec<Box<dyn Logger>> = vec![console, file];

    for logger in &loggers {
        logger.info("Logging from collection");
        logger.error("Error in collection");
    }
}

fn demo_dual_logger() {
    println!("\n╔════════════════════════════════╗");
    println!("║ DUAL LOGGER EXAMPLE            ║");
    println!("╚════════════════════════════════╝");

    let dual = DualLogger::new("/tmp/dual_log.txt", "DUAL");
    run_application(&dual, "DualApp");
}

fn print_separator() {
    println!("\n{}", "─".repeat(40));
}

fn main() {
    println!("╔════════════════════════════════╗");
    println!("║ TRAITS & POLYMORPHISM DEMO     ║");
    println!("╚════════════════════════════════╝");

    println!("\nThis demonstrates both static and dynamic dispatch");
    println!("using a Logger trait with multiple implementations.");

    demo_static_dispatch();
    print_separator();

    demo_dynamic_dispatch();
    print_separator();

    demo_dual_logger();

    print_separator();

    println!("\n╔════════════════════════════════╗");
    println!("║ DISPATCH COMPARISON            ║");
    println!("╠════════════════════════════════╣");
    println!("║ Static (Generic):              ║");
    println!("║ • Compile-time specialization  ║");
    println!("║ • Zero runtime overhead        ║");
    println!("║ • Larger binary size           ║");
    println!("║                                ║");
    println!("║ Dynamic (Trait Objects):       ║");
    println!("║ • Runtime type checking        ║");
    println!("║ • Small binary size            ║");
    println!("║ • Virtual method table lookup  ║");
    println!("╚════════════════════════════════╝");

    println!("\nLog files created at /tmp/rust_log.txt, /tmp/dyn_log.txt, /tmp/dual_log.txt");
}
